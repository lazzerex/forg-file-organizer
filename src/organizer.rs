use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::{DateTime, Local};
use colored::*;
use anyhow::{Context, Result};

use crate::utils::{get_file_type_folder, format_file_size};

pub struct FileOrganizer {
    source: PathBuf,
    target: PathBuf,
}

impl FileOrganizer {
    pub fn new(source: PathBuf, target: PathBuf) -> Self {
        Self { source, target }
    }

    pub fn organize_by_type(&self, dry_run: bool) -> Result<()> {
        println!("{} Organizing files by type...", "[INFO]".cyan());
        println!("Source: {}", self.source.display());
        println!("Target: {}", self.target.display());
        println!();

        let mut moved_count = 0;
        let mut error_count = 0;

        for entry in WalkDir::new(&self.source)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                match self.move_file_by_type(entry.path(), dry_run) {
                    Ok(moved) => {
                        if moved {
                            moved_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Error processing {}: {}", 
                                "[ERROR]".red(), 
                                entry.path().display(), 
                                e);
                        error_count += 1;
                    }
                }
            }
        }

        println!();
        if dry_run {
            println!("{} Would move {} files", "[PREVIEW]".blue(), moved_count);
        } else {
            println!("{} Moved {} files", "[SUCCESS]".green(), moved_count);
        }
        
        if error_count > 0 {
            println!("{} {} errors occurred", "[WARNING]".yellow(), error_count);
        }

        Ok(())
    }

    pub fn organize_by_date(&self, format: &str, dry_run: bool) -> Result<()> {
        println!("{} Organizing files by date ({})...", "[INFO]".cyan(), format);
        println!("Source: {}", self.source.display());
        println!("Target: {}", self.target.display());
        println!();

        let mut moved_count = 0;
        let mut error_count = 0;

        for entry in WalkDir::new(&self.source)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                match self.move_file_by_date(entry.path(), format, dry_run) {
                    Ok(moved) => {
                        if moved {
                            moved_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Error processing {}: {}", 
                                "[ERROR]".red(), 
                                entry.path().display(), 
                                e);
                        error_count += 1;
                    }
                }
            }
        }

        println!();
        if dry_run {
            println!("{} Would move {} files", "[PREVIEW]".blue(), moved_count);
        } else {
            println!("{} Moved {} files", "[SUCCESS]".green(), moved_count);
        }
        
        if error_count > 0 {
            println!("{} {} errors occurred", "[WARNING]".yellow(), error_count);
        }

        Ok(())
    }

    pub fn clean_empty_dirs(&self, dry_run: bool) -> Result<()> {
        println!("{} Cleaning empty directories...", "[INFO]".cyan());
        println!("Directory: {}", self.source.display());
        println!();

        let mut removed_count = 0;

        // Collect directories to remove (we need to go depth-first)
        let mut dirs_to_check: Vec<PathBuf> = Vec::new();
        
        for entry in WalkDir::new(&self.source)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                dirs_to_check.push(entry.path().to_path_buf());
            }
        }

        // Sort by depth (deepest first)
        dirs_to_check.sort_by(|a, b| b.components().count().cmp(&a.components().count()));

        for dir in dirs_to_check {
            if self.is_empty_dir(&dir)? {
                if dry_run {
                    println!("{} Would remove: {}", "[PREVIEW]".blue(), dir.display());
                } else {
                    fs::remove_dir(&dir)
                        .with_context(|| format!("Failed to remove directory: {}", dir.display()))?;
                    println!("{} Removed: {}", "[SUCCESS]".green(), dir.display());
                }
                removed_count += 1;
            }
        }

        println!();
        if dry_run {
            println!("{} Would remove {} empty directories", "[PREVIEW]".blue(), removed_count);
        } else {
            println!("{} Removed {} empty directories", "[SUCCESS]".green(), removed_count);
        }

        Ok(())
    }

    pub fn show_stats(&self) -> Result<()> {
        println!("{} File statistics for: {}", "[INFO]".cyan(), self.source.display());
        println!();

        let mut file_types: HashMap<String, (usize, u64)> = HashMap::new();
        let mut total_files = 0;
        let mut total_size = 0u64;

        for entry in WalkDir::new(&self.source)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                total_files += 1;
                
                let metadata = entry.metadata()?;
                let size = metadata.len();
                total_size += size;

                let extension = entry.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("no extension")
                    .to_lowercase();

                let entry = file_types.entry(extension).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += size;
            }
        }

        // Sort by file count
        let mut sorted_types: Vec<_> = file_types.iter().collect();
        sorted_types.sort_by(|a, b| b.1.0.cmp(&a.1.0));

        println!("{} {}", "Total files:".bold(), total_files);
        println!("{} {}", "Total size:".bold(), format_file_size(total_size));
        println!();
        println!("{}", "File types:".bold());

        for (ext, (count, size)) in sorted_types.iter().take(10) {
            println!("  {:<15} {:>6} files  {:>10}", 
                    format!(".{}", ext).cyan(), 
                    count.to_string().yellow(),
                    format_file_size(*size).green());
        }

        if sorted_types.len() > 10 {
            println!("  ... and {} more types", sorted_types.len() - 10);
        }

        Ok(())
    }

    fn move_file_by_type(&self, file_path: &Path, dry_run: bool) -> Result<bool> {
        let file_name = file_path.file_name()
            .context("Failed to get file name")?;
        
        let extension = file_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        let folder_name = get_file_type_folder(extension);
        let target_dir = self.target.join(&folder_name);
        let target_path = target_dir.join(file_name);

        // Skip if file is already in the correct location
        if file_path == target_path {
            return Ok(false);
        }

        if dry_run {
            println!("{} {} -> {}", 
                    "[PREVIEW]".blue(),
                    file_path.display(), 
                    target_path.display());
        } else {
            // Create target directory if it doesn't exist
            fs::create_dir_all(&target_dir)
                .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

            // Move the file
            fs::rename(file_path, &target_path)
                .with_context(|| format!("Failed to move file: {} -> {}", 
                                       file_path.display(), 
                                       target_path.display()))?;
            
            println!("{} {} -> {}", 
                    "[MOVED]".green(),
                    file_path.display(), 
                    target_path.display());
        }

        Ok(true)
    }

    fn move_file_by_date(&self, file_path: &Path, format: &str, dry_run: bool) -> Result<bool> {
        let file_name = file_path.file_name()
            .context("Failed to get file name")?;
        
        let metadata = fs::metadata(file_path)?;
        let modified_time = metadata.modified()?;
        let datetime: DateTime<Local> = modified_time.into();

        let folder_name = match format {
            "year" => datetime.format("%Y").to_string(),
            "month" => datetime.format("%Y-%m").to_string(),
            "day" => datetime.format("%Y-%m-%d").to_string(),
            _ => return Err(anyhow::anyhow!("Invalid date format: {}", format)),
        };

        let target_dir = self.target.join(&folder_name);
        let target_path = target_dir.join(file_name);

        // Skip if file is already in the correct location
        if file_path == target_path {
            return Ok(false);
        }

        if dry_run {
            println!("{} {} -> {}", 
                    "[PREVIEW]".blue(),
                    file_path.display(), 
                    target_path.display());
        } else {
            // Create target directory if it doesn't exist
            fs::create_dir_all(&target_dir)
                .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

            // Move the file
            fs::rename(file_path, &target_path)
                .with_context(|| format!("Failed to move file: {} -> {}", 
                                       file_path.display(), 
                                       target_path.display()))?;
            
            println!("{} {} -> {}", 
                    "[MOVED]".green(),
                    file_path.display(), 
                    target_path.display());
        }

        Ok(true)
    }

    fn is_empty_dir(&self, dir_path: &Path) -> Result<bool> {
        let mut entries = fs::read_dir(dir_path)?;
        Ok(entries.next().is_none())
    }
}