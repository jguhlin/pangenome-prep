use serde::{Deserialize, Serialize};

mod ncbi_data_assembly_report_serde;
use ncbi_data_assembly_report_serde::*;

fn main() {
    // Open the file
    let file = std::fs::File::open("assembly_data_report.jsonl").unwrap();
    let reader = std::io::BufReader::new(file);
    let json = serde_json::Deserializer::from_reader(reader).into_iter::<Root>();
    for assembly_data_report in json {
        let assembly_data_report = assembly_data_report.unwrap();
        println!("{:?}", assembly_data_report);
    }
}
