use std::fmt::Write;

pub fn systemd() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r##"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*/systemd/.+\.(automount|conf|link|mount|network|path|service|slice|socket|target|timer) %[
    set-option buffer filetype ini

    # NOTE: INI files define the commenting character to be `;`, which won't work in `systemd` files.
    hook -once buffer BufSetOption comment_line=.+ %[
        set-option buffer comment_line "#"
    ]
]
"##
    )?;

    Ok(buf)
}
