use regex::Regex;

fn main() {
    let contents = match aoclib::lib::read_data("aoc2023/src/day1/calibration.txt") {
        Ok(contents) => contents,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    part1(contents.clone());

    part2(contents);
}

fn part2(contents: String) {
    let regx = match Regex::new(r"(?i)(\d{1}|one|two|three|four|five|six|seven|eight|nine)") {
        Ok(regx_set) => regx_set,
        Err(err) => panic!("error while compiling the regex : {}", err),
    };
    let mut total: u32 = 0;

    let mut count: u16 = 0;
    for line in contents.lines() {
        let mut caps = regx.captures_iter(line);

        let first = caps
            .next()
            .map_or("", |l| l.get(0).map_or("", |l1| l1.as_str()));

        let last = caps
            .last()
            .map_or("", |l| l.get(0).map_or("", |l1| l1.as_str()));

        let parse_text = |num_text: &str| -> u8 {
            return match num_text {
                "zero" => 0,
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            };
        };

        let fst: u8 = match first.parse::<u8>() {
            Ok(rs) => rs,
            Err(_) => parse_text(first),
        };

        let lst: u8 = if last.is_empty() {
            fst
        } else {
            match last.parse::<u8>() {
                Ok(rs) => rs,
                Err(_) => parse_text(last),
            }
        };

        let num = format!("{}{}", fst, lst).parse::<u32>().unwrap();

        total = num + total;

        count = count + 1;

        if count == 2 {
            println!("line {}:  {}", count, num);
        }
    }

    println!("new total is {}", total);
}

fn part1(contents: String) {
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
