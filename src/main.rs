use clap::{Parser, Subcommand};
use finch::*;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

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

    #[command(about = "IN PROGRESS: Convert DTOL for Cactus, compare against NCBI dataset")]
    DTOLToCactus { path: String },

    #[command(
        about = "Find contamination in NCBI dataset. Finds assembly-level contamination, i.e., misclassified"
    )]
    NCBIAssemblyCompare { file: String, data_path: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::NcbiToCactus { file, data_path } => {
            ncbi_data_assembly_report_to_cactus(file.to_string(), data_path.to_string());
        }
        Commands::DTOLToCactus { path } => {
            dtol_to_cactus(path.to_string());
        }
        Commands::NCBIAssemblyCompare { file, data_path } => ncbi_assembly_compare(file, data_path),
    }
}

fn ncbi_assembly_compare(file: &str, data_path: &str) {
    // For each pair of genomes, run finch dist.
    // We want a matrix of all the distances

    let mut genomes: Vec<(String, String)> = Vec::new();

    let file = std::fs::File::open(file).unwrap();
    let reader = std::io::BufReader::new(file);
    let json = serde_json::Deserializer::from_reader(reader).into_iter::<Root>();
    for assembly_data_report in json {
        let assembly_data_report = assembly_data_report.unwrap();
        let name = sanitize_name(&assembly_data_report.organism.organism_name);
        let filename = ncbi_get_assembly_path(&data_path, &assembly_data_report);
        genomes.push((name, filename));
    }

    // Sketch all files and save as ".sketch" file
    let all_files = genomes.iter().map(|x| x.1.as_str()).collect::<Vec<&str>>();

    // Check that each file exists
    let mut abort_error = false;
    for file in &all_files {
        if !std::path::Path::new(file).exists() {
            println!("File {} does not exist", file);
            abort_error = true;
        }
    }

    if abort_error {
        std::process::exit(1);
    }

    let sketch_params = finch::sketch_schemes::SketchParams::default();
    let filter_params = finch::filtering::FilterParams::default();

    println!("Sketching {} files", all_files.len());

    let result = finch::sketch_files(&all_files, &sketch_params, &filter_params);
    let sketches = match result {
        Ok(sketches) => sketches,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Comparing {} sketches - {} Total Pairwise Comparisons", sketches.len(), sketches.len() * (sketches.len() - 1));

    // TODO: contamination matrix is invalid name
    // Writer for comparison matrix - Output file called "contamination_matrix.tsv"
    let writer = std::fs::File::create("contamination_matrix.tsv").unwrap();
    let mut writer = std::io::BufWriter::new(writer);

    // For each pairwise sketch
    for i in 0..sketches.len() {

        // Write the genome name
        let genome_name = &genomes[i].0;
        writer.write(genome_name.as_bytes()).unwrap();
        writer.write("\t".as_bytes()).unwrap();

        for j in 0..sketches.len() {
            
            if i == j {
                writer.write("0\t".as_bytes()).unwrap();
                continue;
            }

            let sketch1 = &sketches[i];
            let sketch2 = &sketches[j];

            let result = finch::distance::distance(sketch1, sketch2, false);
            match result {
                Ok(dist) => {
                    println!("{} {} {}", genomes[i].0, genomes[j].0, dist.mash_distance);
                    writer.write(format!("{}\t", dist.mash_distance).as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        writer.write("\n".as_bytes()).unwrap();
    }    
}

fn dtol_to_cactus(path: String) {
    // Get list of all files at the path
    let files = std::fs::read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let path = path.to_str().unwrap();

        println!("Opening file: {}", path);

        // Open the file and read the first line only
        let fh = std::fs::File::open(path).unwrap();
        let reader = GzDecoder::new(fh);
        let reader = std::io::BufReader::new(reader);
        let line = reader.lines().next().unwrap().unwrap();
        println!("Line: {}", line);

        // TODO: Parse the first line of the JSON File
        // Check if the species exists in an NCBI Dataset file (optional)
        // Also check the GCA_ name if it exists in the JSON file
        // If it does, ignore it, we don't wanna double up on the genomes
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
    name.to_string()
        .replace(" ", "_")
        .replace(".", "")
        .replace("-", "_")
}

fn ncbi_get_assembly_path(root_directory: &str, root: &Root) -> String {
    // Names look like: data/GCA_022816945.1/GCA_022816945.1_ASM2281694v1_genomic.fna
    // This is from root.accession and root.assemblyInfo.assemblyName
    format!(
        "{}/data/{}/{}_{}_genomic.fna",
        root_directory,
        root.accession,
        root.accession,
        root.assembly_info.assembly_name.replace(" ", "_")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_name() {
        assert_eq!(
            sanitize_name(&"Trissoscelio sp. ZL-2020".to_string()),
            "Trissoscelio_sp_ZL_2020".to_string()
        );
    }
}
