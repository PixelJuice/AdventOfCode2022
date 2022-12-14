fn main() {
    let content = include_str!("../input.txt").to_string();
    let mut chars = Vec::new();
    let unique_length = 14; //4 for part 1
    for (index, character) in content.chars().enumerate()  {
        if chars.len() >= unique_length {
            chars.remove(0);
        } 
        chars.push(character);
        if all_unique(&mut chars, unique_length) {
            println!("unique at {}", index + 1);
            break;
        }
    }
}

fn all_unique(original: &mut Vec<char>, unique_length: usize) -> bool {
    if original.len() != unique_length {
        return false
    }
    let mut test = original.clone();
    for _character in original {
        let pop = test.pop().unwrap();
        if test.contains(&pop) {
            return false;
        }
    }
    //this was not the way
    //test.dedup();
    true
}
