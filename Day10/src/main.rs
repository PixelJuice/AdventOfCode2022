
fn main() {
    let content = include_str!("../input.txt").to_string();
    let mut signal: Vec<i32> = Vec::new();
    let mut signal_strength: i32 = 1;
    let mut display:Vec<char> = Vec::new();
    let mut cycle: i32 = 1;
    signal.push(signal_strength);
    for line in content.lines() {
        let command = line.split_once(" ").unwrap_or(("noop", "none"));
        let first_char = command.0.chars().next().unwrap();
        match first_char {
            'a' => {
                draw(cycle, signal_strength, &mut display);
                signal.push(signal_strength);

                cycle += 1;

                draw(cycle, signal_strength, &mut display);
                signal_strength += command.1.parse::<i32>().unwrap();
                signal.push(signal_strength);

                cycle += 1;
            },
            'n' => {
                draw(cycle, signal_strength, &mut display);
                signal.push(signal_strength);
                cycle += 1;
            },
            _ => println!("unkown command")
        }
        
    }
    let mut i = 0;
    while i < display.len() {
        let mut x = 0;
        while x <= 39 {
            print!("{}", display[i]);
            i += 1;
            x += 1;   
        }
        println!();
    }
    /*println!("20 is at {} * 20 == {}",signal[19] , (signal[19] * 20));
    println!("60 is at {} * 60 == {}",signal[59] , (signal[59] * 60));
    println!("100 is at {} * 100 == {}",signal[99] , (signal[99] * 100));
    println!("140 is at {} * 140 == {}",signal[139] , (signal[139] * 140));
    println!("180 is at {} * 180 == {}",signal[179] , (signal[179] * 180));
    println!("220 is at {} * 180 == {}",signal[219] , (signal[219] * 220));*/
    println!("combine is {}", (signal[19] * 20) + (signal[59] * 60) + (signal[99] * 100) + (signal[139] * 140) + (signal[179] * 180) + (signal[219] * 220));
}

fn draw(cycle: i32, signal: i32, display: &mut Vec<char>) {
    let compare = cycle % 40;
    if compare  == signal || compare == signal +1 || compare  == signal +2 {
        display.push('#'); 
    } else {
        display.push('.'); 
    }
}