/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod read_csv { // todo: modify for new csv file

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

/*

header of csv data:

Movie_Title 0,Year 1, Director 2,Actors 3,Rating 4,Runtime(Mins) 5,Censor 6,Total_Gross 7,main_genre 8,side_genre 9

do not need:
runtime (5)
censor (6)

we want:
line[0], line[1], line[2], line[3], line[4], line[7], line[8], line[9]

data types:
0: str
1: u16
2: str
3: vec<str>
4: f16
7: f16
8,9: (str1, str2)

FIND by movie_title


 */