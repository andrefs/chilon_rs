use std::path::Path;

pub fn gen_file_name<'a>(name: String, ext: String) -> String {
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

    return file_path.to_str().unwrap().to_string();
}
