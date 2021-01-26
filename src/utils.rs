use anyhow::Context;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn kak_send_message(kak_session: &str, message: &[u8]) -> anyhow::Result<()> {
    let mut kak = Command::new("kak")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&["-p", kak_session])
        .spawn()?;
    let kak_stdin = kak
        .stdin
        .as_mut()
        .context("Failed to write to kak's stdin")?;
    kak_stdin.write_all(message)?;
    drop(kak_stdin);
    kak.wait()?;
    Ok(())
}

pub fn sh_quote(s: &str) -> String {
    let mut s = s.replace('\'', r#"'\''"#);
    s.insert(0, '\'');
    s.push('\'');
    s
}
