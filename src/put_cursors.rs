/// `lines` indexes are 1-based like in Kakoune.
/// `lines` is assumed to be sorted in ascending order.
pub fn put_cursors(total_lines: usize, lines: &[usize]) -> String {
    if lines.len() == 1 {
        format!("execute-keys '{}g'", lines[0])
    } else {
        let mut line_no = 1;
        let mut lines_idx = 0;
        let mut keys = "%<a-s>gh)".to_owned();
        while line_no <= total_lines {
            if lines_idx >= lines.len() {
                keys.push_str("<a-space>");
            } else if lines[lines_idx] == line_no {
                keys.push(')');
                lines_idx += 1;
            } else {
                keys.push_str("<a-space>");
            }
            line_no += 1;
        }
        if lines.last() < Some(&total_lines) {
            keys.push(')');
        }
        format!("execute-keys '{}'", keys)
    }
}
