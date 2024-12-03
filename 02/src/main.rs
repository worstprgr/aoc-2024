use core::panic;
use std::path::PathBuf;
use std::fs;


const MIN_TOLERANCE: u32 = 1;
const MAX_TOLERANCE: u32 = 3;

#[derive(Debug, PartialEq)]
enum SafeStatus {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
enum Flow {
    Increasing,
    Decreasing,
    Init,
}


#[derive(Debug)]
struct SafetyReport {
    file_name: PathBuf,
    safe_reports: usize,
    reports: Vec<Vec<u32>>,
    flow_state: Flow,
}

impl SafetyReport {
    fn new(file_name: &str) -> SafetyReport {
        let safety_reporter = SafetyReport {
            file_name: PathBuf::from(file_name),
            safe_reports: 0,
            reports: vec![],
            flow_state: Flow::Init,
        };
        safety_reporter
    }

    fn open_input_file(&mut self) -> &Self {
        let content = fs::read(&self.file_name).unwrap();
        let nums = String::from_utf8(content).unwrap();
        let reports: Vec<&str> = nums.lines().collect();

        for line in &reports {
            let parsed: Vec<u32> = line.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            self.reports.push(parsed);
        }
        self
    }

    fn analyze_reports(&mut self) -> &Self {
        let reports = self.reports.clone();
        for report in reports {
            if report[0] > report[1] {
                self.flow_state = Flow::Decreasing;
            } else {
                self.flow_state = Flow::Increasing;
            }
            let grad = self.analyze_gradient(&report);
            match grad {
                SafeStatus::Safe => self.safe_reports += 1,
                _ => ()
            }
        }
        self
    }

    fn analyze_gradient(&mut self, report: &Vec<u32>) -> SafeStatus {
        for (prev, curr) in report.iter().zip(report.iter().skip(1)) {
            match self.flow_state {
                Flow::Decreasing => {
                    if prev < curr {
                        return SafeStatus::Unsafe
                    }
                    let diff = curr.abs_diff(*prev);
                    if MIN_TOLERANCE > diff || diff > MAX_TOLERANCE {
                        return SafeStatus::Unsafe
                    }
                },
                Flow::Increasing => {
                    if prev > curr {
                        return SafeStatus::Unsafe
                    }
                    let diff = curr.abs_diff(*prev);
                    if MIN_TOLERANCE > diff || diff > MAX_TOLERANCE {
                        return SafeStatus::Unsafe
                    }
                },
                Flow::Init => {
                    panic!("Flow State: Init");
                }
            }
        }
        SafeStatus::Safe
    }
}



fn main() {
    let mut sr = SafetyReport::new("input.txt");
    sr.open_input_file();
    sr.analyze_reports();

    println!("Reports Safe: {}", sr.safe_reports);
}
