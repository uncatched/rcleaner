pub fn home_path_to(path: &str) -> String {
    let home = std::env::var("HOME").unwrap();
    format!("{}{}", home, path)
}
