use colored::Colorize;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub fn print_differences(file_1: &str, file_2: &str) {
    let differing: Vec<BTreeMap<u32, String>> = crate::files::iterate_through_lines(file_1, file_2);
    print_in_sequence(differing);
}

fn print_in_sequence(differing: Vec<BTreeMap<u32, String>>) {
    let turn = Arc::new(Mutex::new(0));
    let turn_clone = turn.clone();

    // May not be the most memory efficient way to do it but it works
    let differing_clone = differing.clone();

    let handle = thread::spawn(move || {
        filter_printing(&differing_clone[0], turn_clone, 0, "green");
    });
    filter_printing(&differing[1], turn, 1, "red");
    handle.join().unwrap();
}

fn filter_printing(
    differing_lines: &BTreeMap<u32, String>,
    turn: Arc<Mutex<i32>>,
    turn_num: i32,
    color: &str,
) {
    // TODO make an optional option to print out start - end segments
    let keys: Vec<_> = differing_lines.keys().collect();
    let mut sequences: Vec<String> = Vec::new();

    for i in 0..keys.len() {
        // Wait for the turn
        loop {
            let current_turn = *turn.lock().unwrap();
            if current_turn == turn_num {
                break;
            }
            thread::sleep(time::Duration::from_millis(10));
        }

        let key = keys[i];
        let value = differing_lines.get(key);
        let value_str = value.as_ref().map(|s| s.as_str()).unwrap();

        let to_insert = format!("{}\t{}", key, value_str);
        sequences.push(to_insert);

        if i != keys.len() - 1 {
            // If it is equal we only push into the vec and wait for when it isn't (or for the last line)
            // This way we print those in sequences where they differ
            if *keys[i] != *keys[i + 1] - 1 {
                for line in &sequences {
                    println!("{}", line.color(color));
                }
                sequences.clear(); // The next sequence will start at index 0
                if turn_num == 1 {
                    // This ensures there's a space between sequences
                    println!();
                }
                *turn.lock().unwrap() = if turn_num == 0 { 1 } else { 0 };
                thread::sleep(time::Duration::from_millis(50));
            }
        } else {
            // We want to always print the last differing line(s)
            for line in &sequences {
                println!("{}", line.color(color));
            }
            *turn.lock().unwrap() = if turn_num == 0 { 1 } else { 0 };
        }
    }
}
