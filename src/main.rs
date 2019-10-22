extern crate jsonist;
use jsonist::format;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_path = args[1].clone();

        let json = fs::read_to_string(&file_path)
            .expect(&format!("No file with name {}", file_path));
       
        match format(json, None) {
            Ok(formatted_json) => {
               fs::write(file_path, formatted_json).expect("Unable to write file");                     
            }
            Err(e) => panic!("{}", e),
        }
    }
}
