use core::fmt;

use super::file::read_input;

struct Monkey {
    items: Vec<i32>,
    inspected: i32,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {{ items: {:?}, inspected: {} }}",
            self.items, self.inspected
        )
    }
}

pub fn run() {
    let input = read_input("input/d11.txt").unwrap();
    parse_input(input);
}

fn parse_input(input: Vec<String>) {
    // 0: operator
    // 1: number of operation
    // 2: number of divisible
    // 3: number of true
    // 4: number of false
    let mut monkey_no: i32 = -1;
    let mut items = Vec::<i32>::new();
    let mut opertor = "";
    for line in input {
        if line == "\n" {
		println!("Monkey {}: {:?}", monkey_no, items);
        } else {
            if line.contains("Monkey") {
                let (_f, l) = split_line(line.as_str(), "Monkey");
		monkey_no = l[..l.len()-1].trim().parse::<i32>().unwrap();
            } else if line.contains("Starting") {
                let (_f, l) = split_line(line.as_str(), "Starting items: ");
                items = l
                    .split(",")
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect();
            } else if line.contains("Operation") {

	    }
        }
    }
}

fn split_line<'a>(line: &'a str, txt: &'a str) -> (&'a str, &'a str) {
    line.split_at(txt.len() + 1)
}
