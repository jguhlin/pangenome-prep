use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

mod ncbi_data_assembly_report_serde;
use ncbi_data_assembly_report_serde::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Convert NCBI data assembly report JSONL file to Cactus config")]
    NcbiToCactus { file: String, data_path: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::NcbiToCactus { file, data_path } => {
            ncbi_data_assembly_report_to_cactus(file.to_string(), data_path.to_string());
        }
    }
}

fn ncbi_data_assembly_report_to_cactus(file: String, data_path: String) {
    // Open the file
    // let file = std::fs::File::open("assembly_data_report.jsonl").unwrap();
    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    let json = serde_json::Deserializer::from_reader(reader).into_iter::<Root>();
    for assembly_data_report in json {
        let assembly_data_report = assembly_data_report.unwrap();
        let name = sanitize_name(&assembly_data_report.organism.organism_name);
        let filename = ncbi_get_assembly_path(&data_path, &assembly_data_report);
        println!("{} {}", name, filename);
    }
   
}

fn sanitize_name(name: &str) -> String {
    // Ex: Trissoscelio sp. ZL-2020 must becomes Trissoscelio_sp_ZL2020
    name.to_string().replace(" ", "_").replace(".", "").replace("-", "_")
}

fn ncbi_get_assembly_path(root_directory: &str, root: &Root) -> String {
    // Names look like: data/GCA_022816945.1/GCA_022816945.1_ASM2281694v1_genomic.fna
    // This is from root.accession and root.assemblyInfo.assemblyName
    format!("{}/data/{}/{}_{}_genomic.fna", root_directory, root.accession, root.accession, root.assembly_info.assembly_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name(&"Trissoscelio sp. ZL-2020".to_string()), "Trissoscelio_sp_ZL_2020".to_string());
    }
}