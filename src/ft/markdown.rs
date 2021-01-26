use std::fmt::Write;

const LANGUAGES: [&str; 47] = [
    "awk",
    "c",
    "cabal",
    "clojure",
    "coffee",
    "cpp",
    "css",
    "cucumber",
    "d",
    "diff",
    "dockerfile",
    "fish",
    "gas",
    "go",
    "haml",
    "haskell",
    "html",
    "ini",
    "java",
    "javascript",
    "json",
    "julia",
    "kak",
    "kickstart",
    "latex",
    "lisp",
    "lua",
    "makefile",
    "markdown",
    "moon",
    "objc",
    "perl",
    "pug",
    "python",
    "ragel",
    "ruby",
    "rust",
    "sass",
    "scala",
    "scss",
    "sh",
    "swift",
    "toml",
    "tupfile",
    "typescript",
    "yaml",
    "sql",
];

pub fn markdown() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*[.](markdown|md|mkd) %[
    set-option buffer filetype markdown
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=markdown %[
    require-module markdown

    hook window InsertChar \n -group markdown-indent markdown-indent-on-new-line
    hook -once -always window WinSetOption filetype=.* %[ remove-hooks window markdown-.+ ]
]

hook -group markdown-load-languages global WinSetOption filetype=markdown %[
    hook -group markdown-load-languages window NormalIdle .* markdown-load-languages
    hook -group markdown-load-languages window InsertIdle .* markdown-load-languages
]

hook -group markdown-highlight global WinSetOption filetype=markdown %[
    add-highlighter window/markdown ref markdown
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/markdown ]
]

provide-module markdown %§

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/markdown regions
add-highlighter shared/markdown/inline default-region regions
add-highlighter shared/markdown/inline/text default-region group

{}

add-highlighter shared/markdown/codeblock region -match-capture \
    ^(\h*)```\h* \
    ^(\h*)```\h*$ \
    fill meta

add-highlighter shared/markdown/listblock region ^\h*[-*]\s ^\h*((?=[-*])|$) group
add-highlighter shared/markdown/listblock/ ref markdown/inline
add-highlighter shared/markdown/listblock/marker regex ^\h*([-*])\s 1:bullet

add-highlighter shared/markdown/inline/code region -match-capture (`+) (`+) fill mono

# Setext-style header.
add-highlighter shared/markdown/inline/text/ regex (\A|^\n)[^\n]+\n={{2,}}\h*\n\h*$ 0:title
add-highlighter shared/markdown/inline/text/ regex (\A|^\n)[^\n]+\n-{{2,}}\h*\n\h*$ 0:header

# Atx-style header.
add-highlighter shared/markdown/inline/text/ regex ^#[^\n]* 0:header

add-highlighter shared/markdown/inline/text/ regex (?<!\*)(\*([^\s*]|([^\s*](\n?[^\n*])*[^\s*]))\*)(?!\*) 1:+i
add-highlighter shared/markdown/inline/text/ regex (?<!_)(_([^\s_]|([^\s_](\n?[^\n_])*[^\s_]))_)(?!_) 1:+i
add-highlighter shared/markdown/inline/text/ regex (?<!\*)(\*\*([^\s*]|([^\s*](\n?[^\n*])*[^\s*]))\*\*)(?!\*) 1:+b
add-highlighter shared/markdown/inline/text/ regex (?<!_)(__([^\s_]|([^\s_](\n?[^\n_])*[^\s_]))__)(?!_) 1:+b
add-highlighter shared/markdown/inline/text/ regex <(([a-z]+://.*?)|((mailto:)?[\w+-]+@[a-z]+[.][a-z]+))> 0:link
add-highlighter shared/markdown/inline/text/ regex ^\[[^\]\n]*\]:\h*([^\n]*) 1:link
add-highlighter shared/markdown/inline/text/ regex ^\h*(>\h*)+ 0:comment
add-highlighter shared/markdown/inline/text/ regex "\H( {{2,}})$" 1:+r@meta

# Inline code.
add-highlighter shared/markdown/inline/text/ regex "^( {{4}}|\t)+([^\n]+)" 2:meta

# Commands.
# ‾‾‾‾‾‾‾‾‾

define-command -hidden markdown-indent-on-new-line %[
    evaluate-commands -draft -itersel %[
        # Copy block quote(s), list item prefix and following white spaces.
        try %[ execute-keys -draft k <a-x> s ^\h*\K((>\h*)+([*+-]\h)?|(>\h*)*[*+-]\h)\h* <ret> y gh j P ]
        # Preserve previous line indent.
        try %[ execute-keys -draft <semicolon> K <a-&> ]
        # Remove trailing white spaces.
        try %[ execute-keys -draft -itersel %[ k<a-x> s \h+$ <ret> d ] ]
    ]
]

define-command -hidden markdown-load-languages %[
    evaluate-commands -draft %[ try %[
        execute-keys 'gtGbGls```\h*\{{?[.=]?\K[^}}\s]+<ret>'
        evaluate-commands -itersel %[ require-module %val[selection] ]
    ]]
]

§
"#,
        {
            let mut buf = String::new();
            for lang in &LANGUAGES {
                write!(buf, r#"
add-highlighter shared/markdown/{lang} region -match-capture ^(\h*)```\h*({lang}|\{{[.=]?{lang}\}}))\b ^(\h*)``` regions
add-highlighter shared/markdown/{lang}/ default-region fill meta
add-highlighter shared/markdown/{lang}/inner region \A```[^\n]*\K (?=```) ref {ref}
"#, lang = lang, ref = if *lang == "kak" { "kakrc" } else { lang })?;
            }
            buf
        }
    )?;

    Ok(buf)
}
