use core::fmt;
use std::collections::HashMap;

use super::file::read_input;

#[derive(Clone)]
struct Operation {
    no: i32,
    items: Vec<i32>,
    operator: String,
    subfix: String,
    divisible: i32,
    true_throw: i32,
    false_throw: i32,
    inspected: HashMap<i32, i32>,
    time: i32,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operation {{
		no: {},
		items: {:?},
		operator: {},
		subfix: {},
		divisible: {},
		true_throw: {},
		false_throw: {},
		inspected: {:?},
		time: {}
	    }}",
            self.no,
            self.items,
            self.operator,
            self.subfix,
            self.divisible,
            self.true_throw,
            self.false_throw,
            self.inspected,
            self.time
        )
    }
}

pub fn run() {
    let input = read_input("input/d11.txt").unwrap();
    let mut operations = parse_input(input);
    let mut count = 0;
    loop {
        for i in 0..operations.len() {
            for _j in 0..operations[i].items.len() {
                let a = operate(
                    operations[i].items[0].clone(),
                    operations[i].operator.clone(),
                    operations[i].subfix.clone(),
                );
                let b = a / 3;
                let is_inspected = b % operations[i].divisible == 0;
                let t = if is_inspected {
                    operations[i].true_throw
                } else {
                    operations[i].false_throw
                };
                operations[i].time += 1;
                throw_to_monkey(&mut operations, t, b);
                operations[i].items.remove(0);
            }
        }

        // for i in 0..operations.len() {
        // 	if operations[i].items.len() > 0 {
        // 		operations[i].time += operations[i].items.len() as i32;
        // 	}
        // }

        count += 1;
        if count == 20 {
            break;
        }
    }
    //     println!("{:?}", operations);
    //     for op in operations.iter() {
    //         println!("{:?}", op.time);
    //     }
    let mut times: Vec<i32> = operations.iter().map(|op| op.time).collect();
    times.sort_by(|a, b| b.cmp(a));
    let top_2_times = &times[..2];
    println!("Top 2 max times: {:?}", top_2_times);
    let result = top_2_times.iter().product::<i32>();
    println!("Product of top 2 max times: {}", result);
}

//347 * 345 = 119415
//345

fn throw_to_monkey(o: &mut Vec<Operation>, to: i32, v: i32) {
    for op in o.iter_mut() {
        if op.no == to {
            op.inspected.insert(v, v);
            op.items.push(v);
            //     if is_inspected {
            //     op.time += 1;
            //     }
            break;
        }
    }
}

fn operate(i: i32, o: String, n: String) -> i32 {
    match o.as_str() {
        "+" => i + check_old_val(i, n),
        "-" => i - check_old_val(i, n),
        "*" => i * check_old_val(i, n),
        "/" => i / check_old_val(i, n),
        _ => 0,
    }
}

fn check_old_val(i: i32, n: String) -> i32 {
    if n == "old" {
        i
    } else {
        n.parse::<i32>().unwrap()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Operation> {
    let mut monkeys = Vec::<Operation>::new();

    let mut no: i32 = -1;
    let mut items = Vec::<i32>::new();
    let mut oper = String::from("");
    let mut num = String::from("");
    let mut divisible = 1;
    let mut true_throw = 0;
    let mut false_throw = 0;

    for line in input {
        if line == "" {
            monkeys.push(Operation {
                no,
                items: items.clone(),
                operator: oper.clone(),
                subfix: num.clone(),
                divisible,
                true_throw,
                false_throw,
                inspected: HashMap::<i32, i32>::new(),
                time: 0,
            });
        } else {
            if line.contains("Monkey") {
                let (_f, l) = split_line(line.as_str(), "Monkey");
                no = l[..l.len() - 1].trim().parse::<i32>().unwrap();
            } else if line.contains("Starting") {
                let (_f, l) = split_line(line.as_str(), "Starting items: ");
                items = l
                    .split(",")
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect();
            } else if line.contains("Operation") {
                let (_f, l) = line.split_at("Operation: new = old  ".len() + 1);
                let (o, n) = split_line(l, " ");
                oper = o.trim().to_string();
                num = n.trim().to_string();
            } else if line.contains("Test") {
                let (_f, l) = split_line(line.as_str(), "Test: divisible by ");
                divisible = l.trim().parse::<i32>().unwrap();
            } else if line.contains("If true") {
                let (_f, l) = split_line(line.as_str(), "If true: throw to monkey   ");
                true_throw = l.trim().parse::<i32>().unwrap();
            } else if line.contains("If false") {
                let (_f, l) = split_line(line.as_str(), "If false: throw to monkey   ");
                false_throw = l.trim().parse::<i32>().unwrap();
            }
        }
    }

    monkeys
}

fn split_line<'a>(line: &'a str, txt: &'a str) -> (&'a str, &'a str) {
    line.split_at(txt.len() + 1)
}
