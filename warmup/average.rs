use std::{os, float, uint};

fn main() {
  let args = os::args();
  
  if args.len() <= 1 {
    println("Must provide at least one argument");
    return
  }

  let mut count = 0;
  let mut sum: float = 0.0;
  for uint::range(1,args.len()) |i| {
    match float::from_str(args[i]) {
      Some(num) => {sum+=num; count+=1;}
      None => {println(fmt!("Bad Input: %s",args[i]));}
    }
  }
  println(fmt!("Average: %f", sum/(count as float) ));
}
