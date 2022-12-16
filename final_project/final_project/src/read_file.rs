/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod read_csv { // todo: modify for new csv file

    use crate::graph::graph;
    use csv::Reader;

    pub fn file2node(filepath: &str) -> Result<Vec<graph::NodeData>, Box<dyn std::error::Error>> { // adapted from my hw9
        let mut reader = Reader::from_path(filepath)?;
        let mut tmp_res: Vec<graph::NodeData> = Vec::new();

        for res in reader.records() {

            let record = res?;
            let mut actors: Vec<String> = Vec::new();

            let csv_entry: Vec<&str> = record.get(3).unwrap().split(',').collect();
            for str in csv_entry {
                let tmp = str.clone().parse().unwrap();
                actors.push(tmp);
            }

            tmp_res.push(graph::NodeData {
                movie_title: record.get(0).unwrap().parse().unwrap(),
                year: record.get(1).unwrap().parse().unwrap(),
                director: record.get(2).unwrap().parse().unwrap(),
                main_actors: actors,
                genres: (record.get(8).unwrap().parse().unwrap(), record.get(9).unwrap().parse().unwrap()),
            });
        }
        Ok(tmp_res)
    }
}

/*
new data structure for graph:
graph is a hashmap (key = individual actor, value = vec<nodedata>)

 */