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

impl LocationIDs {
    fn new() -> LocationIDs {
        let location_ids = LocationIDs {
            file_name: PathBuf::from("input.txt"),
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


fn main() {
    let mut loc = LocationIDs::new();
    loc.open_input_file();

    let mut ids_left = loc.ids_l;
    let mut ids_right = loc.ids_r;

    ids_left.sort();
    ids_right.sort();

    let mut distances = Distances {
        distance_sum: 0,
        buffer: vec![],
    };

    for (index, id) in ids_left.iter().enumerate() {
        let right_id = &ids_right[index];
        let distance = id.abs_diff(*right_id);
        distances.buffer.push(distance);
    }

    distances.distance_sum = distances.buffer.iter().sum();

    println!("Distance Sum: {}", distances.distance_sum);

}
