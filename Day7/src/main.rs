use std::{collections::HashMap, ops::Add};


fn main() {
    let content = include_str!("../input.txt").to_string();
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("/".to_string(), 0);
    let mut location: Vec<String> = Vec::new();
    for line in content.lines() {
        let first_char = line.chars().next().unwrap();      
        match first_char {
            '$' => {
                if line.contains("ls") {
                    continue;
                }
                if line.contains("..") {
                    location.remove(location.len() -1);
                } else {
                    let directory = line.split(' ').last().unwrap();
                    let path = location.concat().add(directory);
                    location.push(path);
                }
            },
            'd' => 
            {
                let directory = line.split_whitespace().last().unwrap();
                let path = location.concat().add(directory);
                map.insert(path, 0);
            },
            '0'..='9' => {
                let size: u32 = line.split_whitespace().filter_map(|num| num.parse::<u32>().ok()).collect::<Vec<u32>>()[0];
                for loc in location.iter() {
                    map.insert(loc.to_string(), map.get(&loc.to_string()).unwrap().add(size));
                }

            },
            _ => println!("don't know"),
        }
    }
    let mut total_size = 0;
    for dir in map.iter() {
        if dir.1 < &100000 {
            total_size += dir.1;
        }
    }
    let needed = 30000000 - (70000000 - map.get(&"/".to_string()).unwrap());
    println!("needed is {}", needed);
    let smallest = map.into_values().into_iter().filter(|x| x > &needed).min().unwrap();
    println!("smallest is {}", smallest);
    println!("total size to delete is {}", total_size);
}
