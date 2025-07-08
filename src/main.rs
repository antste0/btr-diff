fn main() {
    println!("Hello, world!");
}

mod files {
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};
    use std::collections::BTreeMap;

    fn get_lines(filename_1: &str, filename_2: &str) -> io::Result<(Vec<String>, Vec<String>)> {
        // filename_1 and filename_2 are paths, named like this due to lack of better name
        
        let file_1 = File::open(filename_1)?;
        let reader_1 = BufReader::new(file_1);

        let file_2 = File::open(filename_2)?;
        let reader_2 = BufReader::new(file_2);

        let lines_1: Vec<String> = reader_1.lines()
        .filter_map(Result::ok)
        .collect();

        let lines_2: Vec<String> = reader_2.lines()
        .filter_map(Result::ok)
        .collect();

        Ok((lines_1, lines_2))
    }


    fn iterate_through_lines(filename_1: &str, filename_2: &str) -> Vec<BTreeMap<i32, String>> {
        match get_lines(filename_1, filename_2) {
            Ok((lines_1, lines_2)) => {
                let max_line_number = lines_1.len().min(lines_2.len());

                let mut result: Vec<BTreeMap<i32, String>> = vec![BTreeMap::new(), BTreeMap::new()];

                for i in 0..max_line_number {
                    if is_line_different(&lines_1[i], &lines_2[i]) {
                        result[0].insert((i + 1) as i32, lines_1[i].clone());
                        result[1].insert((i + 1) as i32, lines_2[i].clone());
                    }
                }

                return result;
            }
            Err(e) => {
                panic!("Error reading files: {}", e);
            }
        }
    }

    fn is_line_different(s_1: &str, s_2: &str) -> bool {
        if s_1 == s_2 {
            return false;
        }
        return true;
    }
}
