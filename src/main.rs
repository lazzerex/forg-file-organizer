use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod organizer;
mod config;
mod utils;

use organizer::FileOrganizer;
use config::Config;

#[derive(Parser)]
#[command(name = "forg")]
#[command(about = "A CLI tool to organize files by type, date, or custom rules")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// organize files by extension into folders
    ByType {
        /// source directory to organize
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// target directory (creates subfolders here maybe same as source)
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// preview changes without executing 
        #[arg(short, long)]
        dry_run: bool,
    },
    /// organize files by creation/modification date
    ByDate {
        /// source directory to organize
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// target directory
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// date format: year, month, or day
        #[arg(short, long, default_value = "month")]
        format: String,
        /// preview changes without executing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// clean up empty directories
    Clean {
        /// directory to clean
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
        /// preview changes without executing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// show file statistics for a directory
    Stats {
        /// directory to analyze
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
    },
    /// generate a default configuration file
    Config {
        /// output path for config file
        #[arg(short, long, default_value = "forg-config.json")]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::ByType { source, target, dry_run } => {
            let target = target.unwrap_or_else(|| source.clone());
            let organizer = FileOrganizer::new(source, target);
            
            if dry_run {
                println!("{}", "[DRY RUN] No files will be moved".yellow().bold());
            }
            
            organizer.organize_by_type(dry_run)?;
        }
        Commands::ByDate { source, target, format, dry_run } => {
            let target = target.unwrap_or_else(|| source.clone());
            let organizer = FileOrganizer::new(source, target);
            
            if dry_run {
                println!("{}", "[DRY RUN] No files will be moved".yellow().bold());
            }
            
            organizer.organize_by_date(&format, dry_run)?;
        }
        Commands::Clean { directory, dry_run } => {
            let organizer = FileOrganizer::new(directory.clone(), directory);
            
            if dry_run {
                println!("{}", "[DRY RUN] No directories will be removed".yellow().bold());
            }
            
            organizer.clean_empty_dirs(dry_run)?;
        }
        Commands::Stats { directory } => {
            let organizer = FileOrganizer::new(directory.clone(), directory);
            organizer.show_stats()?;
        }
        Commands::Config { output } => {
            Config::generate_default(&output)?;
            println!("{} Generated config file: {}", "[SUCCESS]".green(), output.display());
        }
    }
    
    Ok(())
}