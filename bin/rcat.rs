use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    if let Some(file_name) = args.nth(1) {
        let mut file = File::open(&file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        println!("\n{}", content);
    } else {
        println!("\nCommand: rcat <file name>");
    }
    Ok(())
}
