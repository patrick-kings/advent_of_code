use regex::Regex;

fn main() {
    let contents = match aoclib::lib::read_data("aoc2023/src/day1/calibration.txt") {
        Ok(contents) => contents,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    get_total(contents);
}

fn get_total(contents: String) {
    let re = match Regex::new(r"(\d{1})") {
        Ok(reg) => reg,
        Err(err) => {
            println!("couldn't compile regex, error is: {}", err);
            return;
        }
    };

    let mut total: u32 = 0;

    for line in contents.lines() {
        let rs = match re.captures(line) {
            Some(rs) => rs,
            None => continue,
        };

        let reversed = line.chars().rev().collect::<String>();

        let rs2 = match re.captures(reversed.as_str()) {
            Some(rs) => rs,
            None => continue,
        };

        let num = format!("{}{}", &rs[0], &rs2[0]).parse::<u32>().unwrap();

        total = num + total;
    }

    println!("Total is {}", total)
}
