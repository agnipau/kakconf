use crate::escape;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn fzf_edit_inner_preview(kak_session: &str, kak_client: &str, fzf_file: &str) {
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
    );
}

pub fn fzf_edit_inner(kak_session: &str, kak_client: &str, kak_buffile: &str) {
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
        .output()
        .unwrap()
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
        );
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
        );
    }
}

fn sh_quote(s: &str) -> String {
    let mut s = s.replace('\'', r#"'\''"#);
    s.insert(0, '\'');
    s.push('\'');
    s
}

fn kak_send_message(kak_session: &str, message: &[u8]) {
    let mut kak = Command::new("kak")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&["-p", kak_session])
        .spawn()
        .unwrap();
    let kak_stdin = kak.stdin.as_mut().unwrap();
    kak_stdin.write_all(message).unwrap();
    drop(kak_stdin);
    kak.wait().unwrap();
}

pub fn fzf_edit(kak_session: &str, kak_client: &str, kak_buffile: &str) {
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
        .output()
        .unwrap();
}

const FZF_CD_TMPFILE: &str = "/tmp/kak_fzf_cd";

pub fn fzf_cd_inner_preview(kak_session: &str, kak_client: &str, mut fzf_selected_list: Vec<&str>) {
    fzf_selected_list.insert(0, "-lha");
    let ls_output = Command::new("ls")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&fzf_selected_list)
        .output()
        .unwrap()
        .stdout;
    fs::write(FZF_CD_TMPFILE, &ls_output).unwrap();
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
    );
}

pub fn fzf_cd_inner(kak_session: &str, kak_client: &str) {
    let file_list = Command::new("fd")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&["--type", "d", "--hidden", "--follow", "--exclude", ".git"])
        .output()
        .unwrap()
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
        .spawn()
        .unwrap();
    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    fzf_stdin.write_all(&file_list).unwrap();
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output().unwrap().stdout;

    if !fzf_output.is_empty() {
        let args = String::from_utf8(fzf_output).unwrap();
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
        );
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
    );
    fs::remove_file(FZF_CD_TMPFILE).unwrap();
}

pub fn fzf_cd(kak_session: &str, kak_client: &str) {
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
        .output()
        .unwrap();
}

pub fn fzf_change_buffer_inner_preview(kak_session: &str, kak_client: &str, fzf_file: &str) {
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
    );
}

pub fn fzf_change_buffer_inner(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: &[&str],
) {
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
        .spawn()
        .unwrap();
    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    fzf_stdin.write_all(kak_buflist.as_bytes()).unwrap();
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output().unwrap().stdout;

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
        );
    }
}

pub fn fzf_change_buffer(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: Vec<&str>,
) {
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
        .output()
        .unwrap();
}

pub fn fzf_delete_buffer_inner_preview(kak_session: &str, kak_client: &str, fzf_file: &str) {
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
    );
}

pub fn fzf_delete_buffer_inner(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: &[&str],
) {
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
        .spawn()
        .unwrap();
    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    fzf_stdin.write_all(kak_buflist.as_bytes()).unwrap();
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output().unwrap().stdout;

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
        );
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
        );
    }
}

pub fn fzf_delete_buffer(
    kak_session: &str,
    kak_client: &str,
    kak_buffile: &str,
    kak_buflist: Vec<&str>,
) {
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
        .output()
        .unwrap();
}

const FZF_LINES_TMPFILE: &str = "/tmp/kak_fzf_lines";

pub fn fzf_lines_inner_preview(kak_session: &str, kak_client: &str, indexes: &[&str]) {
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
    );
}

pub fn fzf_lines_inner(kak_session: &str, kak_client: &str) {
    // std::thread::sleep(std::time::Duration::from_secs(2));
    let tmpfile_contents = fs::read(FZF_LINES_TMPFILE).unwrap();
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
        .spawn()
        .unwrap();
    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    fzf_stdin.write_all(&tmpfile_contents).unwrap();
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output().unwrap().stdout;

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
        );
    }
    fs::remove_file(FZF_LINES_TMPFILE).unwrap();
}

pub fn fzf_lines(kak_session: &str, kak_client: &str) {
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
        .output()
        .unwrap();
}

pub fn fzf_rg_inner_preview(kak_session: &str, kak_client: &str, fzf_file: &str) {
    let re = Regex::new(r#"(.+):(\d+)"#).unwrap();
    let caps = re.captures(fzf_file).unwrap();
    let filepath = caps.get(1).map(|x| x.as_str()).unwrap();
    let line_number = caps.get(2).map(|x| x.as_str()).unwrap();

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
    );
}

pub fn fzf_rg_inner(kak_session: &str, kak_client: &str, rg_query: &str) {
    let rg_output = Command::new("rg")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args(&[rg_query, "--line-number"])
        .output()
        .unwrap()
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
        .spawn()
        .unwrap();
    let fzf_stdin = fzf.stdin.as_mut().unwrap();
    fzf_stdin.write_all(&rg_output).unwrap();
    drop(fzf_stdin);
    let fzf_output = fzf.wait_with_output().unwrap().stdout;

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
        );
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
        );
    }
}

pub fn fzf_rg(kak_session: &str, kak_client: &str, rg_query: &str) {
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
        .output()
        .unwrap();
}
