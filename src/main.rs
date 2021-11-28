extern crate tree_magic;

use std::path::Path;
use std::fs::metadata;
use std::{io, str};
use std::env;
use std::process;


fn get_mimetype(target_file: &str) {
    let file_path = Path::new(target_file);
    let mut mtype: String = "none".to_string();
    if Path::new(file_path).exists() { 
        let md = metadata(file_path).unwrap();
        if md.is_file() {
            mtype = tree_magic::from_filepath(file_path);
        }
    }

    println!("{}", mtype);
}

fn print_help() {
    println!("\nAuthor: Brian Kellogg");
    println!("Determine a file's mimetype.");
    println!("See: https://docs.rs/tree_magic/latest/tree_magic/\n");
    println!("\nUsage: mimetype <file path>\n");
    process::exit(1)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if !(args.len() == 2) { print_help() }
    let file_path = &args[1];
    get_mimetype(file_path);
    Ok(())
}