use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::process;

fn main() {
    let file = open_file();
    let mut elfs = parse_file(file);
    let first = find_highest_value(&elfs);
    elfs.remove(first.1);
    println!("{}", first.0);
    let second = find_highest_value(&elfs);
    elfs.remove(second.1);
    println!("{}", second.0);
    let third = find_highest_value(&elfs);
    elfs.remove(third.1);
    println!("{}", third.0);
    println!("togheter: {}", first.0 + second.0 + third.0);
}

fn parse_file(mut file: File) -> Vec<u32> {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Problem Reading From File: {}", e);
            process::exit(1);
        }
    };
    let mut elfs: Vec<u32> = Vec::new();
    let mut calories: u32 = 0;
    for line in content.lines() {
        if !line.is_empty() {
            let text = String::from(line);
            let as_int = text.parse::<u32>().unwrap();
            calories += as_int;
        } else {
            elfs.push(calories);
            calories = 0;
        }
    }
    drop(file);
    elfs
}

fn find_highest_value(elfs: &[u32]) -> (u32, usize) {
    let mut elf_index: usize = 0;
    let mut highest_calories: u32 = 0;
    for (index, calories) in elfs.iter().enumerate() {
        if calories > &highest_calories {
            highest_calories = *calories;
            elf_index = index;
        }
    }
    (highest_calories, elf_index)
}


fn open_file() -> File {
    let file = match OpenOptions::new()
        .read(true)
        .open("input.txt")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Problem Opening {:?} : {}",
                "input.txt", e
            );
            process::exit(1);
        }
    };
    file
}
