// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use tauri::api::file;

#[derive(Serialize, Deserialize)]
struct DirectoryFile {
    name: String,
    path: String,
}

#[derive(Serialize)]
struct Directory {
    name: String,
    path: String,
    files: Vec<DirectoryFile>,
}

#[derive(Serialize)]
struct Manifest {
    title: String,
    icon: String,
    chapters: Vec<String>,
    sections: Vec<String>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greeting,
            load_cheatsheet,
            list_cheatsheet_directories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greeting() -> String {
    format!("Hello, person")
}

#[tauri::command]
fn load_cheatsheet(files: Vec<DirectoryFile>) -> HashMap<String, String> {
    let mut cheatsheet: HashMap<String, String> = HashMap::new();
    for file in files {
        let file_content = read_file_to_string(&file.path).unwrap_or_default();
        let clean_name = file.name.replace(".md", "");
        cheatsheet.insert(clean_name, file_content);
    }
    cheatsheet
}

#[tauri::command]
fn list_cheatsheet_directories() -> Vec<Directory> {
    let home = cognitio_home_dir();
    list_subdirectories(&home)
}

fn cognitio_home_dir() -> String {
    let home_dir = match env::var("HOME") {
        Ok(value) => value,
        Err(_e) => {
            eprintln!("Error reading environment variable HOME: Empty or not exist");
            String::from("asdasd")
        }
    };

    match env::var("COGNITIO_HOME") {
        Ok(value) => value,
        Err(_e) => PathBuf::from(home_dir)
            .join(".config")
            .join("cognitio")
            .to_str()
            .unwrap_or_default()
            .to_string(),
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

// fn get_subdirectories(root_dir: &str) -> Vec<String> {
//     let mut dirs = Vec::new();

//     if let Ok(entries) = fs::read_dir(root_dir) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let path = entry.path();
//                 if path.is_dir() {
//                     dirs.push(path.to_str().unwrap_or_default().to_string())
//                 }
//             }
//         }
//     }

//     dirs
// }

fn list_subdirectories(root_dir: &str) -> Vec<Directory> {
    let mut directories = Vec::new();

    if let Ok(entries) = fs::read_dir(root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    // let subdirectories = list_subdirectories(path.to_str().unwrap());
                    let files = list_files_in_directory(path.to_str().unwrap());
                    directories.push(Directory {
                        name,
                        path: path.to_str().unwrap().to_string(),
                        files,
                    });
                }
            }
        }
    }

    directories
}

fn list_files_in_directory(directory_path: &str) -> Vec<DirectoryFile> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with(".md") {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    files.push(DirectoryFile {
                        name,
                        path: path.to_str().unwrap().to_string(),
                    });
                }
            }
        }
    }

    files
}
