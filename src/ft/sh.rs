use std::fmt::Write;

// Generated with `compgen -k` in bash.
pub const KEYWORDS: [&str; 17] = [
    "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while", "until", "do",
    "done", "in", "function", "time", "coproc",
];

// Generated with `compgen -b` in bash.
pub const BUILTINS: [&str; 58] = [
    "alias",
    "bg",
    "bind",
    "break",
    "builtin",
    "caller",
    "cd",
    "command",
    "compgen",
    "complete",
    "compopt",
    "continue",
    "declare",
    "dirs",
    "disown",
    "echo",
    "enable",
    "eval",
    "exec",
    "exit",
    "export",
    "false",
    "fc",
    "fg",
    "getopts",
    "hash",
    "help",
    "history",
    "jobs",
    "kill",
    "let",
    "local",
    "logout",
    "mapfile",
    "popd",
    "printf",
    "pushd",
    "pwd",
    "read",
    "readarray",
    "readonly",
    "return",
    "set",
    "shift",
    "shopt",
    "source",
    "suspend",
    "test",
    "times",
    "trap",
    "true",
    "type",
    "typeset",
    "ulimit",
    "umask",
    "unalias",
    "unset",
    "wait",
];

pub fn sh() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
hook global BufCreate .*\.((z|ba|c|k|mk)?sh(rc|_profile)?|profile) %[
    set-option buffer filetype sh
]

hook global WinSetOption filetype=sh %[
    require-module sh
    set-option window static_words %opt[sh_static_words]

    hook window ModeChange pop:insert:.* -group sh-trim-indent sh-trim-indent
    hook window InsertChar \n -group sh-insert sh-insert-on-new-line
    hook window InsertChar \n -group sh-indent sh-indent-on-new-line
    hook -once -always window WinSetOption filetype=.* %[
        remove-hooks window sh-.+
    ]
]

hook -group sh-highlight global WinSetOption filetype=sh %[
    add-highlighter window/sh ref sh
    hook -once -always window WinSetOption filetype=.* %[
        remove-highlighter window/sh
    ]
]

# Using non-ascii characters here so that we can use the '[' command.
provide-module sh %§

add-highlighter shared/sh regions
add-highlighter shared/sh/code default-region group
add-highlighter shared/sh/arithmetic region -recurse \(.*?\( (\$|(?<=for)\h*)\(\( \)\) group
add-highlighter shared/sh/double_string region  %[(?<!\\)(?:\\\\)*\K"] %[(?<!\\)(?:\\\\)*"] group
add-highlighter shared/sh/single_string region %[(?<!\\)(?:\\\\)*\K'] %['] fill string
add-highlighter shared/sh/expansion region -recurse (?<!\\)(?:\\\\)*\K\$\{{ (?<!\\)(?:\\\\)*\K\$\{{ \}}|\n fill value
add-highlighter shared/sh/comment region (?<!\\)(?:\\\\)*(?:^|\h)\K# '$' fill comment
add-highlighter shared/sh/heredoc region -match-capture '<<-?\h*''?(\w+)''?' '^\t*(\w+)$' fill string

add-highlighter shared/sh/arithmetic/expansion ref sh/double_string/expansion
add-highlighter shared/sh/double_string/fill fill string

# Add the language's grammar to the static completion list.
declare-option str-list sh_static_words {keywords_all}

# Highlight keywords.
add-highlighter shared/sh/code/ regex (?<!-)\b({keywords})\b(?!-) 0:keyword

# Highlight builtins.
add-highlighter shared/sh/code/builtin regex (?<!-)\b({builtins})\b(?!-) 0:builtin

add-highlighter shared/sh/code/operators regex [\[\]\(\)&|]{{1,2}} 0:operator
add-highlighter shared/sh/code/variable regex ((?<![-:])\b\w+)= 1:variable
add-highlighter shared/sh/code/alias regex \balias(\h+[-+]\w)*\h+([\w-.]+)= 2:variable
add-highlighter shared/sh/code/function regex ^\h*(\S+)\h*\(\) 1:function

add-highlighter shared/sh/code/unscoped_expansion regex (?<!\\)(?:\\\\)*\K\$(\w+|#|@|\?|\$|!|-|\*) 0:value
add-highlighter shared/sh/double_string/expansion regex (?<!\\)(?:\\\\)*\K\$(\w+|#|@|\?|\$|!|-|\*|\{{.+?\}}) 0:value

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden sh-trim-indent %[
    # Remove trailing white spaces.
    try %[ execute-keys -draft -itersel <a-x> s \h+$ <ret> d ]
]

# This is at best an approximation, since shell syntax is very complex.
# Also note that this targets plain sh syntax, not bash - bash adds a whole
# other level of complexity. If your bash code is fairly portable this will
# probably work.
#
# Of necessity, this is also fairly opinionated about indentation styles.
# Doing it "properly" would require far more context awareness than we can
# bring to this kind of thing.
define-command -hidden sh-insert-on-new-line %[
    evaluate-commands -draft -itersel %[
        # Copy '#' comment prefix and following white spaces.
        try %[ execute-keys -draft k <a-x> s ^\h*\K#\h* <ret> y gh j P ]
    ]
]

# Use custom object matching to copy indentation for the various logical
# blocks.
#
# Note that we're using a weird non-ascii character instead of [ or {{ here
# because the '[' and '{{' characters need to be available for the commands.
define-command -hidden sh-indent-on-new-line %¶
    evaluate-commands -draft -itersel %@
        # Preserve previous line indent.
        try %[ execute-keys -draft <semicolon> K <a-&> ]
        # Filter previous line.
        try %[ execute-keys -draft k : sh-trim-indent <ret> ]

        # Indent loop syntax, e.g.:
        # for foo in bar; do
        #       things
        # done
        #
        # or:
        #
        # while foo; do
        #       things
        # done
        #
        # or equivalently:
        #
        # while foo
        # do
        #       things
        # done
        #
        # indent after do
        try %[ execute-keys -draft <space> k <a-x> <a-k> \bdo$ <ret> j <a-gt> ]
        # Copy the indentation of the matching for/when - matching on the do
        # statement, so we don't need to duplicate this for the two loop
        # structures.
        try %[ execute-keys -draft <space> k <a-x> <a-k> \bdone$ <ret> gh [c\bdo\b,\bdone\b <ret> <a-x> <a-S> 1<a-&> <space> j K <a-&> ]

        # Indent if/then/else syntax, e.g.:
        # if [ $foo = $bar ]; then
        #       things
        # else
        #       other_things
        # fi
        #
        # or equivalently:
        # if [ $foo = $bar ]
        # then
        #       things
        # else
        #       other_things
        # fi
        #
        # indent after then
        try %[ execute-keys -draft <space> k <a-x> <a-k> \bthen$ <ret> j <a-gt> ]
        # Copy the indentation of the matching if.
        try %[ execute-keys -draft <space> k <a-x> <a-k> \bfi$ <ret> gh [c\bif\b,\bfi\b <ret> <a-x> <a-S> 1<a-&> <space> j K <a-&> ]
        # Copy the indentation of the matching if, and then re-indent afterwards.
        try %[ execute-keys -draft <space> k <a-x> <a-k> \belse$ <ret> gh [c\bif\b,\bfi\b <ret> <a-x> <a-S> 1<a-&> <space> j K <a-&> j <a-gt> ]

        # Indent case syntax, e.g.:
        # case "$foo" in
        #       bar) thing1;;
        #       baz)
        #               things
        #               ;;
        #       *)
        #               default_things
        #               ;;
        # esac
        #
        # or equivalently:
        # case "$foo"
        # in
        #       bar) thing1;;
        # esac
        #
        # indent after in
        try %[ execute-keys -draft <space> k <a-x> <a-k> \bin$ <ret> j <a-gt> ]
        # Copy the indentation of the matching case.
        try %[ execute-keys -draft <space> k <a-x> <a-k> \besac$ <ret> gh [c\bcase\b,\besac\b <ret> <a-x> <a-S> 1<a-&> <space> j K <a-&> ]
        # Indent after ).
        try %[ execute-keys -draft <space> k <a-x> <a-k> ^\s*\(?[^(]+[^)]\)$ <ret> j <a-gt> ]
        # Deindent after ;;.
        try %[ execute-keys -draft <space> k <a-x> <a-k> ^\s*\;\;$ <ret> j <a-lt> ]

        # Indent compound commands as logical blocks, e.g.:
        # {{
        #       thing1
        #       thing2
        # }}
        #
        # or in a function definition:
        # foo () {{
        #       thing1
        #       thing2
        # }}
        #
        # We don't handle () delimited compond commands - these are technically very
        # similar, but the use cases are quite different and much less common.
        #
        # Note that in this context the '{{' and '}}' characters are reserved
        # words, and hence must be surrounded by a token separator - typically
        # white space (including a newline), though technically it can also be
        # ';'. Only vertical white space makes sense in this context, though,
        # since the syntax denotes a logical block, not a simple compound command.
        try %= execute-keys -draft <space> k <a-x> <a-k> (\s|^)\{{$ <ret> j <a-gt> =
        # Deindent closing }}.
        try %= execute-keys -draft <space> k <a-x> <a-k> ^\s*\}}$ <ret> <a-lt> j K <a-&> =
        # Deindent closing }} when after cursor.
        try %= execute-keys -draft <a-x> <a-k> ^\h*\}} <ret> gh / \}} <ret> m <a-S> 1<a-&> =
    @
¶

§
"#,
        keywords_all = format!("{} {}", KEYWORDS.join(" "), BUILTINS.join(" ")),
        keywords = KEYWORDS.join("|"),
        builtins = BUILTINS.join("|"),
    )?;

    Ok(buf)
}
