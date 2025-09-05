use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileFinder;

impl FileFinder {
    pub fn find_latest_file(directory: &str, pattern: &str) -> Result<Option<PathBuf>> {
        let search_pattern = if Path::new(directory).is_absolute() {
            format!("{}/{}", directory, pattern)
        } else {
            format!("{}/{}/{}", std::env::current_dir()?.display(), directory, pattern)
        };

        let mut latest_file: Option<(PathBuf, DateTime<Local>)> = None;

        for entry in glob(&search_pattern).context("Failed to read glob pattern")? {
            match entry {
                Ok(path) => {
                    let metadata = fs::metadata(&path)
                        .with_context(|| format!("Failed to get metadata for {:?}", path))?;
                    
                    if metadata.is_file() {
                        let modified = metadata.modified()
                            .with_context(|| format!("Failed to get modified time for {:?}", path))?;
                        let modified_time: DateTime<Local> = modified.into();

                        match &latest_file {
                            None => latest_file = Some((path, modified_time)),
                            Some((_, current_latest)) => {
                                if modified_time > *current_latest {
                                    latest_file = Some((path, modified_time));
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error reading glob entry: {:?}", e),
            }
        }

        Ok(latest_file.map(|(path, _)| path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_find_latest_file() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        let file1_path = dir_path.join("file1.txt");
        let mut file1 = File::create(&file1_path)?;
        writeln!(file1, "First file")?;

        thread::sleep(Duration::from_millis(10));

        let file2_path = dir_path.join("file2.txt");
        let mut file2 = File::create(&file2_path)?;
        writeln!(file2, "Second file")?;

        thread::sleep(Duration::from_millis(10));

        let file3_path = dir_path.join("file3.txt");
        let mut file3 = File::create(&file3_path)?;
        writeln!(file3, "Third file")?;

        let _other_file = File::create(dir_path.join("other.log"))?;

        let result = FileFinder::find_latest_file(
            dir_path.to_str().unwrap(),
            "*.txt"
        )?;

        assert!(result.is_some());
        assert_eq!(result.unwrap(), file3_path);

        Ok(())
    }

    #[test]
    fn test_find_latest_file_no_match() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        let _file = File::create(dir_path.join("file.log"))?;

        let result = FileFinder::find_latest_file(
            dir_path.to_str().unwrap(),
            "*.txt"
        )?;

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_find_latest_file_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        let result = FileFinder::find_latest_file(
            dir_path.to_str().unwrap(),
            "*.txt"
        )?;

        assert!(result.is_none());

        Ok(())
    }
}