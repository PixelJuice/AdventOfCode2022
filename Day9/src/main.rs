fn main() {
    let content = include_str!("../input.txt").to_string();
    let mut uniques: Vec<Vec<(i32,i32)>> = vec![Vec::new(); 10];
    let mut knots: Vec<(i32, i32)> = vec![(0,0); 10];
    for line in content.lines() {
        let intruction = parse_instruction(line);
        for _i in 0..intruction.1 {
            move_head(intruction.0, &mut knots[0]);
            for index in 1..knots.len() {
                let dist = calculate_distance(knots[(index -1)],  knots[index]);
                act_on_instruction(dist, &mut knots[index]);
                if !uniques[index].contains(&knots[index]) {
                    uniques[index].push(knots[index]);
                }
            }
        }
    }
    println!("{} unique positions part 1", uniques[1].len());
    println!("{} unique positions part 2", uniques[9].len());
}

fn move_head(intruction: (i32, i32), head_pos: &mut (i32, i32)) {
    head_pos.0 += intruction.0;
    head_pos.1 += intruction.1;
}

fn calculate_distance(follow_pos: (i32, i32), knot_pos: (i32, i32)) -> (i32, i32) {
    (follow_pos.0 - knot_pos.0, follow_pos.1 - knot_pos.1)
}

fn act_on_instruction(distance: (i32, i32), knot_pos: &mut (i32, i32)) {
    if distance.0.abs() > 1 || distance.1.abs() > 1 {
        knot_pos.0 += distance.0.signum();
        knot_pos.1 += distance.1.signum();
    }
}

fn parse_instruction(line: &str) -> ((i32,i32), i32){
    let mut instruction: ((i32,i32), i32) = ((0,0),0);
    let first_char = line.chars().next().unwrap();      
    match first_char {
        'R' => instruction.0 = (1, 0),
        'L' => instruction.0 = (-1, 0),
        'U' => instruction.0 = (0, 1),
        'D' => instruction.0 = (0, -1),
        _ => println!("unhandled"),
    }
    instruction.1 = line.split_whitespace().last().map(|num| num.parse::<i32>().ok()).unwrap().unwrap();
    instruction
}
