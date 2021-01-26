use std::fmt::Write;

const KEYWORDS: [&str; 8] = [
    "ifeq", "ifneq", "ifdef", "ifndef", "else", "endif", "define", "endef",
];

pub fn makefile() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        // kakconf:kak
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*(/?[mM]akefile|\.mk|\.make) %[
    set-option buffer filetype makefile
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=makefile %[
    require-module makefile

    set-option window static_words %opt[makefile_static_words]

    hook window InsertChar \n -group makefile-indent makefile-indent-on-new-line
    hook -once -always window WinSetOption filetype=.* %[ remove-hooks window makefile-.+ ]
]

hook -group makefile-highlight global WinSetOption filetype=makefile %[
    add-highlighter window/makefile ref makefile
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/makefile ]
]

provide-module makefile %[

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/makefile regions

add-highlighter shared/makefile/content default-region group
add-highlighter shared/makefile/comment region (?<!\\)(?:\\\\)*(?:^|\h)\K# '$' fill comment
add-highlighter shared/makefile/evaluate-commands region -recurse \( (?<!\$)(?:\$\$)*\K\$\( \) fill value

add-highlighter shared/makefile/content/ regex ^\S.*?(::|:|!)\s 0:variable
add-highlighter shared/makefile/content/ regex [+?:]= 0:operator

# Add the language's grammar to the static completion list.
declare-option str-list makefile_static_words {keywords_space}

# Highlight keywords.
add-highlighter shared/makefile/content/ regex \b({keywords})\b 0:keyword

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden makefile-indent-on-new-line %[
    evaluate-commands -draft -itersel %[
        # Preserve previous line indent.
        try %[ execute-keys -draft <semicolon>K<a-&> ]
        # If the line above is a target indent with a tab.
        try %[ execute-keys -draft Z k<a-x> <a-k>^\S.*?(::|:|!)\s<ret> z i<tab> ]
        # Cleanup trailing white space son previous line.
        try %[ execute-keys -draft k<a-x> s \h+$ <ret>d ]
        # Indent after some keywords.
        try %[ execute-keys -draft Z k<a-x> <a-k> ^\h*(ifeq|ifneq|ifdef|ifndef|else|define)\b<ret> z <a-gt> ]
    ]
]

]
"#,
        keywords_space = KEYWORDS.join(" "),
        keywords = KEYWORDS.join("|")
    )?;

    Ok(buf)
}
