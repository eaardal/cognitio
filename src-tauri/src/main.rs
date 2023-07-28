// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Read;
use std::io::Result;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![read_cognitio_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn read_cognitio_dir() -> String {
    let file_path = "/Users/eaardal/.config/cognitio/bash/variables.md";
    match read_file_to_string(file_path) {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
            contents
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            format!("Error: {}", e)
        }
    }
}

fn read_file_to_string(file_path: &str) -> Result<String> {
    // Open the file in read-only mode
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
