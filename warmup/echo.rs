use std::{os};

fn main() {
  let args: ~[~str] = os::args();
  let mut i = args.len();
  while i > 1 {
    println(args[i]);
    i-=1;
  }
}
