/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod read_csv {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn file2vectuple(filepath: &str) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> { // adapted from my hw9
        let data = File::open(filepath)?;
        let reader = BufReader::new(data);

        let mut tmp_res: Vec<(usize, usize)> = Vec::new();

        for line in reader.lines() {
            let tmp = line.unwrap();
            let split_lines: Vec<&str> = tmp.split(',').collect();

            let mut res: (usize, usize) = (0, 0);

            match split_lines.len() { // todo: still kinda shit but better
                1 => {
                    res.0 = split_lines[0].parse().unwrap();
                    res.1 = res.0;
                },
                _ => {
                    res.0 = split_lines[0].parse().unwrap();
                    res.1 = split_lines[1].parse().unwrap();
                }
            }
            tmp_res.push(res);
        }
        Ok(tmp_res)
    }
}