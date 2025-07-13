pub mod files {
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};
    use std::collections::BTreeMap;

    fn get_lines(filename_1: &str, filename_2: &str) -> io::Result<(Vec<String>, Vec<String>)> {
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

    fn iterate_through_lines(filename_1: &str, filename_2: &str) -> Vec<BTreeMap<u32, String>> {
        match get_lines(filename_1, filename_2) {
            Ok((lines_1, lines_2)) => {
                let longer_file: u8;
                let max_line_number: usize;

                if lines_1.len() < lines_2.len() {
                    max_line_number = lines_1.len();
                    longer_file = 2;
                } else {
                    max_line_number = lines_2.len();
                    longer_file = 1;
                }

                let mut differing: Vec<BTreeMap<u32, String>> = vec![BTreeMap::new(), BTreeMap::new()];

                for i in 0..max_line_number {
                    if is_line_different(&lines_1[i], &lines_2[i]) {
                        differing[0].insert((i + 1) as u32, lines_1[i].clone());
                        differing[1].insert((i + 1) as u32, lines_2[i].clone());
                    }
                }

                if longer_file == 1 {
                    insert_the_remaining_lines(&lines_1, lines_1.len(), &mut differing[1]);
                } else if longer_file == 2 && lines_2.len() > lines_1.len() {
                    insert_the_remaining_lines(&lines_2, lines_2.len(), &mut differing[0]);
                }


                return differing;
            }
            Err(e) => {
                panic!("Error reading files: {}", e);
            }
        }
    }

    fn is_line_different(s_1: &str, s_2: &str) -> bool {
        s_1 != s_2
    }

    // All the lines from the file with more lines differ therefore we need to add them to the map
    fn insert_the_remaining_lines(lines: &[String], start_from: usize, map: &mut BTreeMap<u32, String>) {
        for line_num in start_from..lines.len() {
            map.insert((line_num + 1) as u32, lines[line_num].clone());
        }
    }

    pub fn print_differences(file_1: &str, file_2: &str) {
        let differing: Vec<BTreeMap<u32, String>> = iterate_through_lines(file_1, file_2);
    }
}
