mod files;

use files::files::print_differences;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Program needs 2 files to compare differences");
        return;
    } else {
        print_differences(&args[1], &args[2]);
    }
}
