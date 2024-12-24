use core::fmt;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::collections::HashMap;

use super::file::read_input;

#[derive(Clone)]
struct Operation {
    no: i8,
    items: Vec<BigUint>,
    operator: String,
    subfix: String,
    divisible: BigUint,
    true_throw: i8,
    false_throw: i8,
    inspected: HashMap<u128, u128>,
    time: u128,
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

    let mut lcm: BigUint = BigUint::from_u32(1).unwrap();
    for x in 0..operations.len() {
        lcm *= &operations[x].divisible;
    }

    loop {
        for i in 0..operations.len() {
            for _j in 0..operations[i].items.len() {
                let a: BigUint = operate(
                    &operations[i].items[0],
                    operations[i].operator.clone(),
                    operations[i].subfix.clone(),
                );
                // Comment for part 2
                // let b = a / 3;
                let b = a;
                let zero = BigUint::from_u32(0).unwrap();
                let is_inspected = &b % &operations[i].divisible == zero;
                let t = if is_inspected {
                    operations[i].true_throw
                } else {
                    operations[i].false_throw
                };

                let n = &b % &lcm;

                // ถ้า lcm เกิดจากการคูณของ divisor ค่าของ item ที่ได้จากการ mod ด้วย lcm ต้องเท่ากับค่าที่ได้จากการ item mod ด้วย divisor
                if b != n {
                    println!("b != n: {} {} = {}", b, n, b != n);
                    println!("items = {:?}", operations[i].items[0]);
                    println!("operator = {}", operations[i].operator);
                    println!("subfix = {}", operations[i].subfix);
                    println!("lcm = {}", lcm);
                    println!("divisible = {}", operations[i].divisible);
                    if count == 20 {
                        panic!("exit");
                    }
                }

                operations[i].time += 1;
                operations[i].items[0] = &b % &lcm;

                throw_to_monkey(&mut operations, t, n);
                operations[i].items.remove(0);
            }
        }

        // for i in 0..operations.len() {
        // 	if operations[i].items.len() > 0 {
        // 		operations[i].time += operations[i].items.len() as u128;
        // 	}
        // }

        println!("round: {}", count);

        count += 1;
        if count == 10000 {
            break;
        }
    }
    //     println!("{:?}", operations);
    //     for op in operations.iter() {
    //         println!("{:?}", op.time);
    //     }
    let mut times: Vec<u128> = operations.iter().map(|op| op.time).collect();
    times.sort_by(|a, b| b.cmp(a));
    let top_2_times = &times[..2];
    println!("Top 2 max times: {:?}", top_2_times);
    let result = top_2_times.iter().product::<u128>();
    println!("Product of top 2 max times: {}", result);
}

//347 * 345 = 119415
//345

fn throw_to_monkey(o: &mut Vec<Operation>, to: i8, v: BigUint) {
    // println!("throw to monkey: {} {} {}", to, v, o.len());
    for op in o.iter_mut() {
        if op.no == to {
            //     op.inspected.insert(v, v);
            op.items.push(v);
            //     if is_inspected {
            //     op.time += 1;
            //     }
            break;
        }
    }
}

fn operate(i: &BigUint, o: String, n: String) -> BigUint {
    //     println!("{} {} {}", i, o, n);
    match o.as_str() {
        "+" => i + check_old_val(i, n),
        "-" => i - check_old_val(i, n),
        "*" => i * check_old_val(i, n),
        "/" => i / check_old_val(i, n),
        _ => panic!("Invalid operator"),
    }
}

fn check_old_val(i: &BigUint, n: String) -> BigUint {
    if n == "old" {
        i.clone()
    } else {
        BigUint::parse_bytes(n.as_bytes(), 10).unwrap()
        // n.parse::<u128>().unwrap()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Operation> {
    let mut monkeys = Vec::<Operation>::new();

    let mut no: i8 = -1;
    let mut items = Vec::<BigUint>::new();
    let mut oper = String::from("");
    let mut num = String::from("");
    let mut divisible = BigUint::from_u32(1).unwrap();
    let mut true_throw = 0;
    let mut false_throw = 0;

    for line in input {
        if line == "" {
            monkeys.push(Operation {
                no,
                items: items.clone(),
                operator: oper.clone(),
                subfix: num.clone(),
                divisible: divisible.clone(),
                true_throw,
                false_throw,
                inspected: HashMap::<u128, u128>::new(),
                time: 0,
            });
        } else {
            if line.contains("Monkey") {
                let (_f, l) = split_line(line.as_str(), "Monkey");
                no = l[..l.len() - 1].trim().parse::<i8>().unwrap();
            } else if line.contains("Starting") {
                let (_f, l) = split_line(line.as_str(), "Starting items: ");
                items = l
                    .split(",")
                    .map(|x| BigUint::parse_bytes(x.trim().as_bytes(), 10).unwrap())
                    .collect();
            } else if line.contains("Operation") {
                let (_f, l) = line.split_at("Operation: new = old  ".len() + 1);
                let (o, n) = split_line(l, " ");
                oper = o.trim().to_string();
                num = n.trim().to_string();
            } else if line.contains("Test") {
                let (_f, l) = split_line(line.as_str(), "Test: divisible by ");
                divisible = BigUint::parse_bytes(l.trim().as_bytes(), 10).unwrap()
                // divisible = l.trim().parse::<u128>().unwrap();
            } else if line.contains("If true") {
                let (_f, l) = split_line(line.as_str(), "If true: throw to monkey   ");
                true_throw = l.trim().parse::<i8>().unwrap();
            } else if line.contains("If false") {
                let (_f, l) = split_line(line.as_str(), "If false: throw to monkey   ");
                false_throw = l.trim().parse::<i8>().unwrap();
            }
        }
    }

    monkeys
}

fn split_line<'a>(line: &'a str, txt: &'a str) -> (&'a str, &'a str) {
    line.split_at(txt.len() + 1)
}
