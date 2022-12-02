mod data_import;
pub use data_import::lines_from_file;

fn get_points_symbol(symbol : &str) -> i64
{
    return match symbol
    {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_points_result(symbol : &str) -> i64
{
    return match symbol
    {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}

fn get_points_match(score_player1 : i64, score_player2 : i64) -> i64
{
    if score_player1 == score_player2 //a draw gives 3 points
    {
        return 3;
    }

    if score_player1 < score_player2 && score_player2 - score_player1 == 1 || //player 1 loose
        score_player1 > score_player2 && score_player1 - score_player2 > 1
    {
        return 0;
    }
    return 6; //player 1 wins
}

fn get_points_determine_needed_symbol(score_player2 : i64, win_loose_draw : i64) -> i64
{
    return match win_loose_draw
    {
        3 => score_player2, //a draw needs the same symbol
        6 => if score_player2 + 1 == 4 {1} else {score_player2 + 1},
        0 => if score_player2 - 1 == 0 {3} else {score_player2 - 1},
        _ => 0,
    }
}

pub fn solve(task : i64) -> i64
{
    let data = lines_from_file("input_day2.txt");
    
    let mut score = 0i64;
    
    for line in data
    {
        let part = line.split(" ").collect::<Vec<&str>>();
        let opponents_score = get_points_symbol(part[0]);
        
        if task == 1
        {
            let my_score = get_points_symbol(part[1]);

            score += my_score + get_points_match(my_score, opponents_score);
        }
        else 
        {
            let result_score = get_points_result(part[1]);

            score += get_points_determine_needed_symbol(opponents_score, result_score) + result_score;
        } 
    }
    return score;
}