use std::fs::File;
use std::io::Read;
fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();
    let mut total = 0;
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let possiblitys = vec![("red", 12), ("green", 13), ("blue", 14)];
    let mut exsit_ids: Vec<u32> = Vec::new();
    for line in lines.clone().into_iter() {
        if line.len() == 0 {
            break;
        }
        let parts = line.split(":");
        let game_move = parts.clone().last();
        let game_move = game_move.unwrap();
        let game_number = parts.clone().next().unwrap().split(" ").last().unwrap();
        let single_game_moves = game_move.split(";");
        let mut maximum_value_per_color: Vec<(&str, u32)> =
            vec![("red", 0), ("green", 0), ("blue", 0)];
        for single_game_move in single_game_moves {
            let single_game_move_parts = single_game_move.split(",");
            let everymove = single_game_move_parts.collect::<Vec<&str>>();
            for moves in everymove {
                let move_parts = moves.split(" ");
                let move_parts = move_parts.collect::<Vec<&str>>();
                let move_parts_value = move_parts[1];
                let move_parts_value = move_parts_value.parse::<u32>().unwrap();
                let move_parts_color = move_parts[2];
                if move_parts_color == "red" && move_parts_value > maximum_value_per_color[0].1 {
                    maximum_value_per_color[0] = (move_parts_color, move_parts_value);
                } else if move_parts_color == "green"
                    && move_parts_value > maximum_value_per_color[1].1
                {
                    maximum_value_per_color[1] = (move_parts_color, move_parts_value);
                } else if move_parts_color == "blue"
                    && move_parts_value > maximum_value_per_color[2].1
                {
                    maximum_value_per_color[2] = (move_parts_color, move_parts_value);
                }
                for possiblity in possiblitys.iter() {
                    if move_parts_color == possiblity.0 && move_parts_value > possiblity.1 {
                        exsit_ids.push(game_number.parse::<u32>().unwrap());
                    }
                }
            }
        }
        let max_power = maximum_value_per_color.iter().map(|x| x.1);
        let max_power = max_power.fold(1, |acc, x| acc * x);
        total += max_power;
    }
    let all_ids: Vec<u32> = (1..100).map(|x| x as u32).collect();
    let mut vecofids: Vec<u32> = Vec::new();
    for ids in all_ids.into_iter() {
        if exsit_ids.contains(&ids) {
            continue;
        } else {
            vecofids.push(ids);
        }
    }
    let sum: u32 = vecofids.iter().sum();
    println!("possible ->{:#?}", sum);
    println!("power -> {:#?}", total);
}
