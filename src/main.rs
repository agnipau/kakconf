mod clipboard;
mod editorconfig;
mod editorconfig_sys;
mod escape;
mod fzf;
mod inc_dec_number;
mod put_cursors;
mod selections_desc;
mod wrap;

use {
    clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand},
    clipboard::Direction,
    editorconfig::EditorConfig,
    inc_dec_number::IncDecNumber,
    selections_desc::SelectionsDesc,
    std::{
        convert::TryFrom,
        io::{self, Read},
        path::Path,
    },
};

fn main() -> anyhow::Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .settings(&[
            AppSettings::ColorAuto,
            AppSettings::ArgRequiredElseHelp,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .global_settings(&[
            AppSettings::ColoredHelp,
        ])
        .subcommand(
            SubCommand::with_name("quote")
                .about("Quote a string to be used safely inside kakoune")
                .arg(
                    Arg::with_name("string")
                        .short("s")
                        .long("string")
                        .takes_value(true)
                        .required(true)
                        .help("The string to quote")
                )
        )
        .subcommand(
            SubCommand::with_name("raw-insert")
                .about("Performs a set of escaping operations to make the piped string insertable in a context like `: i${string}<esc>`")
        )
        .subcommand(
            SubCommand::with_name("paste")
                .about("Insert the system clipboard's content will be printed to STDOUT")
                .arg(
                    Arg::with_name("DIRECTION")
                        .help("Paste the text after or before the cursor")
                        .required(true)
                        .possible_values(&["after", "before"]),
                )
                .arg(
                    Arg::with_name("select")
                        .short("s")
                        .long("select")
                        .takes_value(false)
                        .required(false)
                        .help("Select the text after paste (default is false)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("wrap-text")
                .about("Wrap text to fit '--width' chars for a specific language")
                .arg(
                    Arg::with_name("filetype")
                        .short("f")
                        .long("filetype")
                        .takes_value(true)
                        .required(false)
                        .possible_values(&["python", "rust", "kak", "toml", "sh"])
                        .help("%opt{filetype}")
                )
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .takes_value(true)
                        .required(true)
                        .help("%opt{autowrap_column}")
                ),
        )
        .subcommand(
            SubCommand::with_name("inc-number")
                .about("Increment number")
                .arg(
                    Arg::with_name("other")
                        .short("o")
                        .long("other")
                        .takes_value(true)
                        .required(true)
                        .help("The value to increment/decrement")
                ),
        )
        .subcommand(
            SubCommand::with_name("dec-number")
                .about("Decrement number")
                .arg(
                    Arg::with_name("other")
                        .short("o")
                        .long("other")
                        .takes_value(true)
                        .required(true)
                        .help("The value to increment/decrement")
                ),
        )
        .subcommand(
            SubCommand::with_name("put-cursors")
                .about("Put cursors at specific line numbers")
                .arg(
                    Arg::with_name("total-lines")
                        .short("t")
                        .long("total-lines")
                        .takes_value(true)
                        .required(true)
                        .help("%val{buf_line_count}")
                )
                .arg(
                    Arg::with_name("lines")
                        .short("l")
                        .long("lines")
                        .min_values(1)
                        .required(true)
                        .help("The line numbers where to put the cursors")
                )
                .arg(
                    Arg::with_name("zero-index")
                        .short("z")
                        .long("zero-index")
                        .takes_value(false)
                        .required(false)
                        .help("Should the lines indexes be 0-based?")
                ),
        )
        .subcommand(
            SubCommand::with_name("extend-selections")
                .about("Extend selections to the left/right to align with the leftmost/rightmost selected column")
                .arg(
                    Arg::with_name("direction")
                        .short("d")
                        .long("direction")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&["left", "right"])
                        .help("Use the leftmost or rightmost column to align?")
                )
                .arg(
                    Arg::with_name("selections-desc")
                        .short("s")
                        .long("selections-desc")
                        .takes_value(true)
                        .required(true)
                        .help("%val{selections_desc}")
                ),
        )
        .subcommand(
            SubCommand::with_name("editorconfig")
                .about("A rewrite of editorconfig-core-c CLI in Rust")
                .arg(
                    Arg::with_name("file-path")
                        .short("p")
                        .long("file-path")
                        .required(true)
                        .takes_value(true)
                        .help("Full path to the file"),
                )
                .arg(
                    Arg::with_name("version")
                        .short("b")
                        .long("version")
                        .takes_value(true)
                        .help("Specify version (used by devs to test compatibility)")
                )
                .arg(
                    Arg::with_name("conf-filename")
                        .short("f")
                        .long("conf-filename")
                        .takes_value(true)
                        .help("Specify conf filename other than \".editorconfig\"")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-edit")
                .about("Open a file using FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-edit-inner")
                .about("Inner command called by tmux in fzf-edit")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-edit-inner-preview")
                .about("Inner command called by fzf in fzf-edit-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-file")
                        .short("f")
                        .long("fzf-file")
                        .required(true)
                        .takes_value(true)
                        .help("{} in `fzf --preview`")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-cd")
                .about("Change directory using FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-cd-inner")
                .about("Inner command called by tmux in fzf-cd")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-cd-inner-preview")
                .about("Inner command called by fzf in fzf-cd-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-selected-list")
                        .short("f")
                        .long("fzf-selected-list")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("{+} in `fzf --preview`")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-change-buffer")
                .about("Change buffer using FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                )
                .arg(
                    Arg::with_name("kak-buflist")
                        .short("l")
                        .long("kak-buflist")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("%val{buflist}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-change-buffer-inner")
                .about("Inner command called by tmux in fzf-change-buffer")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                )
                .arg(
                    Arg::with_name("kak-buflist")
                        .short("l")
                        .long("kak-buflist")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("%val{buflist}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-change-buffer-inner-preview")
                .about("Inner command called by fzf in fzf-change-buffer-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-file")
                        .short("f")
                        .long("fzf-file")
                        .required(true)
                        .takes_value(true)
                        .help("{} in `fzf --preview`")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-delete-buffer")
                .about("Delete buffer using FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                )
                .arg(
                    Arg::with_name("kak-buflist")
                        .short("l")
                        .long("kak-buflist")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("%val{buflist}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-delete-buffer-inner")
                .about("Inner command called by tmux in fzf-delete-buffer")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("kak-buffile")
                        .short("f")
                        .long("kak-buffile")
                        .required(true)
                        .takes_value(true)
                        .help("%val{buffile}")
                )
                .arg(
                    Arg::with_name("kak-buflist")
                        .short("l")
                        .long("kak-buflist")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("%val{buflist}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-delete-buffer-inner-preview")
                .about("Inner command called by fzf in fzf-delete-buffer-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-file")
                        .short("f")
                        .long("fzf-file")
                        .required(true)
                        .takes_value(true)
                        .help("{} in `fzf --preview`")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-lines")
                .about("Search through lines of the current buffer using FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-lines-inner")
                .about("Inner command called by tmux in fzf-lines")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
        )
        .subcommand(
            SubCommand::with_name("fzf-lines-inner-preview")
                .about("Inner command called by fzf in fzf-lines-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-indexes")
                        .short("i")
                        .long("fzf-indexes")
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("{+n} in `fzf --preview`")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-rg")
                .about("Jump to a file at a specific line using Ripgrep and FZF")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("rg-query")
                        .short("q")
                        .long("rg-query")
                        .required(true)
                        .takes_value(true)
                        .help("The query to pass to ripgrep")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-rg-inner")
                .about("Inner command called by tmux in fzf-rg")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("rg-query")
                        .short("q")
                        .long("rg-query")
                        .required(true)
                        .takes_value(true)
                        .help("The query to pass to ripgrep")
                ),
        )
        .subcommand(
            SubCommand::with_name("fzf-rg-inner-preview")
                .about("Inner command called by fzf in fzf-rg-inner")
                .arg(
                    Arg::with_name("kak-session")
                        .short("s")
                        .long("kak-session")
                        .required(true)
                        .takes_value(true)
                        .help("%val{session}"),
                )
                .arg(
                    Arg::with_name("kak-client")
                        .short("c")
                        .long("kak-client")
                        .required(true)
                        .takes_value(true)
                        .help("%val{client}")
                )
                .arg(
                    Arg::with_name("fzf-file")
                        .short("f")
                        .long("fzf-file")
                        .required(true)
                        .takes_value(true)
                        .help("{} in `fzf --preview`")
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("quote", Some(matches)) => {
            let s = matches.value_of("string").unwrap();
            let result = escape::quote(s);
            print!("{}", result);
        }
        ("raw-insert", Some(_)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let result = escape::escape_raw_insert(&piped_s);
            print!("{}", result);
        }
        ("paste", Some(matches)) => {
            let direction = Direction::try_from(matches.value_of("DIRECTION").unwrap()).unwrap();
            let select = matches.is_present("select");
            let kakcmd = clipboard::get_clipboard(&direction, select)?;
            print!("{}", kakcmd);
        }
        ("wrap-text", Some(matches)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let width = matches.value_of("width").unwrap();

            let prefixes: Option<&[&str]> = match matches.value_of("filetype") {
                Some("rust") => Some(&["// ", "/// ", "//! "]),
                Some("sh") | Some("python") | Some("kak") | Some("toml") => Some(&["# "]),
                Some(_) => Some(&[]),
                None => None,
            };
            if let Some(prefixes) = prefixes {
                if let Some(s) = wrap::wrap(prefixes, &piped_s, width.parse()?) {
                    println!("{}", s);
                } else {
                    print!("{}", piped_s);
                }
            }
        }
        ("inc-number", Some(matches)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let mut piped_s = piped_s.trim().to_owned();
            let other = matches.value_of("other").unwrap();
            let result = IncDecNumber::Increment.compute(&mut piped_s, &mut other.to_owned());
            println!("{}", result.unwrap_or(piped_s));
        }
        ("dec-number", Some(matches)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let mut piped_s = piped_s.trim().to_owned();
            let other = matches.value_of("other").unwrap();
            let result = IncDecNumber::Decrement.compute(&mut piped_s, &mut other.to_owned());
            println!("{}", result.unwrap_or(piped_s));
        }
        ("put-cursors", Some(matches)) => {
            let total_lines: usize = matches.value_of("total-lines").unwrap().parse()?;

            let zero_index = matches.is_present("zero-index");
            let mut lines: Vec<usize> = Vec::new();
            for line in matches.values_of("lines").unwrap() {
                if let Ok(line) = line.parse() {
                    lines.push(if zero_index { line + 1 } else { line });
                }
            }
            let active_cursor = *lines.last().unwrap();
            lines.sort();
            let active_cursor_idx = lines.iter().position(|&x| x == active_cursor).unwrap();

            let result = put_cursors::put_cursors(total_lines, &lines, active_cursor_idx);
            println!("{}", result);
        }
        ("extend-selections", Some(matches)) => {
            let _direction = matches.value_of("direction").unwrap();
            let seldesc = matches.value_of("selections-desc").unwrap();
            if let Some(mut seldesc) = SelectionsDesc::new(seldesc) {
                seldesc.extend_left();
                let result: String = seldesc.into();
                println!("{}", result);
            } else {
                eprintln!("Invalid %val{{selections_desc}}");
            }
        }
        ("editorconfig", Some(matches)) => {
            let version = matches
                .value_of("version")
                .and_then(|x| editorconfig::Version::new(x).ok());
            let conf_filename = matches.value_of("conf-filename");
            let file_path = matches.value_of("file-path").unwrap();

            let econfig = EditorConfig::new(conf_filename, version.as_ref())?;
            let parsed = econfig.parse(Path::new(file_path))?;

            if let Some(v) = parsed.get("indent_style") {
                match v.as_str() {
                    "tab" => {
                        println!(
                            "set-option buffer indentwidth 0
set-option buffer aligntab true"
                        );
                    }
                    "space" => {
                        let indent_size = &parsed["indent_size"];
                        let indent_width = if indent_size == "tab" {
                            4
                        } else {
                            indent_size.parse()?
                        };
                        println!(
                            "set-option buffer indentwidth {}
set-option buffer aligntab false",
                            indent_width
                        );
                    }
                    _ => {}
                }
            }

            if let Some(v) = parsed.get("indent_size") {
                println!("set-option buffer tabstop {}", v);
            }
            if let Some(v) = parsed.get("tab_width") {
                println!("set-option buffer tabstop {}", v);
            }

            if let Some(v) = parsed.get("end_of_line") {
                if v == "lf" || v == "crlf" {
                    println!("set-option buffer eolformat {}", v);
                }
            }

            if let Some(v) = parsed.get("charset") {
                if v == "utf-8-bom" {
                    println!("set-option buffer BOM utf8");
                }
            }

            // if let Some(v) = parsed.get("trim_trailing_whitespace") {
            //     if v == "true" {
            //         println!(
            //             r#"hook buffer BufWritePre {:?} -group editorconfig-hooks %[ try %[ execute-keys -draft %[ %s\h+$|\n+\z<ret>d ] ] ]"#,
            //             file_path,
            //         );
            //     }
            // }

            if let Some(v) = parsed.get("max_line_length") {
                if v != "off" {
                    println!(
                        r#"set window autowrap_column {}
remove-highlighter "window/column_%opt[autowrap_column]_default,%opt[gruvbox_bg0]"
add-highlighter window/ column "%opt[autowrap_column]" "default,%opt[gruvbox_bg0]""#,
                        v
                    );
                }
            }
        }
        ("fzf-edit", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            fzf::fzf_edit(kak_session, kak_client, kak_buffile);
        }
        ("fzf-edit-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            fzf::fzf_edit_inner(kak_session, kak_client, kak_buffile);
        }
        ("fzf-edit-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_file = matches.value_of("fzf-file").unwrap();
            fzf::fzf_edit_inner_preview(kak_session, kak_client, fzf_file);
        }
        ("fzf-cd", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            fzf::fzf_cd(kak_session, kak_client);
        }
        ("fzf-cd-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            fzf::fzf_cd_inner(kak_session, kak_client);
        }
        ("fzf-cd-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_selected_list = matches
                .values_of("fzf-selected-list")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_cd_inner_preview(kak_session, kak_client, fzf_selected_list);
        }
        ("fzf-change-buffer", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            let kak_buflist = matches
                .values_of("kak-buflist")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_change_buffer(kak_session, kak_client, kak_buffile, kak_buflist);
        }
        ("fzf-change-buffer-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            let kak_buflist = matches
                .values_of("kak-buflist")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_change_buffer_inner(kak_session, kak_client, kak_buffile, &kak_buflist);
        }
        ("fzf-change-buffer-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_file = matches.value_of("fzf-file").unwrap();
            fzf::fzf_change_buffer_inner_preview(kak_session, kak_client, fzf_file);
        }
        ("fzf-delete-buffer", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            let kak_buflist = matches
                .values_of("kak-buflist")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_delete_buffer(kak_session, kak_client, kak_buffile, kak_buflist);
        }
        ("fzf-delete-buffer-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let kak_buffile = matches.value_of("kak-buffile").unwrap();
            let kak_buflist = matches
                .values_of("kak-buflist")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_delete_buffer_inner(kak_session, kak_client, kak_buffile, &kak_buflist);
        }
        ("fzf-delete-buffer-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_file = matches.value_of("fzf-file").unwrap();
            fzf::fzf_delete_buffer_inner_preview(kak_session, kak_client, fzf_file);
        }
        ("fzf-lines", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            fzf::fzf_lines(kak_session, kak_client);
        }
        ("fzf-lines-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            fzf::fzf_lines_inner(kak_session, kak_client);
        }
        ("fzf-lines-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_indexes = matches
                .values_of("fzf-indexes")
                .unwrap()
                .collect::<Vec<_>>();
            fzf::fzf_lines_inner_preview(kak_session, kak_client, &fzf_indexes);
        }
        ("fzf-rg", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let rg_query = matches.value_of("rg-query").unwrap();
            fzf::fzf_rg(kak_session, kak_client, rg_query);
        }
        ("fzf-rg-inner", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let rg_query = matches.value_of("rg-query").unwrap();
            fzf::fzf_rg_inner(kak_session, kak_client, rg_query);
        }
        ("fzf-rg-inner-preview", Some(matches)) => {
            let kak_session = matches.value_of("kak-session").unwrap();
            let kak_client = matches.value_of("kak-client").unwrap();
            let fzf_file = matches.value_of("fzf-file").unwrap();
            fzf::fzf_rg_inner_preview(kak_session, kak_client, fzf_file);
        }
        _ => unreachable!(),
    }

    Ok(())
}
