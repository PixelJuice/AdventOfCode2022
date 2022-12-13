use std::collections::VecDeque;

fn main() {
    let content = include_str!("../input.txt").to_string();
    let (stacks_str, instructions_str) = content.split_once("\r\n\r\n").unwrap();
    let mut stacks = get_number_of_stacks(stacks_str.lines().last().unwrap()); 
    stacks = populate_stacks(stacks, stacks_str); 
    let mut instructions:Vec<Vec<u32>> = Vec::new();
    for line in instructions_str.lines() {
        instructions.push(get_instructions(line));
    }
    move_together(instructions, &mut stacks);
    //move_one_by_one(instructions, &mut stacks);
    for mut stack in stacks {
        print!("{}", stack.pop_back().unwrap());
    }
}

fn move_together(instructions: Vec<Vec<u32>>, stacks: &mut [VecDeque<char>]) {
    for instruction in instructions {
        let mut temp: VecDeque<char> = VecDeque::new();
        for _i in 0..instruction[0]  {
            let move_char = stacks[(instruction[1] - 1) as usize].pop_back().unwrap();
            temp.push_front(move_char);
            //println!("moving {} from {} to {}", move_char, instruction[1], instruction[2]);
        }
        for thing in temp {
            stacks[(instruction[2] - 1) as usize].push_back(thing);
        }
    }
}

fn move_one_by_one(instructions: Vec<Vec<u32>>, stacks: &mut [VecDeque<char>]) {
    for instruction in instructions {
        for _i in 0..instruction[0]  {
            let move_char = stacks[(instruction[1] - 1) as usize].pop_back().unwrap();
            //println!("moving {} from {} to {}", move_char, instruction[1], instruction[2]);
            stacks[(instruction[2] - 1) as usize].push_back(move_char);
        }
    }
}

// lucky me there is only a max of 9 stacks :)
fn get_number_of_stacks(num_stacks_str: &str) -> Vec<VecDeque<char>> {
    let mut num_stacks = 0; 
    for char in num_stacks_str.chars() {
        if char.is_numeric() {
            num_stacks = char.to_digit(10).unwrap();
        }
    }
    vec![VecDeque::new(); num_stacks as usize]
}

fn populate_stacks(mut stacks: Vec<VecDeque<char>>, stacks_str: &str) -> Vec<VecDeque<char>> {
    for line in stacks_str.lines().rev() {
        for index in 0..stacks.len() {
            let line_as_bytes = line.as_bytes();
            let character = line_as_bytes[index*4+1];
            if character.is_ascii_alphabetic() {
                //println!("{}", character as char);
                stacks[index].push_back(character as char);
            }
        }
    }
    stacks
}

fn get_instructions(line: &str) -> Vec<u32> {
    let instruction: Vec<u32> =  line .split_whitespace().filter_map(|num| num.parse::<u32>().ok()).collect();
    //naive approch...
    /*let mut instructions = Vec::new();
    for char in line.chars() {
        if char.is_numeric() {
            instructions.push(char.to_digit(10).unwrap());
        }
    }*/
    instruction
}
