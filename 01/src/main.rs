use core::panic;
use std::path::PathBuf;
use std::fs;


#[derive(Debug)]
struct LocationIDs {
    file_name: PathBuf,
    ids_r: Vec<u32>,
    ids_l: Vec<u32>,
}

#[derive(Debug)]
struct Distances {
    distance_sum: u32,
    buffer: Vec<u32>,
}

#[derive(Debug)]
struct SimilarityScore {
    similarity_sum: u32,
    buffer: Vec<u32>,
}


impl LocationIDs {
    fn new(file_name: &str) -> LocationIDs {
        let location_ids = LocationIDs {
            file_name: PathBuf::from(file_name),
            ids_r: vec![],
            ids_l: vec![],
        };
        location_ids
    }

    fn open_input_file(&mut self) -> &Self {
        let content = fs::read(&self.file_name).unwrap();
        let nums = String::from_utf8(content).unwrap();
        let line_of_nums: Vec<&str> = nums.lines().collect();

        for line in &line_of_nums {
            let parsed: Vec<u32> = line.split("   ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            let left = match parsed.get(0) {
                Some(v) => v,
                None => {
                    panic!("Cannot parse line: {:?}\tParse: {:?}", line, parsed);
                }
            };

            let right = match parsed.get(1) {
                Some(v) => v,
                None => {
                    panic!("Cannot parse line: {:?}\tParse: {:?}", line, parsed);
                }
            };

            self.ids_l.push(*left);
            self.ids_r.push(*right);
        }
        self
    }
}


impl Distances {
    fn new() -> Distances {
        let distances = Distances {
            distance_sum: 0,
            buffer: vec![],
        };
        distances
    }

    fn get_distance(&mut self, location_ids: &mut LocationIDs) -> &Self {
        let mut ids_left = location_ids.ids_l.clone();
        ids_left.sort();

        let mut ids_right = location_ids.ids_r.clone();
        ids_right.sort();

        for (index, id) in ids_left.iter().enumerate() {
            let right_id = &ids_right[index];
            let distance = id.abs_diff(*right_id);
            self.buffer.push(distance);
        }

        self.distance_sum = self.buffer.iter().sum();
        self
    }
}


impl SimilarityScore {
    fn new() -> SimilarityScore {
        let similarity_score = SimilarityScore {
            similarity_sum: 0,
            buffer: vec![],
        };
        similarity_score
    }

    fn calc_score(&mut self, location_ids: &LocationIDs) -> &Self {
        let ids_left = location_ids.ids_l.clone();
        let ids_right = location_ids.ids_r.clone();

        for id in ids_left {
            let occurence = ids_right.iter().filter(|&v| *v == id).count();
            let score = id * occurence as u32;
            self.buffer.push(score);
        }
        self.similarity_sum = self.buffer.iter().sum();
        self
    }
}


fn main() {
    let mut loc = LocationIDs::new("input.txt");
    loc.open_input_file();

    let mut distances = Distances::new();
    distances.get_distance(&mut loc);

    let mut similarity_score = SimilarityScore::new();
    similarity_score.calc_score(&loc);

    println!("Distance Sum: {}", distances.distance_sum);
    println!("Similarity Score: {}", similarity_score.similarity_sum);
}
