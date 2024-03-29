use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Loc Nguyen Vu <loc.nguyenvu@outlook.com")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do print new line")
                .required(false)
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let ending = if matches.is_present("omit_newline") { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
