use clap::{App, Arg};

mod ieee754;

fn main() {
    let matches = App::new("Fant-asm helper")
        .about("Helper functions for learning assembly")
        .version("1.0")
        .author("Farzad FARID <farzy@farzy.org>")
        .arg(
            Arg::with_name("ieee754")
                .help("Floating point conversion")
                .long("ieee754"),
        )
        .get_matches();

    if matches.is_present("ieee754") {
        ieee754::main();
    }
}
