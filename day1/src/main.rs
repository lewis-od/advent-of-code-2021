use day1::calc_result;
use day1::calc_windowed_result;

fn main() {
    let result = calc_result();
    println!("Part 1: {}", result);

    let windowed_result = calc_windowed_result();
    println!("Part 2: {}", windowed_result);
}
