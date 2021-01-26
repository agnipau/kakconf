use std::fmt::Write;

pub fn arch_linux() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
hook global BufCreate (.*/)?PKGBUILD %[
    set-option buffer filetype sh
]
"#
    )?;

    Ok(buf)
}
