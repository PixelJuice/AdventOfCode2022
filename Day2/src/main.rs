use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::process;

fn main() {
    let file = open_file();
    let points = parse_file(file);
    println!("{} games were played", points.len());
    let sum: i32 = points.iter().sum();
    println!("{}", sum);
}

fn parse_file(mut file: File) -> Vec<i32>{
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Problem Reading From File: {}", e);
            process::exit(1);
        }
    };
    let mut points: Vec<i32> = Vec::new();
    for line in content.lines() {
        let game: Vec<char> = line.chars().collect();
        points.push(calculate_points(&game));
    }
    drop(file);
    points
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
//a = rock, b = paper, c = scissor
fn calculate_points(game : &[char]) -> i32 {
    let mut points: i32 = 0;
    /*match game[2] {
        'X' => points += 1,
        'Y' => points += 2,
        'Z' => points += 3,
        _ => println!("this should not be here"),
    }
    match (game[0], game[2]) {
        ('A', 'X') => points += 3,
        ('A', 'Y') => points += 6,
        ('A', 'Z') => points += 0,
        ('B', 'X') => points += 0,
        ('B', 'Y') => points += 3,
        ('B', 'Z') => points += 6,
        ('C', 'X') => points += 6,
        ('C', 'Y') => points += 0,
        ('C', 'Z') => points += 3,
        (_, _) => println!("this should not be here"),
    }*/
    match (game[0], game[2]) {
        ('A', 'X') => points += 3,
        ('A', 'Y') => points += 4,
        ('A', 'Z') => points += 8,
        ('B', 'X') => points += 1,
        ('B', 'Y') => points += 5,
        ('B', 'Z') => points += 9,
        ('C', 'X') => points += 2,
        ('C', 'Y') => points += 6,
        ('C', 'Z') => points += 7,
        (_, _) => println!("this should not be here"),
    }
    println!("points: {}, {} vs {}", points, game[0], game[2]);
    points
}
