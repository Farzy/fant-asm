use clap::{App, Arg, SubCommand};

mod ieee754;

fn main() {
    let matches = App::new("Fant-asm helper")
        .about("Helper functions for learning assembly")
        .version("1.0")
        .author("Farzad FARID <farzy@farzy.org>")
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
    } else {
        eprintln!("{}", matches.usage());
        std::process::exit(2);
    }
}
