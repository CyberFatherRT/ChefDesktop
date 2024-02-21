use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

#[tauri::command]
pub fn read_from_file(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|err| err.to_string())?;
    let mut buf_reader = BufReader::new(file);
    let mut content = Vec::new();

    buf_reader
        .read_to_end(&mut content)
        .map_err(|err| err.to_string())?;

    Ok(String::from_utf8_lossy(&content).to_string())
}

#[tauri::command]
pub fn save_to_file(path: &Path, content: String) -> Result<(), String> {
    let file = File::create(path).map_err(|err| err.to_string())?;
    let mut buf_writer = BufWriter::new(file);

    buf_writer
        .write_all(content.as_bytes())
        .map_err(|err| err.to_string())?;

    Ok(())
}
