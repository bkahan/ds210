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
    use std::net::Shutdown::Read;
    use crate::graph::graph;
    use csv;
    use csv::Reader;

    fn file2node_helper(csv_entry: Vec<&str>, ) {

        for str in csv_entry {
            let mut tmp = String::new();
            tmp = str.clone().parse().unwrap();



        }



    }

    pub fn file2node(filepath: &str) -> Result<Vec<graph::NodeData>, Box<dyn std::error::Error>> { // adapted from my hw9
        let mut reader = Reader::from_path(filepath)?;
        let mut tmp_res: Vec<graph::NodeData> = Vec::new();

        for res in reader.records() {
            let record = res?;
            let mut actors : Vec<String> = Vec::new();
            let csv_entry: Vec<&str> = record.get(3).unwrap().split(',').collect();
            for str in csv_entry {
                let mut tmp = String::new();
                tmp = str.clone().parse().unwrap();
                actors.push(tmp);
            }
            tmp_res.push(graph::NodeData {
                node_id: 0,
                movie_title: record.get(0).unwrap().parse().unwrap(),
                year: record.get(1).unwrap().parse().unwrap(),
                director: record.get(2).unwrap().parse().unwrap(),
                main_actors: actors,
                rating: record.get(4).unwrap().parse().unwrap(),
                total_gross: record.get(7).unwrap().parse().unwrap(), // todo FIX THIS
                genres: (record.get(8).unwrap().parse().unwrap(), record.get(9).unwrap().parse().unwrap())
            })
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