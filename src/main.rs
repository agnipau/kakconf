mod clipboard;
mod escape;
mod inc_dec_number;
mod wrap;

use {
    clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand},
    clipboard::Direction,
    inc_dec_number::IncDecNumber,
    std::{
        convert::TryFrom,
        io::{self, Read},
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
            SubCommand::with_name("double-string")
                .about("Doubles all occurences of a string inside the piped string")
                .arg(
                    Arg::with_name("SUB_STRING")
                        .required(true)
                        .help("The string to double inside `MASTER_STRING`"),
                ),
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
            SubCommand::with_name("wrap-comment")
                .about("Wrap comments to fit N chars for a specific language")
                .arg(
                    Arg::with_name("filetype")
                        .short("f")
                        .long("filetype")
                        .takes_value(true)
                        .required(true)
                        .help("%opt{filetype}")
                )
                .arg(
                    Arg::with_name("column")
                        .short("c")
                        .long("column")
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
        .get_matches();

    match matches.subcommand() {
        ("double-string", Some(matches)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let sub_string = matches.value_of("SUB_STRING").unwrap();
            let result = escape::double_string(&piped_s, sub_string);
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
        ("wrap-comment", Some(matches)) => {
            let mut piped_s = String::new();
            io::stdin().read_to_string(&mut piped_s)?;
            let column = matches.value_of("column").unwrap();

            let prefixes: Option<&[&str]> = match matches.value_of("filetype").unwrap() {
                "rust" => Some(&["// ", "/// ", "//! "]),
                "sh" | "python" | "kak" | "toml" => Some(&["# "]),
                _ => None,
            };
            if let Some(prefixes) = prefixes {
                if let Some(s) = wrap::wrap(prefixes, &piped_s, column.parse()?) {
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
        _ => unreachable!(),
    }

    Ok(())
}
