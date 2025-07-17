mod files;
mod print_out;

use print_out::print_differences;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        if &args[1] != &args[2] {
            println!("--------------------");
            println!("Comparing {} and {}:", &args[1], &args[2]);
            println!("--------------------");
            print_differences(&args[1], &args[2]);
            println!("--------------------");
        } else {
            println!("Trying to compare the same file, exiting");
        }
    } else {
        println!("Program needs 2 files to compare differences");
    }
}
