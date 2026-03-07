use walkdir;
use walkdir::WalkDir;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn search_files(query: &str, dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    if query.is_empty() {
        return Ok(matches);
    }
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("<unknown>");
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if line.contains(query) {
                matches.push(format!(
                    "{:?} at line {}: {}",
                    file_name,
                    line_num + 1,
                    line
                ));
            }
        }
    }
    Ok(matches)
}
