
use std::{collections::HashMap};

struct Position {
    title: &'static str,
    x: i32,
    y: i32,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Position {{ title: {}, x: {}, y: {} }}",
            self.title, self.x, self.y
        )
    }
}

pub fn run() {
    let mut p: Vec<Position> = vec!["H", "1", "2", "3", "4", "5", "6", "7", "8", "9", "s"]
        .iter()
        .map(|&x| Position {
            title: x,
            x: 0,
            y: 0,
        })
        .collect();

    // // // ......
    // // // ......
    // // // ......
    // // // ......
    // // // 4321H.
    // right_2(&mut p, 4);

    // // // ....H.
    // // // ....1.
    // // // ..432.
    // // // .5....
    // // // 6.....
    // up_2(&mut p, 4);

    // // .H1...
    // // ...2..
    // // ..43..
    // // .5....
    // // 6.....
    // left_2(&mut p, 3);

    // // ..1...
    // // .H.2..
    // // ..43..
    // // .5....
    // // 6.....
    // down_2(&mut p, 1);

    // // ......
    // // ...21H
    // // ..43..
    // // .5....
    // // 6.....  (6 covers 7, 8, 9, s)
    // right_2(&mut p, 4);

    // // ......
    // // ...21.
    // // ..43.H
    // // .5....
    // // 6.....  (6 covers 7, 8, 9, s)
    // down_2(&mut p, 1);

    // // ......
    // // ......
    // // H123..  (2 covers 4)
    // // .5....
    // // 6.....  (6 covers 7, 8, 9, s)
    // left_2(&mut p, 5);

    // // ......
    // // ......
    // // .1H3..  (H covers 2, 4)
    // // .5....
    // // 6.....  (6 covers 7, 8, 9, s)
    // right_2(&mut p, 2);

    // H.........................
    // 1.........................
    // 2.........................
    // 3.........................
    // 4.........................
    // 5.........................
    // 6.........................
    // 7.........................
    // 8.........................
    // 9.........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ...........s..............
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    // ..........................
    let mut result = HashMap::<String, bool>::new();

    // move_2(&mut p, 5, "R", &mut result);
    // move_2(&mut p, 8, "U", &mut result);
    // move_2(&mut p, 8, "L", &mut result);
    // move_2(&mut p, 3, "D", &mut result);
    // move_2(&mut p, 17, "R", &mut result);
    // move_2(&mut p, 10, "D", &mut result);
    // move_2(&mut p, 25, "L", &mut result);
    // move_2(&mut p, 20, "U", &mut result);

    // // right_2(&mut p, 5);
    // // up_2(&mut p, 8);
    // // left_2(&mut p, 8);
    // // down_2(&mut p, 3);
    // // right_2(&mut p, 17);
    // // down_2(&mut p, 10);
    // // left_2(&mut p, 25);
    // // up_2(&mut p, 20);

    // println!("{:?}", p);
    // println!("{:?}", result.len());

    let input = read_input("input/d9.txt").unwrap();
    
    for i in input.iter() {
        let i2 = i.split(" ");
        let t = i2.clone().nth(0).unwrap();
        let n = i2.clone().nth(1).unwrap().parse::<i32>().unwrap();
        move_2(&mut p, n, t, &mut result);
    }

    // println!("{:?}", input.len());
    println!("{:?}", result.len());
    println!("{:?}", p);
    // println!("{:?}", result);
}

fn read_input(file: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = std::fs::read_to_string(file)?;
    let lines = contents.lines().map(|x| x.to_string()).collect();
    Ok(lines)
} 

fn move_position(
    item: &mut Position,
    prv_x: i32,
    prv_y: i32,
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
) -> bool {
    if item.x + x == prv_x && item.y + y == prv_y {
        item.x += v_x;
        item.y += v_y;
        return true;
    }

    return false;
}

fn move_2(p: &mut Vec<Position>, n: i32, move_type: &str, result: &mut HashMap<String, bool>) {
    let mut prv_x = 0;
    let mut prv_y = 0;

    let a = vec![
        // ..........H*x.............
        (-2, 0, -1, 0),
        // ...........x*H............
        (2, 0, 1, 0),
        // ..........H...............
        // ...........*..............
        // ............x.............
        (-2, 2, -1, 1),
        // ............x.............
        // ...........*..............
        // ..........H...............
        (-2, -2, -1, -1),
        // ..............H...........
        // .............*............
        // ............x.............
        (2, 2, 1, 1),
        // ............x.............
        // .............*............
        // ..............H...........
        (2, -2, 1, -1),
        // ..........H*..............
        // ............x.............
        (-2, 1, -1, 1),
        // ............x.............
        // ..........H*..............
        (-2, -1, -1, -1),
        // .............*H...........
        // ............x.............
        (2, 1, 1, 1),
        // ............x.............
        // .............*H...........
        (2, -1, 1, -1),
        // ...........H..............
        // ...........*..............
        // ............x.............
        (-1, 2, -1, 1),
        // .............H............
        // .............*............
        // ............x.............
        (1, 2, 1, 1),
        // ............x.............
        // ...........*..............
        // ...........H..............
        (-1, -2, -1, -1),
        // ............x.............
        // .............*............
        // .............H............
        (1, -2, 1, -1),
        // ............H.............
        // ............*.............
        // ............x.............
        (0, 2, 0, 1),
        // ............x.............
        // ............*.............
        // ............H.............
        (0, -2, 0, -1),
        // ...............H..........
        // ...............*..........
        // ............x.............
        (3, 2, 3, 1),
        // ............x.............
        // ...............*..........
        // ...............H..........
        (3, -2, 3, -1),
        // .........H................
        // .........*................
        // ............x.............
        (-3, 2, -3, 1),
        // ............x.............
        // .........*................
        // .........H................
        (-3, -2, -3, -1),
    ];

    let indexs = match move_type {
        "U" => (0, 1),
        "D" => (0, -1),
        "R" => (1, 0),
        "L" => (-1, 0),
        _ => (0, 1),
    };

    for _ in 0..n {
        for (_, item) in p.iter_mut().enumerate() {
            if item.title == "H" {
                item.x += indexs.0;
                item.y += indexs.1;
            }
            if item.title != "H" {
                for i in a.iter() {
                    if move_position(item, prv_x, prv_y, i.0, i.1, i.2, i.3) {
                        break;
                    }
                }
            }
            prv_x = item.x;
            prv_y = item.y;

            if item.title == "9" {
                result.insert(format!("{},{}", item.x, item.y), true);
            }
        }
    }
}



// // ---------------------------------------------------------

// fn up_2(p: &mut Vec<Position>, n: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
    
// 	let indexs = vec![
// 	    // ............H.............
// 	    // ............*.............
// 	    // ...........x..............
// 	    (1, 2, 1, 1),
// 	    // ..........H...............
// 	    // ..........*...............
// 	    // ...........x..............
// 	    (-1, 2, -1, 1),
// 	    // ...........H..............
// 	    // ...........*..............
// 	    // ...........x..............
// 	    (0, 2, 0, 1),
// 	    // .........H*...............
// 	    // ...........x..............
// 	    (-2, 1, -1, 1),
// 	    // ............*H............
// 	    // ...........x..............
// 	    (2, 1, 1, 1),
// 	    // .............H............
// 	    // ............*.............
// 	    // ...........x..............
// 	    (2, 2, 1, 1),
// 	    // .........H................
// 	    // ..........*...............
// 	    // ...........x..............
// 	    (-2, 2, -1, 1),
// 	    // ..........................
// 	    // ...........H..............
// 	    // ...........x..............
// 	    (0, 1, 0, 0),
// 	];
    
// 	for _ in 0..n {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.y += 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    for i in indexs.iter() {
// 			if move_position(item, prv_x, prv_y, i.0, i.1, i.2, i.3) {
// 			    break;
// 			}
// 		    }
    
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 	    }
// 	}
//     }
    
//     fn down_2(p: &mut Vec<Position>, n: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
    
// 	let indexs = vec![
// 	    // ...........x..............
// 	    // ............*.............
// 	    // ............H.............
// 	    (1, -2, 1, -1),
// 	    // ...........x..............
// 	    // ..........*...............
// 	    // ..........H...............
// 	    (-1, -2, -1, -1),
// 	    // ...........x..............
// 	    // ............*H............
// 	    (2, -1, 1, -1),
// 	    // ...........x..............
// 	    // ...........*..............
// 	    // ...........H..............
// 	    (0, -2, 0, -1),
// 	    // ...........x..............
// 	    // ............*.............
// 	    // .............H............
// 	    (2, -2, 1, -1),
// 	    // ...........x..............
// 	    // .........H*...............
// 	    (-2, -1, -1, -1),
// 	    // ...........x..............
// 	    // ..........*...............
// 	    // .........H................
// 	    (-2, -2, -1, -1),
// 	    // ...........x..............
// 	    // ...........H..............
// 	    (0, -1, 0, 0),
// 	];
    
// 	for _ in 0..n {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.y -= 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    for i in indexs.iter() {
// 			if move_position(item, prv_x, prv_y, i.0, i.1, i.2, i.3) {
// 			    break;
// 			}
// 		    }
    
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 	    }
// 	}
//     }
    
//     fn right_2(p: &mut Vec<Position>, n: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
    
// 	let indexs = vec![
// 	    // ...........x*H............
// 	    (2, 0, 1, 0),
// 	    // ............*H............
// 	    // ...........x..............
// 	    (2, 1, 1, 1),
// 	    // ............H.............
// 	    // ............*.............
// 	    // ...........x..............
// 	    (1, 2, 1, 1),
// 	    // ...........x..............
// 	    // ............*H............
// 	    (2, -1, 1, -1),
// 	    // ...........x..............
// 	    // ............*.............
// 	    // ............H.............
// 	    (1, -2, 1, -1),
// 	    // ...........x..............
// 	    // ............*.............
// 	    // .............H............
// 	    (2, -2, 1, -1),
// 	    // .............H............
// 	    // ............*.............
// 	    // ...........x..............
// 	    (2, 2, 1, 1),
// 	];
    
// 	for _ in 0..n {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.x += 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    for i in indexs.iter() {
// 			if move_position(item, prv_x, prv_y, i.0, i.1, i.2, i.3) {
// 			    break;
// 			}
// 		    }
    
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 	    }
// 	}
//     }
    
//     fn left_2(p: &mut Vec<Position>, n: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
    
// 	let indexs = vec![
// 	    // ..........H*x.............
// 	    (-2, 0, -1, 0),
// 	    // ..........H*..............
// 	    // ............x.............
// 	    (-2, 1, -1, 1),
// 	    // ...........H..............
// 	    // ...........*..............
// 	    // ............x.............
// 	    (-1, 2, -1, 1),
// 	    // ............x.............
// 	    // ...........*..............
// 	    // ...........H..............
// 	    (-1, -2, -1, -1),
// 	    // ............x.............
// 	    // ..........H*..............
// 	    (-2, -1, -1, -1),
// 	    // ............x.............
// 	    // ...........*..............
// 	    // ..........H...............
// 	    (-2, -2, -1, -1),
// 	    // ..........H...............
// 	    // ..........................
// 	    // ............x.............
// 	    (-2, 2, -1, 1),
// 	];
    
// 	for _ in 0..n {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.x -= 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    for i in indexs.iter() {
// 			if move_position(item, prv_x, prv_y, i.0, i.1, i.2, i.3) {
// 			    break;
// 			}
// 		    }
    
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 	    }
// 	}
//     }
    
//     // ---------------------------------------------------------
    
//     fn up(p: &mut Vec<Position>, num: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
// 	for _ in 0..num {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.y += 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    if item.y + 2 == prv_y && item.x + 1 == prv_x {
// 			item.y += 1;
// 			item.x += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y + 2 == prv_y && item.x - 1 == prv_x {
// 			item.y += 1;
// 			item.x -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x + 2 == prv_x && item.y + 1 == prv_y {
// 			item.x += 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y + 2 == prv_y && item.x == prv_x {
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y + 2 == prv_y && item.x + 2 == prv_x {
// 			item.x += 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x - 2 == prv_x && item.y + 1 == prv_y {
// 			item.x -= 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y + 2 == prv_y && item.x - 2 == prv_x {
// 			item.x -= 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else {
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    }
// 		}
// 	    }
// 	}
//     }
    
//     fn right(p: &mut Vec<Position>, num: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
// 	for _ in 0..num {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.x += 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    if item.x + 2 == prv_x && item.y == prv_y {
// 			item.x += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x + 2 == prv_x && item.y + 1 == prv_y {
// 			item.x += 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x + 1 == prv_x && item.y + 2 == prv_y {
// 			item.x += 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x + 2 == prv_x && item.y - 1 == prv_y {
// 			item.x += 1;
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else {
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    }
// 		}
// 	    }
// 	}
//     }
    
//     fn left(p: &mut Vec<Position>, num: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
// 	for _ in 0..num {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.x -= 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    if item.x - 2 == prv_x && item.y == prv_y {
// 			item.x -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x - 2 == prv_x && item.y + 1 == prv_y {
// 			item.x -= 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x - 1 == prv_x && item.y + 2 == prv_y {
// 			item.x -= 1;
// 			item.y += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else {
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    }
// 		}
// 	    }
// 	}
//     }
    
//     fn down(p: &mut Vec<Position>, num: i32) {
// 	let mut prv_x = 0;
// 	let mut prv_y = 0;
// 	for _ in 0..num {
// 	    for (_, item) in p.iter_mut().enumerate() {
// 		if item.title == "H" {
// 		    item.y -= 1;
// 		    prv_x = item.x;
// 		    prv_y = item.y;
// 		}
// 		if item.title != "H" {
// 		    if item.y - 2 == prv_y && item.x + 1 == prv_x {
// 			item.y -= 1;
// 			item.x += 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y - 2 == prv_y && item.x - 1 == prv_x {
// 			item.y -= 1;
// 			item.x -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x + 2 == prv_x && item.y - 1 == prv_y {
// 			item.x += 1;
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y - 2 == prv_y && item.x == prv_x {
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y - 2 == prv_y && item.x + 2 == prv_x {
// 			item.x += 1;
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.x - 2 == prv_x && item.y - 1 == prv_y {
// 			item.x -= 1;
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else if item.y - 2 == prv_y && item.x - 2 == prv_x {
// 			item.x -= 1;
// 			item.y -= 1;
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    } else {
// 			prv_x = item.x;
// 			prv_y = item.y;
// 		    }
// 		}
// 	    }
// 	}
//     }
    
    