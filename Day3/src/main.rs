use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::process;

fn main() {
    let file = open_file("input.txt".to_string());
    let prio = parse_file(&file);
    println!("final priority is {}", prio);
    let file2 = open_file("input.txt".to_string());
    let prio = parse_badge(&file2);
    println!("badge priority is {}", prio);
    
}

fn parse_file(mut file: &File) -> i32 {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Problem Reading From File: {}", e);
            process::exit(1);
        }
    };
    let mut prio = 0;
    for line in content.lines() {
        let pockets = line.split_at(line.len() / 2);
        //could probably filter this is a better way
        let mut found_before: Vec<char> = vec![];
        for character in pockets.1.chars() {
            if pockets.0.contains(character) && !found_before.contains(&character) {
                found_before.push(character);
                println!("found {} in both", character);
                prio += calculate_priority(&character);
            }
        }
    }
    prio
}

fn parse_badge(mut file: &File) -> i32 {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Problem Reading From File: {}", e);
            process::exit(1);
        }
    };
    let line_count = content.lines().count();
    println!("content has {} lines", line_count);
    let lines = content.lines().enumerate().map(|(index, line)| (index,  line)).collect::<HashMap<usize, &str>>();
    let mut prio = 0;
    let mut i: usize = 0;
    let mut test_count = 0;
    while  i < line_count {
        test_count += 1;
        let mut found_before: Vec<char> = vec![];
        for char in lines[&i].chars()  {
            if lines[&(&i +1)].contains(char) && lines[&(&i +2)].contains(char) && !found_before.contains(&char) {
                found_before.push(char);
                println!("found {} in all", char);
                prio += calculate_priority(&char);
            }
        }
        i += 3;
    }
    println!("{} groups", test_count);
    prio
}

fn calculate_priority(character: &char) -> i32 {
    let priorities = ('a'..='z').chain('A'..='Z').enumerate().map(|(index, char)| (char, index + 1)).collect::<HashMap<char, usize>>();
    println!("priority: {}", priorities[character]);
    priorities[character] as i32
}

fn open_file(name: String) -> File {
    let file = match OpenOptions::new()
        .read(true)
        .open(name)
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

