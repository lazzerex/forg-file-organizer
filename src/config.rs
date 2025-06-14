use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_type_mappings: HashMap<String, String>,
    pub ignore_patterns: Vec<String>,
    pub date_formats: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut file_type_mappings = HashMap::new();
        
        // Images
        for ext in &["jpg", "jpeg", "png", "gif", "bmp", "tiff", "svg", "webp", "ico"] {
            file_type_mappings.insert(ext.to_string(), "Images".to_string());
        }
        
        // Documents
        for ext in &["pdf", "doc", "docx", "txt", "rtf", "odt", "pages"] {
            file_type_mappings.insert(ext.to_string(), "Documents".to_string());
        }
        
        // Code files
        for ext in &["rs", "py", "js", "ts", "html", "css", "cpp", "c", "h", "java"] {
            file_type_mappings.insert(ext.to_string(), "Code".to_string());
        }

        let ignore_patterns = vec![
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            "*.tmp".to_string(),
            "*.temp".to_string(),
            ".git".to_string(),
            ".gitignore".to_string(),
            "node_modules".to_string(),
        ];

        let mut date_formats = HashMap::new();
        date_formats.insert("year".to_string(), "%Y".to_string());
        date_formats.insert("month".to_string(), "%Y-%m".to_string());
        date_formats.insert("day".to_string(), "%Y-%m-%d".to_string());

        Config {
            file_type_mappings,
            ignore_patterns,
            date_formats,
        }
    }
}

impl Config {
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn generate_default<P: AsRef<Path>>(path: P) -> Result<()> {
        let config = Config::default();
        config.save_to_file(path)?;
        Ok(())
    }
}