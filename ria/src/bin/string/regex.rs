use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{Arg, Command};
use regex::Regex;
// run this
// cargo run --bin regex -- --pattern "book"
fn main() -> std::io::Result<()> {
    let _haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    let args = Command::new("grep")
        .version("0.1")
        .about("Searches for a pattern")
        .arg(
            Arg::new("pattern")
                .long("pattern")
                .help("The pattern to search for")
                .required(true)
                .num_args(1),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();

    let re = Regex::new(pattern).unwrap();

    // for line in haystack.lines() {
    //     match re.find(line) {
    //         Some(_) => println!("{}", line),
    //         None => (),
    //     }
    // }
    // Open the file
    let f = File::open("../README.md").unwrap();

    // Create a BufReader
    let reader = BufReader::new(f);

    // String to hold each line
    // let mut line = String::new();

    // for line_ in reader.lines() {
    //     let line = line_.unwrap();
    //     match re.find(&line) {
    //         Some(_) => println!("{}", line),
    //         None => (),
    //     }
    // }

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if let Some(_) = re.find(&line) {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading from file {}", e);
            }
        }
    }
    Ok(())
}
// Read lines in a loop
// loop {
//     let len = reader.read_line(&mut line).unwrap();
//     if len == 0 {
//         break; // EOF reached
//     }

//     println!("line::{} len::{}", line.trim_end(), len);

//     // Clear the line buffer for the next read
//     line.clear();
// }
