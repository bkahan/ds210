/*
Ben Kahan
Homework 9
DS210
16 November 2022
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
            let mut split_lines = tmp.split(' ');
            let res = (
                split_lines.next().unwrap().parse::<i32>().unwrap(),
                split_lines.next().unwrap().parse::<i32>().unwrap(),
            );
            tmp_res.push(res);
        }
        Ok(tmp_res)
    }
}
