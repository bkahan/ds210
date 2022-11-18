/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

use std::fs;

pub mod read_file {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn file2vectuple(filepath: &str) -> Result<Vec<(u32, u32)>, Box<dyn std::error::Error>> {
        let data = File::open(filepath)?;
        let reader = BufReader::new(data);

        let mut tmp_res: Vec<(u32, u32)> = Vec::new();

        for line in reader.lines() {
            let tmp = line.unwrap();
            let mut split_lines = tmp.split(' ');
            let res = (
                split_lines.next().unwrap().parse::<u32>().unwrap(),
                split_lines.next().unwrap().parse::<u32>().unwrap(),
            );
            tmp_res.push(res);
        }

        Ok(tmp_res)
    }
}
