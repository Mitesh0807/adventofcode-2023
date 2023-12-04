use std::fs::File;
use std::io::Read;
fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
    let possiblitys = vec![("red", 12), ("green", 13), ("blue", 14)];
    let mut exsit_ids: Vec<u32> = Vec::new();
    for (index, line) in lines.clone().into_iter().enumerate() {
        if line.len() == 0 {
            break;
        }
        let parts = line.split(":");
        let game_move = parts.clone().last();
        let game_move = game_move.unwrap();
        let game_number = parts.clone().next().unwrap().split(" ").last().unwrap();
        let single_game_moves = game_move.split(";");
        for single_game_move in single_game_moves {
            let single_game_move_parts = single_game_move.split(",");
            let everymove = single_game_move_parts.collect::<Vec<&str>>();
            for moves in everymove {
                let move_parts = moves.split(" ");
                let move_parts = move_parts.collect::<Vec<&str>>();
                let move_parts_value = move_parts[1];
                let move_parts_value = move_parts_value.parse::<u32>().unwrap();
                let move_parts_color = move_parts[2];
                if index == 5 {}
                for possiblity in possiblitys.iter() {
                    if move_parts_color == possiblity.0 && move_parts_value > possiblity.1 {
                        exsit_ids.push(game_number.parse::<u32>().unwrap());
                    }
                }
            }
        }
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
    println!("{:#?}", sum);
}
