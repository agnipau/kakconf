use std::fmt::Write;

pub fn rust() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r##"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*[.](rust|rs) %[
    set-option buffer filetype rust
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=rust %<
    require-module rust
    hook window ModeChange pop:insert:.* -group rust-trim-indent rust-trim-indent
    hook window InsertChar \n -group rust-indent rust-indent-on-new-line
    hook window InsertChar \{{ -group rust-indent rust-indent-on-opening-curly-brace
    hook window InsertChar [)}}\]] -group rust-indent rust-indent-on-closing
    hook -once -always window WinSetOption filetype=.* %[ remove-hooks window rust-.+ ]
>

hook -group rust-highlight global WinSetOption filetype=rust %[
    add-highlighter window/rust ref rust
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/rust ]
]

provide-module rust %§

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/rust regions
add-highlighter shared/rust/code default-region group
add-highlighter shared/rust/string region %[(?<!')"] (?<!\\)(\\\\)*" fill string
add-highlighter shared/rust/raw_string region -match-capture %[(?<!')r(#*)"] %["(#*)] fill string

add-highlighter shared/rust/line_doctest region ^\h*//[!/]\h*```($|should_panic|no_run|ignore|allow_fail|rust|test_harness|compile_fail|E\d{{4}}|edition201[58]) ^\h*//[!/]\h*```$ regions
add-highlighter shared/rust/line_doctest/marker region ``` $ group
add-highlighter shared/rust/line_doctest/marker/fence regex ``` 0:meta
add-highlighter shared/rust/line_doctest/marker/keywords regex [\d\w] 0:meta # already matched above, just ignore comma
add-highlighter shared/rust/line_doctest/inner region '^\h*//[!/]( #(?= ))?' '$| ' group
add-highlighter shared/rust/line_doctest/inner/comment regex //[!/] 0:documentation
add-highlighter shared/rust/line_doctest/inner/hidden regex '#' 0:meta
add-highlighter shared/rust/line_doctest/code default-region ref rust
add-highlighter shared/rust/line_code_rest region ^\h*//[!/]\h*``` ^\h*//[!/]\h*```$ fill documentation # reset invalid doctest
add-highlighter shared/rust/line_comment2 region //[!/]{{2}} $ fill comment
add-highlighter shared/rust/line_doc region //[!/] $ fill documentation
add-highlighter shared/rust/line_comment1 region // $ fill comment

add-highlighter shared/rust/block_comment2 region -recurse /\*\*\* /\*\*\* \*/ fill comment
add-highlighter shared/rust/block_doc region -recurse /\*\* /\*\* \*/ regions
add-highlighter shared/rust/block_doc/doctest region ```($|should_panic|no_run|ignore|allow_fail|rust|test_harness|compile_fail|E\d{{4}}|edition201[58]) ```$ regions
add-highlighter shared/rust/block_doc/doctest/marker region ``` $ group
add-highlighter shared/rust/block_doc/doctest/marker/fence regex ``` 0:meta
# Already matched above, just ignore comma.
add-highlighter shared/rust/block_doc/doctest/marker/keywords regex [\d\w] 0:meta
add-highlighter shared/rust/block_doc/doctest/inner default-region group
add-highlighter shared/rust/block_doc/doctest/inner/hidden regex '^\h*\**\h*#' 0:meta
add-highlighter shared/rust/block_doc/doctest/inner/comment regex ^\h*\* 0:documentation
add-highlighter shared/rust/block_doc/doctest/inner/code ref rust
add-highlighter shared/rust/block_doc/code_rest region ``` ``` fill documentation
add-highlighter shared/rust/block_doc/doc default-region fill documentation
add-highlighter shared/rust/block_comment1 region -recurse /\* /\* \*/ fill comment

add-highlighter shared/rust/macro_attributes region -recurse "\[" "#!?\[" "\]" regions
add-highlighter shared/rust/macro_attributes/ default-region fill meta
add-highlighter shared/rust/macro_attributes/string region %[(?<!')"] (?<!\\)(\\\\)*" fill string
add-highlighter shared/rust/macro_attributes/raw_string region -match-capture %[(?<!')r(#*)"] %["(#*)] fill string

add-highlighter shared/rust/code/long_quoted regex "('\w+)[^']" 1:meta
add-highlighter shared/rust/code/field_or_parameter   regex (_?\w+)(?::)(?!:) 1:variable
add-highlighter shared/rust/code/namespace regex [a-zA-Z](\w+)?(\h+)?(?=::) 0:module
add-highlighter shared/rust/code/field regex ((?<!\.\.)(?<=\.))_?[a-zA-Z]\w*\b 0:meta
add-highlighter shared/rust/code/function_call regex _?[a-zA-Z]\w*\s*(?=\() 0:function
add-highlighter shared/rust/code/user_defined_type regex \b[A-Z]\w*\b 0:type
add-highlighter shared/rust/code/function_declaration regex (?:fn\h+)(_?\w+)(?:<[^>]+?>)?\( 1:function
add-highlighter shared/rust/code/variable_declaration regex (?:let\h+(?:mut\h+)?)(_?\w+) 1:variable
add-highlighter shared/rust/code/macro regex \b[A-z0-9_]+! 0:meta
add-highlighter shared/rust/code/values regex \b(?:self|true|false|[0-9][_0-9]*(?:\.[0-9][_0-9]*|(?:\.[0-9][_0-9]*)?E[\+\-][_0-9]+)(?:f(?:32|64))?|(?:0x[_0-9a-fA-F]+|0o[_0-7]+|0b[_01]+|[0-9][_0-9]*)(?:(?:i|u|f)(?:8|16|32|64|128|size))?)\b 0:value
add-highlighter shared/rust/code/attributes regex \b(?:trait|struct|enum|union|type|mut|ref|static|const|default)\b 0:attribute
add-highlighter shared/rust/code/keywords regex \b(?:let|as|fn|return|match|if|else|loop|for|in|while|break|continue|move|box|where|impl|dyn|pub|unsafe|async|await|mod|crate|use|extern)\b 0:keyword
add-highlighter shared/rust/code/char_character regex "'([^\\]|\\(.|x[0-9a-fA-F]{{2}}|u\{{[0-9a-fA-F]{{1,6}}\}}))'" 0:green
# TODO: highlight error for unicode or single escape byte character.
add-highlighter shared/rust/code/byte_character regex b'([\x00-\x5B\x5D-\x7F]|\\(.|x[0-9a-fA-F]{{2}}))' 0:yellow
add-highlighter shared/rust/code/builtin_types regex \b(?:u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize|f32|f64|bool|char|str|Self)\b 0:type
add-highlighter shared/rust/code/return regex \breturn\b 0:meta

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden rust-trim-indent %[
    # Remove trailing white spaces.
    try %[ execute-keys -draft -itersel <a-x> s \h+$ <ret> d ]
]

define-command -hidden rust-indent-on-new-line %~
    evaluate-commands -draft -itersel %<
        try %[
            try %[
                # Line comment.
                evaluate-commands -draft -save-regs '/"' %[
                    # Copy the commenting prefix.
                    execute-keys -save-regs '' k <a-x> s ^\h*//[!/]{{0,2}}\h* <ret> y
                    try %[
                        # If the previous comment isn't empty, create a new one.
                        execute-keys <a-x><a-K>^\h*//[!/]{{0,2}}$<ret> j<a-x>s^\h*<ret>P
                    ] catch %[
                        # TODO: figure out a way to not delete empty comment in
                        # current line if there is no space and text in the
                        # previous comment, remove it completely.
                        execute-keys s //.*<ret> d
                    ]
                ]
            ] catch %[
                # Block comment.
                # If the previous line isn't within a comment scope, break.
                execute-keys -draft k<a-x> <a-k>^(\h*/\*|\h+\*(?!/))<ret>

                # Find comment opening, validate it was not closed, and check
                # its using star prefixes.
                execute-keys -draft <a-?>/\*<ret><a-H> <a-K>\*/<ret> <a-k>\A\h*/\*([^\n]*\n\h*\*)*[^\n]*\n\h*.\z<ret>

                try %[
                    # If the previous line is opening the comment, insert star
                    # preceeded by space.
                    execute-keys -draft k<a-x><a-k>^\h*/\*<ret>
                    execute-keys -draft i*<space><esc>
                ] catch %[
                    try %[
                        # If the next line is a comment line insert a star.
                        execute-keys -draft j<a-x><a-k>^\h+\*<ret>
                        execute-keys -draft i*<space><esc>
                    ] catch %[
                        try %[
                            # If the previous line is an empty comment line,
                            # close the comment scope.
                            execute-keys -draft k<a-x><a-k>^\h+\*\h+$<ret> <a-x>1s\*(\h*)<ret>c/<esc>
                        ] catch %[
                            # If the previous line is a non-empty comment line,
                            # add a star.
                            execute-keys -draft i*<space><esc>
                        ]
                    ]
                ]

                # Trim trailing whitespace on the previous line.
                try %[ execute-keys -draft s\h+$<ret> d ]
                # Align the new star with the previous one.
                execute-keys K<a-x>1s^[^*]*(\*)<ret>&
            ]
        ] catch %`
            # Re-indent previous line if it starts with where to match previous
            # block.
            try %+ execute-keys -draft k <a-x> <a-k> ^\h*where\b <ret> hh <a-?> ^\h*\b(impl|fn|struct|enum|union)\b <ret> <a-S> 1<a-&> +
            # Preserve previous line indent.
            try %[ execute-keys -draft <semicolon> K <a-&> ]
            # Indent after lines ending with [{{([].+ and move first parameter to
            # own line.
            try %< execute-keys -draft [c[({{[],[)}}\]] <ret> <a-k> \A[({{[][^\n]+\n[^\n]*\n?\z <ret> L i<ret><esc> <gt> <a-S> <a-&> >
            # Indent after non-empty lines not starting with operator and not
            # ending with , or ; or {{ XXX simplify this into a single <a-k>
            # without s.
            try %< execute-keys -draft k <a-x> s [^\h].+ <ret> <a-K> \A[-+*/&|^}})<gt><lt>#] <ret> <a-K> [,<semicolon>{{](\h*/[/*].*|)$ <ret> j <a-gt> >
            # Indent after lines ending with {{.
            try %< execute-keys -draft k <a-x> <a-k> \{{$ <ret> j <a-gt> >
            # Deindent after lines starting with . and ending with }} or ) or ,
            # or ; or .await.
            try %_ execute-keys -draft k <a-x> <a-k> ^\h*\. <ret> <a-k>([}}),<semicolon>]|\.await)\h*$ <ret> j <a-lt> _
            # Align to opening curly brace or paren when newline is inserted
            # before a single closing.
            try %< execute-keys -draft <a-h> <a-k> ^\h*[)}}] <ret> h m <a-S> 1<a-&> >
            # TODO: deindent additional unmatched parenthesis.
            # try %& execute-keys -draft k <a-x> s \((?:[^)(]+|\((?:[^)(]+|\([^)(]*\))*\))*\) l Gl s\) %sh{{
            #     count previous selections length
            #     echo "j $(echo $kak_selections_length | wc -w) <a-lt>"
            # }} &
        `
        # Filter previous line.
        try %[ execute-keys -draft k : rust-trim-indent <ret> ]
    >
~

define-command -hidden rust-indent-on-opening-curly-brace %[
    evaluate-commands -draft -itersel %_
        # Align indent with opening paren when {{ is entered on a new line after
        # the closing paren.
        try %[ execute-keys -draft h <a-F> ) M <a-k> \A\(.*\)\h*\n\h*\{{\z <ret> s \A|.\z <ret> 1<a-&> ]
        # Deindent standalone {{ after impl and related block without any {{ in
        # between.
        try %< execute-keys -draft hh <a-?> ^\h*\b(impl|fn|struct|enum|union|if|for)\b <ret> <a-K> \{{ <ret> <a-semicolon> <semicolon> ll <a-x> <a-k> ^\h*\{{$ <ret> <a-lt> >
    _
]

define-command -hidden rust-indent-on-closing %~
    evaluate-commands -draft -itersel %_
        # Align to opening curly brace or paren when alone on a line.
        try %< execute-keys -draft <a-h> <a-k> ^\h*[)}}\]]$ <ret> h m <a-S> 1<a-&> >
    _
~

§
"##
    )?;

    Ok(buf)
}
