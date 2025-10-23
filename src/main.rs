extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Rget")
        .version("1.0")
        .author("Nathan Tan")
        .about("wget clone in Rust")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .help("The URL to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("Downloading from URL: {}", url);
    

}
