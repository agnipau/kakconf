use crate::editorconfig_sys::{
    editorconfig_get_error_msg, editorconfig_handle, editorconfig_handle_destroy,
    editorconfig_handle_get_err_file, editorconfig_handle_get_name_value,
    editorconfig_handle_get_name_value_count, editorconfig_handle_init,
    editorconfig_handle_set_conf_file_name, editorconfig_handle_set_version, editorconfig_parse,
};
use anyhow::{bail, Context};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::path::Path;

pub struct EditorConfig {
    handle: editorconfig_handle,
}

impl EditorConfig {
    pub fn new(conf_filename: Option<&str>, version: Option<&Version>) -> anyhow::Result<Self> {
        let handle = unsafe { editorconfig_handle_init() };
        if handle.is_null() {
            bail!("Unable to create EditorConfig handle");
        }

        if let Some(conf_filename) = conf_filename {
            let conf_filename = CString::new(conf_filename)?;
            unsafe { editorconfig_handle_set_conf_file_name(handle, conf_filename.as_ptr()) };
        }

        let default_version = Version::default();
        let version = version.unwrap_or(&default_version);
        unsafe {
            editorconfig_handle_set_version(handle, version.major, version.minor, version.patch)
        };

        Ok(Self { handle })
    }

    pub fn parse(&self, path: &Path) -> anyhow::Result<HashMap<String, String>> {
        if !path.is_absolute() {
            bail!("Path is not absolute");
        }
        let path = path.to_str().context("Failed to convert path to &str")?;
        let path = CString::new(path)?;

        let errnum = unsafe { editorconfig_parse(path.as_ptr(), self.handle) };
        if errnum != 0 {
            let mut anyhow_errormsg = String::new();

            let errormsg = unsafe { editorconfig_get_error_msg(errnum) };
            let errormsg = unsafe { CStr::from_ptr(errormsg) };
            let errormsg = errormsg.to_str()?;
            anyhow_errormsg.push_str(errormsg);
            if errnum > 0 {
                let errfile = unsafe { editorconfig_handle_get_err_file(self.handle) };
                let errfile = unsafe { CStr::from_ptr(errfile) };
                let errfile = errfile.to_str()?;
                anyhow_errormsg.push_str(&format!(":{} \"{}\"", errnum, errfile));
            }
            anyhow_errormsg.push('\n');

            bail!("{}", anyhow_errormsg);
        }

        let mut map = HashMap::new();
        let name_value_count = unsafe { editorconfig_handle_get_name_value_count(self.handle) };
        for i in 0..name_value_count {
            let mut name: MaybeUninit<*const c_char> = MaybeUninit::zeroed();
            let mut value: MaybeUninit<*const c_char> = MaybeUninit::zeroed();
            unsafe {
                editorconfig_handle_get_name_value(
                    self.handle,
                    i,
                    name.as_mut_ptr(),
                    value.as_mut_ptr(),
                )
            };
            let name = (unsafe { CStr::from_ptr(name.assume_init()) })
                .to_str()?
                .to_owned();
            let value = (unsafe { CStr::from_ptr(value.assume_init()) })
                .to_str()?
                .to_owned();
            map.insert(name, value);
        }
        Ok(map)
    }
}

impl Drop for EditorConfig {
    fn drop(&mut self) {
        unsafe { editorconfig_handle_destroy(self.handle) };
    }
}

pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    pub fn new(s: &str) -> anyhow::Result<Self> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .context("Major version number not found")?
            .parse()?;
        let minor = parts
            .next()
            .context("Minor version number not found")?
            .parse()?;
        let patch = parts
            .next()
            .context("Patch version number not found")?
            .parse()?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: -1,
            minor: -1,
            patch: -1,
        }
    }
}

#[test]
fn test_parse() {
    let editorconfig = EditorConfig::new(None, None).unwrap();
    let parsed = editorconfig.parse(Path::new("/home/agnipau/progetti/UnrealEngine/Templates/TP_FirstPerson/Source/TP_FirstPerson/TP_FirstPerson.cpp")).unwrap();
    dbg!(parsed);
}
