use dirs;

pub fn format_path(path: &str) -> String {
    let home: String = match dirs::home_dir() {
        Some(dir) => String::from(dir.to_str().expect("Error")),
        None => String::new(),
    };

    return path.replace("~", home.as_str());
}
