use anyhow::{anyhow, Context};
use std::io::Write;
use std::process::{Command, Stdio};

fn js_quote(s: &str) -> String {
    let mut s = s.replace('\'', r#"\'"#);
    s
}

pub fn json_quote(s: &str) -> String {
    let mut s = s.replace('"', r#"\""#);
    s
}

pub fn json_fx(json: &[u8], indent: usize, transform: &str) -> anyhow::Result<String> {
    let eval = format!(
        r#"
const fs = require("fs");
const json = JSON.parse(fs.readFileSync(0));
const result = eval('(self) => {}')();
console.log(JSON.stringify({}));
"#,
        js_quote(transform),
        format!("result, null, {}", indent),
    )
    .trim()
    .to_owned();

    let mut node = Command::new("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&["-e", &eval])
        .spawn()?;
    let node_stdin = node
        .stdin
        .as_mut()
        .context("Failed to write to node's stdin")?;
    node_stdin.write_all(json)?;
    drop(node_stdin);

    let node_output = node.wait_with_output()?;
    if node_output.status.success() {
        let node_stdout = String::from_utf8(node_output.stdout)?;
        Ok(node_stdout)
    } else {
        let node_stderr = String::from_utf8(node_output.stderr)?;
        Err(anyhow!("{}", node_stderr))
    }
}
