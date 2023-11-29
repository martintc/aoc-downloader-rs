use std::io::Write;

pub fn generate_file_name(day: u16, year: u32) -> String {
    format!("{year}-{day}")
}

pub fn generate_output_path(file_name: String, path: String) -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(path);
    path.push(file_name);
    return path;
}

pub fn write_contents_to_file(
    path: std::path::PathBuf,
    contents: String,
) -> anyhow::Result<(), anyhow::Error> {
    let mut file = std::fs::File::create(path)?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
