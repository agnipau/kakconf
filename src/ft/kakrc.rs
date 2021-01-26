use std::fmt::Write;

pub const KEYWORDS: [&str; 58] = [
    "add-highlighter",
    "alias",
    "arrange-buffers",
    "buffer",
    "buffer-next",
    "buffer-previous",
    "catch",
    "change-directory",
    "colorscheme",
    "debug",
    "declare-option",
    "declare-user-mode",
    "define-command",
    "delete-buffer",
    "delete-buffer!",
    "echo",
    "edit",
    "edit!",
    "enter-user-mode",
    "evaluate-commands",
    "execute-keys",
    "fail",
    "hook",
    "info",
    "kill",
    "kill!",
    "map",
    "menu",
    "nop",
    "on-key",
    "prompt",
    "provide-module",
    "quit",
    "quit!",
    "remove-highlighter",
    "remove-hooks",
    "rename-buffer",
    "rename-client",
    "rename-session",
    "require-module",
    "select",
    "set-face",
    "set-option",
    "set-register",
    "source",
    "trigger-user-hook",
    "try",
    "unalias",
    "unmap",
    "unset-face",
    "unset-option",
    "update-option",
    "write",
    "write!",
    "write-all",
    "write-all-quit",
    "write-quit",
    "write-quit!",
];

pub const ATTRIBUTES: [&str; 29] = [
    "global",
    "buffer",
    "window",
    "current",
    "normal",
    "insert",
    "menu",
    "prompt",
    "goto",
    "view",
    "user",
    "object",
    "number-lines",
    "show-matching",
    "show-whitespaces",
    "fill",
    "regex",
    "dynregex",
    "group",
    "flag-lines",
    "ranges",
    "line",
    "column",
    "wrap",
    "ref",
    "regions",
    "region",
    "default-region",
    "replace-ranges",
];

pub const TYPES: [&str; 10] = [
    "int",
    "bool",
    "str",
    "regex",
    "int-list",
    "str-list",
    "completions",
    "line-specs",
    "range-specs",
    "str-to-str-map",
];

pub const VALUES: [&str; 13] = [
    "default", "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white", "yes", "no",
    "false", "true",
];

pub fn kakrc() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate (.*/)?(kakrc|.*\.kak) %[
    set-option buffer filetype kak
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=kak %~
    require-module kak

    set-option window static_words %opt[kak_static_words]

    hook window InsertChar \n -group kak-insert kak-insert-on-new-line
    hook window InsertChar \n -group kak-indent kak-indent-on-new-line
    hook window InsertChar [>)}}\]] -group kak-indent kak-indent-on-closing-matching
    hook window InsertChar (?![[{{(<>)}}\]])[^\s\w] -group kak-indent kak-indent-on-closing-char
    # Cleanup trailing whitespaces on current line insert end.
    hook window ModeChange pop:insert:.* -group kak-trim-indent %[ try %[ execute-keys -draft <semicolon> <a-x> s ^\h+$ <ret> d ] ]
    set-option buffer extra_word_chars '_' '-'

    hook -once -always window WinSetOption filetype=.* %[ remove-hooks window kak-.+ ]
~

hook -group kak-highlight global WinSetOption filetype=kak %[
    add-highlighter window/kakrc ref kakrc
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/kakrc ]
]

provide-module kak %§

require-module sh

# Highlighters & Completion.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/kakrc regions
add-highlighter shared/kakrc/code default-region group
add-highlighter shared/kakrc/comment region (^|\h)\K# $ fill comment
add-highlighter shared/kakrc/double_string region -recurse %[(?<!")("")+(?!")] %[(^|\h)\K"] %["(?!")] group
add-highlighter shared/kakrc/single_string region -recurse %[(?<!')('')+(?!')] %[(^|\h)\K'] %['(?!')] group
add-highlighter shared/kakrc/shell1 region -recurse '\{{' '(^|\h)\K%?%sh\{{' '\}}' ref sh
add-highlighter shared/kakrc/shell2 region -recurse '\(' '(^|\h)\K%?%sh\(' '\)' ref sh
add-highlighter shared/kakrc/shell3 region -recurse '\[' '(^|\h)\K%?%sh\[' '\]' ref sh
add-highlighter shared/kakrc/shell4 region -recurse '<' '(^|\h)\K%?%sh<' '>' ref sh
add-highlighter shared/kakrc/shell5 region -recurse '\{{' '(^|\h)\K-shell-script-(completion|candidates)\h+%\{{' '\}}' ref sh
add-highlighter shared/kakrc/shell6 region -recurse '\(' '(^|\h)\K-shell-script-(completion|candidates)\h+%\(' '\)' ref sh
add-highlighter shared/kakrc/shell7 region -recurse '\[' '(^|\h)\K-shell-script-(completion|candidates)\h+%\[' '\]' ref sh
add-highlighter shared/kakrc/shell8 region -recurse '<' '(^|\h)\K-shell-script-(completion|candidates)\h+%<' '>' ref sh

# Add the language's grammar to the static completion list.
declare-option str-list kak_static_words {keywords_all}

# Highlight keywords (which are always surrounded by whitespace).
add-highlighter shared/kakrc/code/keywords regex (?:\s|\A)\K({keywords})(?:(?=\s)|\z) 0:keyword
add-highlighter shared/kakrc/code/attributes regex (?:\s|\A)\K({attributes})(?:(?=\s)|\z) 0:attribute
add-highlighter shared/kakrc/code/types regex (?:\s|\A)\K({types})(?:(?=\s)|\z) 0:type
add-highlighter shared/kakrc/code/values regex (?:\s|\A)\K({values})(?:(?=\s)|\z) 0:value

add-highlighter shared/kakrc/code/colors regex \b(rgb:[0-9a-fA-F]{{6}}|rgba:[0-9a-fA-F]{{8}})\b 0:value
add-highlighter shared/kakrc/code/numbers regex \b\d+\b 0:value

add-highlighter shared/kakrc/double_string/fill fill string
add-highlighter shared/kakrc/double_string/escape regex '""' 0:default+b
add-highlighter shared/kakrc/single_string/fill fill string
add-highlighter shared/kakrc/single_string/escape regex "''" 0:default+b

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden kak-insert-on-new-line %~
    evaluate-commands -draft -itersel %=
        # Copy '#' comment prefix and following white spaces.
        try %[ execute-keys -draft k <a-x> s ^\h*#\h* <ret> y jgh P ]
    =
~

define-command -hidden kak-indent-on-new-line %~
    evaluate-commands -draft -itersel %=
        # Preserve previous line indent.
        try %[ execute-keys -draft <semicolon> K <a-&> ]
        # Cleanup trailing whitespaces from previous line.
        try %[ execute-keys -draft k <a-x> s \h+$ <ret> d ]
        # Indent after line ending with %\w*[^\s\w].
        try %[ execute-keys -draft k <a-x> <a-k> \%\w*[^\s\w]$ <ret> j <a-gt> ]
        # Deindent closing brace when after cursor.
        try %_ execute-keys -draft -itersel <a-x> <a-k> ^\h*([>)}}\]]) <ret> gh / <c-r>1 <ret> m <a-S> 1<a-&> _
        # Deindent closing char(s).
        try %[ execute-keys -draft -itersel <a-x> <a-k> ^\h*([^\s\w]) <ret> gh / <c-r>1 <ret> <a-?> <c-r>1 <ret> <a-T>% <a-k> \w*<c-r>1$ <ret> <a-S> 1<a-&> ]
    =
~

define-command -hidden kak-indent-on-closing-matching %~
    # Align to opening matching brace when alone on a line.
    try %= execute-keys -draft -itersel <a-h><a-k>^\h*\Q %val[hook_param] \E$<ret> mGi s \A|.\z<ret> 1<a-&> =
~

define-command -hidden kak-indent-on-closing-char %[
    # Align to opening matching character when alone on a line.
    try %[ execute-keys -draft -itersel <a-h><a-k>^\h*\Q %val[hook_param] \E$<ret>gi<a-f> %val[hook_param] <a-T>%<a-k>\w*\Q %val[hook_param] \E$<ret> s \A|.\z<ret> gi 1<a-&> ]
]

§
"#,
        keywords_all = format!(
            "{} {} {} {}",
            KEYWORDS.join(" "),
            ATTRIBUTES.join(" "),
            TYPES.join(" "),
            VALUES.join(" "),
        ),
        keywords = KEYWORDS.join("|"),
        attributes = ATTRIBUTES.join("|"),
        types = TYPES.join("|"),
        values = VALUES.join("|")
    )?;

    Ok(buf)
}
