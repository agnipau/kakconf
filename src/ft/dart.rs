use std::fmt::Write;

const KEYWORDS: [&str; 49] = [
    "abstract",
    "do",
    "import",
    "super",
    "as",
    "in",
    "switch",
    "assert",
    "else",
    "interface",
    "async",
    "enum",
    "is",
    "this",
    "export",
    "library",
    "throw",
    "await",
    "external",
    "mixin",
    "break",
    "extends",
    "new",
    "try",
    "case",
    "factory",
    "typedef",
    "catch",
    "operator",
    "class",
    "final",
    "part",
    "const",
    "finally",
    "rethrow",
    "while",
    "continue",
    "for",
    "return",
    "with",
    "covariant",
    "get",
    "set",
    "yield",
    "default",
    "if",
    "static",
    "deferred",
    "implements",
];

const GENERATOR_KEYWORDS: [&str; 3] = [r#"async\*"#, r#"sync\*"#, r#"yield\*"#];

const TYPES: [&str; 7] = ["void", "bool", "num", "int", "double", "dynamic", "var"];

const VALUES: [&str; 3] = ["false", "true", "null"];

pub fn dart() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*\.dart %[
    set-option buffer filetype dart
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=dart %[
    require-module dart

    set-option window static_words %opt[dart_static_words]

    # Cleanup trailing whitespaces when exiting insert mode.
    hook window ModeChange pop:insert:.* -group dart-trim-indent %[ try %[ execute-keys -draft <a-x>s^\h+$<ret>d ] ]
    hook window InsertChar \n -group dart-indent dart-indent-on-new-line
    hook window InsertChar \{{ -group dart-indent dart-indent-on-opening-curly-brace
    hook window InsertChar \}} -group dart-indent dart-indent-on-closing-curly-brace

    hook -once -always window WinSetOption filetype=.* %[ remove-hooks window dart-.+ ]
]

hook -group dart-highlight global WinSetOption filetype=dart %[
    add-highlighter window/dart ref dart
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/dart ]
]

provide-module dart %§

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/dart regions
add-highlighter shared/dart/code default-region group
add-highlighter shared/dart/back_string region '`' '`' fill string
add-highlighter shared/dart/double_string region '"' (?<!\\)(\\\\)*" fill string
add-highlighter shared/dart/single_string region "'" (?<!\\)(\\\\)*' fill string
add-highlighter shared/dart/comment region /\* \*/ fill comment
add-highlighter shared/dart/comment_line region '//' $ fill comment

add-highlighter shared/dart/code/ regex %[-?([0-9]*\.(?!0[xX]))?\b([0-9]+|0[xX][0-9a-fA-F]+)\.?([eE][+-]?[0-9]+)?i?\b] 0:value

# Add the language's grammar to the static completion list.
declare-option str-list dart_static_words {keywords_space}

# Highlight keywords.
add-highlighter shared/dart/code/ regex \b({keywords})\b 0:keyword
add-highlighter shared/dart/code/ regex \b({generator_keywords}) 0:keyword
add-highlighter shared/dart/code/ regex \b({types})\b 0:type
add-highlighter shared/dart/code/ regex \b({values})\b 0:value
add-highlighter shared/dart/code/ regex \b((_?[a-z][a-zA-Z0-9]*)(\(|\w+=>)) 2:function
add-highlighter shared/dart/code/ regex (@[a-zA-Z]+)\b 0:meta
add-highlighter shared/dart/code/ regex \b(_?[A-Z][a-zA-Z0-9]*)\b 0:module

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden dart-indent-on-new-line %~
    evaluate-commands -draft -itersel %=
        # Preserve previous line indent.
        try %[ execute-keys -draft <semicolon>K<a-&> ]
        # Indent after lines ending with {{ or (.
        try %[ execute-keys -draft k<a-x> <a-k> [{{(]\h*$ <ret> j<a-gt> ]
        # Cleanup trailing white spaces on the previous line.
        try %[ execute-keys -draft k<a-x> s \h+$ <ret>d ]
        # Align to opening paren of previous line.
        try %[ execute-keys -draft [( <a-k> \A\([^\n]+\n[^\n]*\n?\z <ret> s \A\(\h*.|.\z <ret> '<a-;>' & ]
        # Copy // comments prefix.
        try %[ execute-keys -draft <semicolon><c-s>k<a-x> s ^\h*\K/{{2,}} <ret> y<c-o>P<esc> ]
        # Indent after a switch's case/default statements.
        try %[ execute-keys -draft k<a-x> <a-k> ^\h*(case|default).*:$ <ret> j<a-gt> ]
        # Indent after if|else|while|for.
        try %[ execute-keys -draft <semicolon><a-F>)MB <a-k> \A(if|else|while|for)\h*\(.*\)\h*\n\h*\n?\z <ret> s \A|.\z <ret> 1<a-&>1<a-space><a-gt> ]
        # Deindent closing brace when after cursor.
        try %[ execute-keys -draft <a-x> <a-k> ^\h*\}} <ret> gh / \}} <ret> m <a-S> 1<a-&> ]
    =
~

define-command -hidden dart-indent-on-opening-curly-brace %[
    # Align indent with opening paren when {{ is entered on a new line after the closing paren.
    try %[ execute-keys -draft -itersel h<a-F>)M <a-k> \A\(.*\)\h*\n\h*\{{\z <ret> s \A|.\z <ret> 1<a-&> ]
]

define-command -hidden dart-indent-on-closing-curly-brace %[
    # Align to opening curly brace when alone on a line.
    try %[ execute-keys -itersel -draft <a-h><a-k>^\h+\}}$<ret>hms\A|.\z<ret>1<a-&> ]
]

§
"#,
        keywords_space = format!(
            "{} {} {}",
            KEYWORDS.join(" "),
            TYPES.join(" "),
            VALUES.join(" ")
        ),
        keywords = KEYWORDS.join("|"),
        generator_keywords = GENERATOR_KEYWORDS.join("|"),
        types = TYPES.join("|"),
        values = VALUES.join("|")
    )?;

    Ok(buf)
}
