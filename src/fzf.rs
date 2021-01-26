use crate::{
    escape,
    utils::{kak_send_message, sh_quote},
};
use anyhow::Context;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn fzf_edit_inner_preview(
    kak_session: &str,
    kak_client: &str,
    fzf_file: &str,
) -> anyhow::Result<()> {
    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    edit {}
}}
",
            kak_client,
            escape::quote(fzf_file),
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_edit_inner(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
) -> anyhow::Result<()> {
    let fzf_output = Command::new("fzf")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-edit-inner-preview -s {} -c {} -f {{}}",
                kak_session, kak_client
            ),
            "--preview-window",
            ":0",
        ])
        .output()?
        .stdout;
    if fzf_output.is_empty() {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    buffer {}
    temp-delete-buffer
}}
",
                kak_client,
                escape::quote(kak_buffile),
            )
            .trim()
            .as_bytes(),
        )?;
    } else {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    set-option global temp_edit_last_buffer %{{}}
}}
",
                kak_client
            )
            .trim()
            .as_bytes(),
        )?;
    }
    Ok(())
}

pub fn fzf_edit(kak_session: &str, kak_client: &str, kak_buffile: &str) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!(
                "kakconf fzf-edit-inner -s {} -c {} -f {}",
                kak_session,
                kak_client,
                sh_quote(kak_buffile),
            ),
        ])
        .output()?;
    Ok(())
}

const FZF_CD_TMPFILE: &str = "/tmp/kak_fzf_cd";

pub fn fzf_cd_inner_preview(
    kak_session: &str,
    kak_client: &str,
    mut fzf_selected_list: Vec<&str>,
) -> anyhow::Result<()> {
    fzf_selected_list.insert(0, "-lha");
    let ls_output = Command::new("ls")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&fzf_selected_list)
        .output()?
        .stdout;
    fs::write(FZF_CD_TMPFILE, &ls_output)?;
    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    edit! -readonly {}
}}
",
            kak_client, FZF_CD_TMPFILE,
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_cd_inner(kak_session: &str, kak_client: &str) -> anyhow::Result<()> {
    let file_list = Command::new("fd")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&["--type", "d", "--hidden", "--follow", "--exclude", ".git"])
        .output()?
        .stdout;
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-cd-inner-preview -s {} -c {} -f {{+}}",
                kak_session, kak_client
            ),
            "--preview-window",
            ":0",
        ])
        .spawn()?;
    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Failed to write to fzf's stdin")?;
    fzf_stdin.write_all(&file_list)?;
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output()?.stdout;

    if !fzf_output.is_empty() {
        let args = String::from_utf8(fzf_output)?;
        let args = escape::quote(args.trim_end());
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    change-directory {}
}}
",
                kak_client, args
            )
            .trim()
            .as_bytes(),
        )?;
    }

    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    restore-selections
    delete-buffer {}
}}
",
            kak_client, FZF_CD_TMPFILE
        )
        .trim()
        .as_bytes(),
    )?;
    fs::remove_file(FZF_CD_TMPFILE)?;
    Ok(())
}

pub fn fzf_cd(kak_session: &str, kak_client: &str) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!("kakconf fzf-cd-inner -s {} -c {}", kak_session, kak_client,),
        ])
        .output()?;
    Ok(())
}

pub fn fzf_change_buffer_inner_preview(
    kak_session: &str,
    kak_client: &str,
    fzf_file: &str,
) -> anyhow::Result<()> {
    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    buffer {}
}}
",
            kak_client,
            escape::quote(fzf_file)
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_change_buffer_inner(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: &[&str],
) -> anyhow::Result<()> {
    let kak_buflist = kak_buflist.join("\n");

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-change-buffer-inner-preview -s {} -c {} -f {{}}",
                kak_session, kak_client,
            ),
            "--preview-window",
            ":0",
        ])
        .spawn()?;
    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Failed to write to fzf's stdin")?;
    fzf_stdin.write_all(kak_buflist.as_bytes())?;
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output()?.stdout;

    if fzf_output.is_empty() {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    buffer {}
}}
",
                kak_client,
                escape::quote(kak_buffile),
            )
            .trim()
            .as_bytes(),
        )?;
    }
    Ok(())
}

pub fn fzf_change_buffer(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: Vec<&str>,
) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!(
                "kakconf fzf-change-buffer-inner -s {} -c {} -f {} -l {}",
                kak_session,
                kak_client,
                sh_quote(kak_buffile),
                kak_buflist
                    .into_iter()
                    .map(|x| sh_quote(x))
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
        ])
        .output()?;
    Ok(())
}

pub fn fzf_delete_buffer_inner_preview(
    kak_session: &str,
    kak_client: &str,
    fzf_file: &str,
) -> anyhow::Result<()> {
    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    buffer {}
}}
",
            kak_client,
            escape::quote(fzf_file)
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_delete_buffer_inner(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: &[&str],
) -> anyhow::Result<()> {
    let kak_buflist = kak_buflist.join("\n");

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-delete-buffer-inner-preview -s {} -c {} -f {{}}",
                kak_session, kak_client,
            ),
            "--preview-window",
            ":0",
        ])
        .spawn()?;
    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Failed to write to fzf's stdin")?;
    fzf_stdin.write_all(kak_buflist.as_bytes())?;
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output()?.stdout;

    if fzf_output.is_empty() {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    buffer {}
}}
",
                kak_client,
                escape::quote(kak_buffile),
            )
            .trim()
            .as_bytes(),
        )?;
    } else {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    delete-buffer
}}
",
                kak_client,
            )
            .trim()
            .as_bytes(),
        )?;
    }
    Ok(())
}

pub fn fzf_delete_buffer(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: Vec<&str>,
) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!(
                "kakconf fzf-delete-buffer-inner -s {} -c {} -f {} -l {}",
                kak_session,
                kak_client,
                sh_quote(kak_buffile),
                kak_buflist
                    .into_iter()
                    .map(|x| sh_quote(x))
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
        ])
        .output()?;
    Ok(())
}

const FZF_LINES_TMPFILE: &str = "/tmp/kak_fzf_lines";

pub fn fzf_lines_inner_preview(
    kak_session: &str,
    kak_client: &str,
    indexes: &[&str],
) -> anyhow::Result<()> {
    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    put-cursors-zero {}
}}
",
            kak_client,
            indexes.join(" "),
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_lines_inner(kak_session: &str, kak_client: &str) -> anyhow::Result<()> {
    let tmpfile_contents = fs::read(FZF_LINES_TMPFILE)?;
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-lines-inner-preview -s {} -c {} -i {{+n}}",
                kak_session, kak_client
            ),
            "--preview-window",
            ":0",
        ])
        .spawn()?;
    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Failed to write to fzf's stdin")?;
    fzf_stdin.write_all(&tmpfile_contents)?;
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output()?.stdout;

    if fzf_output.is_empty() {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    restore-selections
}}
",
                kak_client,
            )
            .trim()
            .as_bytes(),
        )?;
    }
    fs::remove_file(FZF_LINES_TMPFILE)?;
    Ok(())
}

pub fn fzf_lines(kak_session: &str, kak_client: &str) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!(
                "kakconf fzf-lines-inner -s {} -c {}",
                kak_session, kak_client,
            ),
        ])
        .output()?;
    Ok(())
}

pub fn fzf_rg_inner_preview(
    kak_session: &str,
    kak_client: &str,
    fzf_file: &str,
) -> anyhow::Result<()> {
    let re = Regex::new(r#"(.+):(\d+)"#).unwrap();
    let caps = re
        .captures(fzf_file)
        .context("Failed to capture file path and line number from line")?;
    let filepath = caps
        .get(1)
        .map(|x| x.as_str())
        .context("Failed to capture file path")?;
    let line_number = caps
        .get(2)
        .map(|x| x.as_str())
        .context("Failed to capture line number")?;

    kak_send_message(
        kak_session,
        format!(
            "
evaluate-commands -client {} %{{
    temp-edit {} {}
}}
",
            kak_client, filepath, line_number,
        )
        .trim()
        .as_bytes(),
    )?;
    Ok(())
}

pub fn fzf_rg_inner(kak_session: &str, kak_client: &str, rg_query: &str) -> anyhow::Result<()> {
    let rg_output = Command::new("rg")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&[rg_query, "--line-number"])
        .output()?
        .stdout;

    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&[
            "--preview",
            &format!(
                "kakconf fzf-rg-inner-preview -s {} -c {} -f {{}}",
                kak_session, kak_client
            ),
            "--preview-window",
            ":0",
        ])
        .spawn()?;
    let fzf_stdin = fzf
        .stdin
        .as_mut()
        .context("Failed to write to fzf's stdin")?;
    fzf_stdin.write_all(&rg_output)?;
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output()?.stdout;

    if fzf_output.is_empty() {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    restore-selections
    temp-delete-buffer
}}
",
                kak_client,
            )
            .trim()
            .as_bytes(),
        )?;
    } else {
        kak_send_message(
            kak_session,
            format!(
                "
evaluate-commands -client {} %{{
    set-option global temp_edit_last_buffer %[]
}}
",
                kak_client,
            )
            .trim()
            .as_bytes(),
        )?;
    }
    Ok(())
}

pub fn fzf_rg(kak_session: &str, kak_client: &str, rg_query: &str) -> anyhow::Result<()> {
    Command::new("tmux")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&[
            "split-window",
            "-f",
            "-l",
            "30%",
            &format!(
                "kakconf fzf-rg-inner -s {} -c {} -q {}",
                kak_session,
                kak_client,
                sh_quote(rg_query),
            ),
        ])
        .output()?;
    Ok(())
}
