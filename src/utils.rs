/// Maps file extensions to logical folder names
pub fn get_file_type_folder(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        // Images
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "svg" | "webp" | "ico" => "Images".to_string(),
        
        // Documents
        "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "pages" => "Documents".to_string(),
        
        // Spreadsheets
        "xls" | "xlsx" | "csv" | "ods" | "numbers" => "Spreadsheets".to_string(),
        
        // Presentations
        "ppt" | "pptx" | "odp" | "key" => "Presentations".to_string(),
        
        // Videos
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "3gp" => "Videos".to_string(),
        
        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "Audio".to_string(),
        
        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "dmg" | "iso" => "Archives".to_string(),
        
        // Code
        "rs" | "py" | "js" | "ts" | "html" | "css" | "cpp" | "c" | "h" | "java" | 
        "go" | "php" | "rb" | "swift" | "kt" | "cs" | "vb" | "sql" | "sh" | "bat" | 
        "ps1" | "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" => "Code".to_string(),
        
        // Executables
        "exe" | "msi" | "deb" | "rpm" | "pkg" | "app" => "Applications".to_string(),
        
        // Fonts
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "Fonts".to_string(),
        
        // 3D/CAD
        "obj" | "fbx" | "dae" | "3ds" | "blend" | "max" | "dwg" | "dxf" => "3D-Models".to_string(),
        
        // eBooks
        "epub" | "mobi" | "azw" | "azw3" | "fb2" => "eBooks".to_string(),
        
        // Unknown/No extension
        "unknown" | "" => "Others".to_string(),
        
        // Default case - use the extension as folder name
        _ => format!("{}-Files", extension.to_uppercase()),
    }
}

/// formats file size in human-readable format 
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

//run tests because why not

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_mapping() {
        assert_eq!(get_file_type_folder("jpg"), "Images");
        assert_eq!(get_file_type_folder("JPG"), "Images");
        assert_eq!(get_file_type_folder("pdf"), "Documents");
        assert_eq!(get_file_type_folder("rs"), "Code");
        assert_eq!(get_file_type_folder("unknown"), "Others");
        assert_eq!(get_file_type_folder("xyz"), "XYZ-Files");
    }

    #[test]
    fn test_file_size_formatting() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1023), "1023 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }
}