extern crate tree_magic;

use std::path::Path;
use std::{io, str};
use std::env;
use std::process;


fn get_mimetype(target_file: &Path) {
    let mtype = tree_magic::from_filepath(target_file);

    println!("{}", mtype);
}

fn convert_to_path(target_file: &str) -> io::Result<&Path> {
    let file_path = Path::new(target_file);
    if file_path.exists() && file_path.is_file() { 
        return Ok(file_path)
    }

    process::exit(1)
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
    if args.len() != 2 { print_help() }
    let file_path = &args[1];
    let path = convert_to_path(&file_path).unwrap();
    get_mimetype(path);
    Ok(())
}