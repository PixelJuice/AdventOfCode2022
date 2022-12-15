
fn main() {
    let content = include_str!("../input.txt").to_string();
    let mut board:Vec<Vec<u32>> = Vec::new();
    for line in content.lines() {
        let mut line_vec: Vec<u32> = Vec::new();
        for char in line.chars() {
            line_vec.push(char.to_digit(10).unwrap());
        }
        board.push(line_vec);
    }
    let mut seen_trees = 0;
    let mut scores : Vec<u32> = Vec::new();
    for (row_index, row) in board.iter().enumerate() {   
        for (column_index, _column) in row.iter().enumerate() {
            let (seen, score) = check_directions(row_index, column_index, &board);
            scores.push(score);
            if seen {
                seen_trees += 1;
            }
        }
    }
    println!("{} number of trees seen", seen_trees);
    println!("highest score is {}", scores.iter().max().unwrap());
}

fn check_directions(row:usize, column:usize, board: &Vec<Vec<u32>>) -> (bool, u32) {
    let mut can_see_right = true;
    let mut can_see_up = true;
    let mut can_see_down = true;
    let mut can_see_left = true;
    let tree = board[row][column];
    let mut score_vec : Vec<u32> = Vec::new();
    //check right
    let mut score:u32 = 0;
    for i in column+1..board[row].len()  {
        score += 1;
        if tree <= board[row][i] {
            can_see_right = false;
            break;
        }
    }
    score_vec.push(score);
    score = 0;
    //check up
    for i in (0..row).rev()  {
        score += 1;
        if tree <= board[i][column] {
            can_see_up = false;
            break;
        }
    }
    score_vec.push(score);
    score = 0;
    //check down
    for i in row+1..board.len()  {
        score += 1;
        if tree <= board[i][column] {
            can_see_down = false;
            break;
        }
    }
    score_vec.push(score);
    score = 0;
    //check left
    for i in (0..column).rev() {
        score += 1;
        if tree <= board[row][i] {
            can_see_left = false;
            break;
        }
    }
    //could be more rusty
    score_vec.push(score);
    let mut final_score = score_vec.iter().next().unwrap().clone();
    for score in score_vec.iter().skip(1) {
        final_score *= score;
    }

    (can_see_right || can_see_up || can_see_down || can_see_left, final_score)
}
