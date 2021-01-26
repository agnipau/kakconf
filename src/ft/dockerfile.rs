// See https://docs.docker.com/reference/builder.

use std::fmt::Write;

const KEYWORDS: [&str; 17] = [
    "ADD",
    "ARG",
    "CMD",
    "COPY",
    "ENTRYPOINT",
    "ENV",
    "EXPOSE",
    "FROM",
    "HEALTHCHECK",
    "LABEL",
    "MAINTAINER",
    "RUN",
    "SHELL",
    "STOPSIGNAL",
    "USER",
    "VOLUME",
    "WORKDIR",
];

pub fn dockerfile() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        // kakconf:kak
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*/?Dockerfile(\.\w+)?$ %[
    set-option buffer filetype dockerfile
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=dockerfile %[
    require-module dockerfile
    set-option window static_words %opt[dockerfile_static_words]
]

hook -group dockerfile-highlight global WinSetOption filetype=dockerfile %[
    add-highlighter window/dockerfile ref dockerfile
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/dockerfile ]
]

provide-module dockerfile %[

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/dockerfile regions
add-highlighter shared/dockerfile/code default-region group
add-highlighter shared/dockerfile/double_string region '"' '(?<!\\)(\\\\)*"' fill string
add-highlighter shared/dockerfile/single_string region "'" "'" fill string
add-highlighter shared/dockerfile/comment region '#' $ fill comment

# Add the language's grammar to the static completion list.
declare-option str-list dockerfile_static_words ONBUILD {keywords_spaces}

# Highlight keywords.
add-highlighter shared/dockerfile/code/ regex '^(?i)(ONBUILD\h+)?({keywords})\b' 2:keyword
add-highlighter shared/dockerfile/code/ regex '^(?i)(ONBUILD)\h+' 1:keyword

add-highlighter shared/dockerfile/code/ regex (?<!\\)(?:\\\\)*\K\$\{{[\w_]+\}} 0:value
add-highlighter shared/dockerfile/code/ regex (?<!\\)(?:\\\\)*\K\$[\w_]+ 0:value

]
"#,
        keywords_spaces = KEYWORDS.join(" "),
        keywords = KEYWORDS.join("|")
    )?;

    Ok(buf)
}
