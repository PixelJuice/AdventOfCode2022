
fn main() {
    let content = include_str!("../input.txt");
    let mut pairs = 0;
    for line in content.lines() {
        println!("{line}");
        let (one, two) = line.split_once(',').unwrap();
        let x: (u32, u32) = get_range_numbers(one);
        let y: (u32, u32) = get_range_numbers(two);
        if overlaps(x, y) || fully_contains(x, y) {
            pairs += 1;
        }
    }
    
    println!("pairs: {}", pairs);
}

fn get_range_numbers(string: &str) -> (u32, u32) {
    string.split_once('-').map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())).unwrap()
}

fn fully_contains(one: (u32, u32), two: (u32, u32)) -> bool {
    if one.0 <= two.0 && one.1 >= two.1 || two.0 <= one.0 && two.1 >= one.1 {
        return true
    }
    false
}

fn overlaps(one: (u32, u32), two: (u32, u32)) -> bool {
    if (one.0 >= two.0 && one.0 <= two.1) || (two.0 >= one.0 && two.0 <= one.1) {
        return true
    }
    false
}