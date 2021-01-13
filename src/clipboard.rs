use crate::escape;
use std::{convert::TryFrom, process::Command};

pub enum Direction {
    Before,
    After,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "before" => Ok(Self::Before),
            "after" => Ok(Self::After),
            _ => Err(anyhow::anyhow!(
                "Invalid value. Accepted values are `before` or `after`"
            )),
        }
    }
}

pub fn get_clipboard(direction: &Direction, select: bool) -> anyhow::Result<String> {
    let contents = Command::new("xsel").arg("-ob").output()?.stdout;
    let contents = std::str::from_utf8(&contents)?;
    let contents = escape::escape_raw_insert(&contents);
    let contents = escape::double_string(&contents, "'");
    let ends_with_newline = contents.ends_with('\n');
    let contents = contents.trim_end();

    Ok(match (ends_with_newline, select, direction) {
        (true, false, Direction::After) => format!("exec -draft '<a-o>ji{}<esc>'", contents),
        (true, false, Direction::Before) => format!("exec -draft '<a-O>ki{}<esc>'", contents),
        (true, true, Direction::After) => format!("exec '<a-o>glla{}<esc>i<esc>La<esc>'", contents),
        (true, true, Direction::Before) => format!(
            "exec '<a-O>ka{}<ret><esc>i<esc>Li<backspace><esc>a<esc>'",
            contents
        ),
        (false, false, Direction::After) => format!("exec -draft 'li{}<esc>'", contents),
        (false, false, Direction::Before) => format!("exec -draft 'i{}<esc>'", contents),
        (false, true, Direction::After) => format!("exec 'a{}<esc>i<esc>La<esc>'", contents),
        (false, true, Direction::Before) => format!("exec 'ha{}<esc>i<esc>La<esc>'", contents),
    })
}
