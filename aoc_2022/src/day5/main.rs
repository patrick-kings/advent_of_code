use regex::Regex;

fn main() {
    read_crates_from_file()
}

fn read_crates_from_file() {
    lazy_static::lazy_static! {
        static ref REGX: Regex = Regex::new(r"(?P<stack1>(\[[A-Z]\]){0,}) (?P<stack2>(\[[A-Z]\]){0,}) (?P<stack3>(\[[A-Z]\]){0,})").unwrap();
    }

    let contents = std::fs::read_to_string("aoc_2022/crates.txt").unwrap();

    let cargo: Vec<Stack> = vec![];

    for line in contents.lines() {
        // println!("{}", line);
        for caps in REGX.captures_iter(line) {
            println!(
                "{:?} {:?} {:?}",
                &caps["stack1"], &caps["stack2"], &caps["stack3"]
            );

            let mut stack1: Vec<String> = vec![];
            let stack2: Vec<String> = vec![];
            let stack3: Vec<String> = vec![];

            stack1.push(caps["stack1"].to_owned());
        }
    }
}

pub struct Stack {
    pub crates: Vec<String>,
}
