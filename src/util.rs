use crate::error::ChilonError;
use std::path::Path;

pub fn gen_file_name(name: String, ext: String) -> String {
    let dot_ext = if !ext.is_empty() {
        format!(".{ext}")
    } else {
        "".to_string()
    };
    let mut path = format!("{}{}", name, dot_ext);
    let mut file_path = Path::new(&path);

    let mut copy_count = 1;

    while file_path.exists() {
        copy_count += 1;
        path = format!("{}-{}{}", name, copy_count, dot_ext);
        file_path = Path::new(&path);
    }

    file_path.to_str().unwrap().to_string()
}

pub fn validate_workers(n_workers: usize) -> Result<(), ChilonError> {
    if n_workers < 2 {
        return Err(ChilonError::InvalidInput(format!(
            "Number of workers must be at least 2, got {}",
            n_workers
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn gen_file_name_no_ext() {
        let result = gen_file_name("test".into(), "".into());
        assert_eq!(result, "test");
    }

    #[test]
    fn gen_file_name_with_ext() {
        let result = gen_file_name("test".into(), "txt".into());
        assert_eq!(result, "test.txt");
    }

    #[test]
    fn gen_file_name_existing_creates_copy() {
        let dir = TempDir::new().unwrap();
        let dir_str = dir.path().to_str().unwrap().to_string();

        let first = gen_file_name(format!("{}/test", dir_str), "txt".into());
        assert_eq!(first, format!("{}/test.txt", dir_str));

        File::create(&first).unwrap();

        let second = gen_file_name(format!("{}/test", dir_str), "txt".into());
        assert_eq!(second, format!("{}/test-2.txt", dir_str));
    }

    #[test]
    fn gen_file_name_multiple_copies() {
        let dir = TempDir::new().unwrap();
        let dir_str = dir.path().to_str().unwrap().to_string();

        File::create(format!("{}/doc.txt", dir_str)).unwrap();
        File::create(format!("{}/doc-2.txt", dir_str)).unwrap();

        let result = gen_file_name(format!("{}/doc", dir_str), "txt".into());
        assert_eq!(result, format!("{}/doc-3.txt", dir_str));
    }
}
