use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum VisitDirError {
    IOError { error: io::Error },
    NonDirError,
}

/// Recursively collects the dir entries below the chosen directory, bundled with a forward slash separated path from the starting dir to that entry (starting with a /).
/// If you want to, you can make a prefix for the forward slash separated paths, by handing it an inner_path.
pub fn dir_entries(dir: &Path, inner_path: &str) -> Result<Vec<(DirEntry, String)>, VisitDirError> {
    if dir.is_dir() {
        let mut result = Vec::new();
        let iterator = match fs::read_dir(dir) {
            Ok(i) => i,
            Err(error) => return Err(VisitDirError::IOError { error }),
        };
        for entry in iterator {
            let entry = match entry {
                Ok(e) => e,
                Err(error) => return Err(VisitDirError::IOError { error }),
            };
            let path = entry.path();
            let forward_slash_path = &format!(
                "{}/{}",
                inner_path,
                path.file_name().unwrap().to_str().unwrap()
            );

            if path.is_dir() {
                let mut append = dir_entries(&path, forward_slash_path)?;
                result.append(&mut append);
            } else {
                result.push((entry, forward_slash_path.to_owned()));
            }
        }
        Ok(result)
    } else {
        Err(VisitDirError::NonDirError)
    }
}

#[cfg(test)]
mod tests {
    use super::dir_entries;
    use std::path::Path;
    #[test]
    fn simple_test() {
        let vector = dir_entries(Path::new("./src"), "").unwrap();

        for entry in vector {
            println!("{}", entry.1);
        }
    }
}
