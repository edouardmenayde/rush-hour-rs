use std::fs::File;
use std::io::prelude::*;

mod puzzle;

use puzzle::Puzzle;

fn main() -> std::io::Result<()> {
    let mut file = File::open("assets/puzzles/31.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{}", content);

    let puzzle = Puzzle::parse(content.lines());

    println!("{}", &puzzle);

    Ok(())
}
