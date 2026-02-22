mod basic;
mod ownership;

use crate::basic::datatypes::data_types;
use crate::basic::functions::functions;
use crate::basic::variables::variable;
use crate::basic::control_flow::control_flow;

fn main() {
   //  println!("Hello, world!");
   //  let mut inputA = String::new();
   //  let mut inputB = String::new();
   //
   // let mut c: i8 = 0;
   //  println!("Please input your Number 1");
   //  io::stdin().read_line(&mut inputA).expect("Failed to read line");
   //  println!("Please input your Number 2");
   //  io::stdin().read_line(&mut inputB).expect("Failed to read line");
   //
   //    let a =   match inputA.trim().parse::<i32>() {
   //          Ok(n) => {n},
   //          Err(_) => {
   //              return;
   //          }
   //      };
   //  let b = match inputB.trim().parse::<i32>() {
   //      Ok(n) => n,
   //      Err(_) => {
   //             return;
   //      }
   //  };
   //
   //  println!("You entered a: {0}", a);
   //  println!("You entered b: {0}", b);

    variable();
    data_types();
    functions();
    control_flow();
}
