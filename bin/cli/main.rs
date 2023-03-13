use std::error::Error;

use clap::{Parser, Subcommand};
use fsid::{
    adapter::out::{file::create_load_records, mining::create_get_bin_distance},
    application::{
        create_audit_use_case, create_mining_use_case,
        port::incoming::{audit_use_case::AuditCommand, mining_use_case::MiningCommand},
    },
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Audits a dataset
    Audit {
        /// Path to the dataset
        #[arg(long)]
        dataset: String,
        /// Path to the distance matrix
        #[arg(long)]
        distances: Option<String>,
        /// Minimun number of institutions that published a species
        #[arg(long)]
        sources: usize,
        /// Minimun number of records of a species
        #[arg(long)]
        size: usize,
        /// Maximun distance of neighbour bin
        #[arg(long)]
        distance: f64,
    },
    /// Downloads distances of all bin_uri in the dataset
    Mining {
        /// Path to the dataset
        #[arg(long)]
        dataset: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    match &args.command {
        Commands::Audit {
            dataset,
            distances: _,
            sources,
            size,
            distance,
        } => {
            let lrp = create_load_records();
            let gdp = create_get_bin_distance();
            let ads = create_audit_use_case(lrp, gdp);
            let grading = ads.audit(AuditCommand::new(
                dataset.to_string(),
                *sources,
                *size,
                *distance,
            ));
            println!("{grading:?}");
        }
        Commands::Mining { dataset } => {
            let lrp = create_load_records();
            let gdp = create_get_bin_distance();
            let ms = create_mining_use_case(lrp, gdp);
            let mining = ms.mine(MiningCommand::new(dataset.to_string()));
            println!("{mining:?}");
        }
    }

    Ok(())
}
