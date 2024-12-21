use rust_playground::d;

fn main() {
    // d::d9::run();
    d::d11::run();

}


// use std::fmt;

// trait Bag {
//     fn display(&self);
// }

// struct Inventory<T> {
//     item: T,
// }

// impl<T> Bag for Inventory<T>
// where
//     T: fmt::Display,
// {
//     fn display(&self) {
//         println!("{}", self.item);
//     }
// }

// // struct Item {
// //     name: String,
// //     amount: i32,
// // }

// fn main() {
//     let i = Inventory {
//         item: 10,
//     };
//     i.display();
// }

// // fn main() {
// //     let s1 = "hell";
// //     let s;
// //     {
// //         let s2 = "world";
// //         s = longest(s1, s2);
// //         println!("The longest string is {}", s);
// //     }

// //     println!("The longest string is {}", s);

// //     let a = 5;
// //     let c;
// //     {
// //         let b = 10;
// //         c = &b
// //     }
// //     println!("c: {}", c);
// // }

// // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// //     if x.len() > y.len() {
// //         x
// //     } else {
// //         y
// //     }
// // }
