use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub accession: String,
    pub assembly_info: AssemblyInfo,
    pub assembly_stats: AssemblyStats,
    pub current_accession: String,
    pub organelle_info: Vec<OrganelleInfo>,
    pub organism: Organism2,
    pub source_database: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyInfo {
    pub assembly_level: String,
    pub assembly_method: String,
    pub assembly_name: String,
    pub assembly_status: String,
    pub assembly_type: String,
    pub bioproject_accession: String,
    pub bioproject_lineage: Vec<BioprojectLineage>,
    pub biosample: Biosample,
    pub blast_url: String,
    pub comments: String,
    pub diploid_role: String,
    pub linked_assemblies: Vec<LinkedAssembly>,
    pub refseq_category: String,
    pub release_date: String,
    pub sequencing_tech: String,
    pub submitter: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BioprojectLineage {
    pub bioprojects: Vec<Bioproject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bioproject {
    pub accession: String,
    #[serde(default)]
    pub parent_accessions: Vec<String>,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biosample {
    pub accession: String,
    pub attributes: Vec<Attribute>,
    pub description: Description,
    pub last_updated: String,
    pub models: Vec<String>,
    pub owner: Owner,
    pub package: String,
    pub publication_date: String,
    pub sample_ids: Vec<SampleId>,
    pub status: Status,
    pub submission_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub organism: Organism,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organism {
    pub organism_name: String,
    pub tax_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleId {
    pub db: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub status: String,
    pub when: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedAssembly {
    pub assembly_type: String,
    pub linked_assembly: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyStats {
    pub contig_l50: i64,
    pub contig_n50: i64,
    pub gc_count: String,
    pub gc_percent: f64,
    pub genome_coverage: String,
    pub number_of_component_sequences: i64,
    pub number_of_contigs: i64,
    pub number_of_organelles: i64,
    pub number_of_scaffolds: i64,
    pub scaffold_l50: i64,
    pub scaffold_n50: i64,
    pub total_number_of_chromosomes: i64,
    pub total_sequence_length: String,
    pub total_ungapped_length: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganelleInfo {
    pub description: String,
    pub submitter: String,
    pub total_seq_length: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organism2 {
    pub organism_name: String,
    pub tax_id: i64,
}
