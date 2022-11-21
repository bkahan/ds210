/*
Ben Kahan
Homework 10
DS210
20 November 2022
Collaborators: none
*/

pub(crate) mod read_file {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn file2vectuple(filepath: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
        let data = File::open(filepath)?;
        let reader = BufReader::new(data);

        let mut tmp_res: Vec<(i32, i32)> = Vec::new();

        for line in reader.lines() {
            let tmp = line.unwrap();
            let mut split_lines: Vec<&str> = tmp.split(' ').collect();

            let mut res: (i32, i32) = (0, 0);

            if split_lines.len() == 1  { // make the first node be the dimensions todo: make this better, this is shit code
                res.0 = split_lines[0].parse().unwrap();
                res.1 = res.0;
            } else {
                res.0 = split_lines[0].parse().unwrap();
                res.1 = split_lines[1].parse().unwrap();
            }
            tmp_res.push(res);
        }
        Ok(tmp_res)
    }
}