fn main() {
    read_calibration_data();
}
fn read_calibration_data() {
    let contents = match std::fs::read_to_string("aoc2023/src/day1/calibration.txt") {
        Ok(lines) => lines,
        Err(err) => {
            println!("error reading calibration data: {}", err);
            return;
        }
    };

    for line in contents.lines() {
        println!("{}", line)
    }
}
