pub fn wrap(valid_prefixes: &[&str], text: &str, width: u32) -> Option<String> {
    let mut prefix = None;
    for pre in valid_prefixes {
        if text.starts_with(pre) {
            prefix = Some(pre);
        }
    }
    if prefix.is_none() {
        return None;
    }
    let prefix = if valid_prefixes.is_empty() {
        None
    } else {
        Some(*prefix.unwrap())
    };
    let prefixlen = prefix.map(|x| x.len()).unwrap_or_default();

    let mut text_no_prefixes = String::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(prefix) = prefix {
            if !line.starts_with(prefix) {
                return None;
            }
        }
        if i > 0 {
            text_no_prefixes.push(' ');
        }
        text_no_prefixes.push_str(&line[prefixlen..]);
    }

    let mut output = String::new();
    let mut outputlen = 0;
    if let Some(prefix) = prefix {
        output.push_str(prefix);
        outputlen += prefixlen;
    }
    for (i, word) in text_no_prefixes.split(' ').enumerate() {
        let wordlen = word.len();
        let mut first_word_of_line = false;
        if wordlen + outputlen + 1 >= width as usize {
            output.push('\n');
            outputlen = 0;
            if let Some(prefix) = prefix {
                output.push_str(prefix);
                outputlen += prefixlen;
            }
            first_word_of_line = true;
        }
        if prefix.is_some() && !first_word_of_line && i > 0 {
            output.push(' ');
            outputlen += 1;
        }
        output.push_str(word);
        outputlen += wordlen;
    }
    Some(output)
}

#[test]
fn test_wrap() {
    assert_eq!(
        wrap(&["// ", "//! ", "/// "], "// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris hendrerit, justo in consequat molestie, diam nisi commodo nulla, in vehicula sapien nisl ut nisl. Donec in pretium tellus. Morbi egestas porttitor lectus, eget posuere purus. Ut vehicula mauris nunc, vel malesuada purus ullamcorper at.", 100),
        Some("
// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris hendrerit, justo in consequat
// molestie, diam nisi commodo nulla, in vehicula sapien nisl ut nisl. Donec in pretium tellus.
// Morbi egestas porttitor lectus, eget posuere purus. Ut vehicula mauris nunc, vel malesuada purus
// ullamcorper at.".trim_start().into())
    );
    assert_eq!(
        wrap(
            &["# "],
            "#     add-highlighter window/ number-lines -hlcursor -separator ' ' # -relative",
            80
        ),
        Some(
            "#     add-highlighter window/ number-lines -hlcursor -separator ' ' # -relative"
                .into()
        ),
    );
    assert_eq!(
        wrap(
            &["# "],
            "#      add-highlighter window/ number-lines -hlcursor -separator ' ' # -relative",
            80
        ),
        Some(
            "#      add-highlighter window/ number-lines -hlcursor -separator ' ' #\n# -relative"
                .into()
        ),
    );
    assert_eq!(
        wrap(
            &["# "],
            "
# Ciao masiero io mi chiam oaaidops iopdasi opdiasop idopasipo diopasio pdiaops ipodiaposi
# podad iopsap dpoasiop iapdsiod pasipd ias das[ idpasi odiaspi
# dpoasi d"
                .trim_start(),
            80
        ),
        Some(
            "
# Ciao masiero io mi chiam oaaidops iopdasi opdiasop idopasipo diopasio pdiaops
# ipodiaposi podad iopsap dpoasiop iapdsiod pasipd ias das[ idpasi odiaspi
# dpoasi d"
                .trim_start()
                .into()
        ),
    );
}
