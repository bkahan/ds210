/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod read_csv { // todo: modify for new csv file

    use std::collections::hash_map::DefaultHasher;
    use crate::graph::graph;
    use csv::Reader;
    use std::hash::{Hash, Hasher};

    pub fn calculate_id<T: Hash>(movie_title: &T) -> i16 { // adapted from https://doc.rust-lang.org/std/hash/index.html example
        let mut s = DefaultHasher::new();
        movie_title.hash(&mut s);

        let mut tmp = s.finish() as i16;

        tmp = tmp.abs();

        return tmp;

    }

    pub fn calculate_hash<T: Hash>(to_hash: &T) -> i16 {
        let mut s = DefaultHasher::new();
        to_hash.hash(&mut s);
        let tmp = s.finish() as i16;
        return tmp.abs();

    }

    pub fn file2node(filepath: &str) -> Result<Vec<graph::NodeData>, Box<dyn std::error::Error>> { // adapted from my hw9
        let mut reader_row_count = Reader::from_path(filepath)?; // not sure if there's a way around this
        let mut reader = Reader::from_path(filepath)?;
        let mut tmp_res: Vec<graph::NodeData> = Vec::new();

        let row_count = reader_row_count.records().count() as i16 ;

        for res in reader.records() {

            let record = res?;
            let mut actors: Vec<String> = Vec::new();

            let mut gross_string: String = record.get(7).unwrap().parse().unwrap();
            let gross_unknown = "Gross";

            let csv_entry: Vec<&str> = record.get(3).unwrap().split(',').collect();
            for str in csv_entry {
                let mut tmp = String::new();
                tmp = str.clone().parse().unwrap();
                actors.push(tmp);
            }

            if !gross_string.contains(gross_unknown) { // remove leading $ and trailing M
                gross_string.pop();
                gross_string.remove(0);
            } else {
                gross_string = String::from("-1.0");
            }

            let mut title: String = record.get(0).unwrap().parse().unwrap();
            let id = calculate_id(&mut title);

            tmp_res.push(graph::NodeData {
                // node_index: 0, // make sure that the index is correctly sized
                // node_id: id as usize,
                movie_title: record.get(0).unwrap().parse().unwrap(),
                year: record.get(1).unwrap().parse().unwrap(),
                director: record.get(2).unwrap().parse().unwrap(),
                main_actors: actors,
                // rating: record.get(4).unwrap().parse().unwrap(),
                // total_gross: gross_string.parse().unwrap(),
                genres: (record.get(8).unwrap().parse().unwrap(), record.get(9).unwrap().parse().unwrap()),
            });
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
2: stri
3: vec<str>
4: f16
7: f16
8,9: (str1, str2)

FIND by movie_title


 */


/*
new data structure for graph:
graph is a hashmap (key = individual actor, value = vec<nodedata>)

 */