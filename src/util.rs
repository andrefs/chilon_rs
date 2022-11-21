use std::path::Path;

pub fn gen_file_name<'a>(name: String, ext: String) -> String {
    let mut path = format!("{}.{}", name, ext);
    let mut file_path = Path::new(&path);

    let mut copy_count = 1;
    while file_path.exists() {
        copy_count += 1;
        path = format!("{}-{}.{}", name, copy_count, ext);
        file_path = Path::new(&path);
    }
    return file_path.to_str().unwrap().to_string();
}
