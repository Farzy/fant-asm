#[macro_use]
extern crate clap;
use clap::{App, Arg, Shell, SubCommand};
use std::io;

mod ieee754;
mod unicode;

fn build_cli() -> App<'static, 'static> {
    App::new("Fant-asm helper")
        .about("Helper functions for learning assembly")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("ieee754")
                .about("Floating point conversion")
                .arg(
                    Arg::with_name("HEX")
                        .help("Hexadecimal representation of an IEEE 754 floating point")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("unicode").about("Play with Unicode / UTF-8"))
        .subcommand(
            SubCommand::with_name("generate-bash-completions").about("Output Bash completion"),
        )
        .subcommand(
            SubCommand::with_name("generate-zsh-completions").about("Output Zsh completion"),
        )
}

fn main() {
    let matches = build_cli().get_matches();

    if matches.is_present("generate-bash-completions") {
        build_cli().gen_completions_to(crate_name!(), Shell::Bash, &mut io::stdout());
        std::process::exit(0);
    }

    if matches.is_present("generate-zsh-completions") {
        build_cli().gen_completions_to(crate_name!(), Shell::Zsh, &mut io::stdout());
        std::process::exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("ieee754") {
        // println!("Using HEX: {}", matches.value_of("HEX").unwrap());
        let mut s = matches.value_of("HEX").unwrap();
        if &s[0..2] == "0x" || &s[0..2] == "0X" {
            s = &s[2..];
        }
        let hex = match u32::from_str_radix(s, 16) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error parsing hexadecimal number '{}: {}", s, e);
                std::process::exit(1);
            }
        };
        ieee754::main(hex);
    } else if matches.subcommand_matches("unicode").is_some() {
        unicode::main();
    } else {
        eprintln!("{}", matches.usage());
        std::process::exit(2);
    }
}
