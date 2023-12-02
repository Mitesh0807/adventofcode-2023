use core::num::ParseIntError;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let calibrations = input.lines();
    let mut sum = 0;
    for calibration in calibrations.into_iter().enumerate() {
        let (index, line) = calibration;
        let mut it = line.chars().filter_map(|character| character.to_digit(10));
        let first = it.next().expect("should be a number");
        let res = match it.last() {
            Some(num) => Ok::<Result<u32, ParseIntError>, ParseIntError>(
                format!("{first}{num}").parse::<u32>(),
            ),

            None => Ok(format!("{first}{first}").parse::<u32>()),
        }
        .expect(" should be a valid number");
        match res {
            Ok(num) => sum += num,
            Err(_) => {
                println!(" {index} {line} Error ");
            }
        }
    }
    println!("{}", sum);
}

/*
*
* input :-
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
output -
142
* */
