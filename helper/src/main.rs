#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

mod ieee754;
mod unicode;

fn main() {
    let matches = App::new("Fant-asm helper")
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
        .get_matches();

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
