mod files;
mod print_out;

use print_out::print_differences;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Program needs 2 files to compare differences");
    } else if &args[1] == &args[2] {
        println!("Trying to compare the same file")
    } else {
        println!("--------------------");
        println!("Comparing {} and {}:", &args[1], &args[2]);
        println!("--------------------");
        print_differences(&args[1], &args[2]);
        println!("--------------------");
    }
}
