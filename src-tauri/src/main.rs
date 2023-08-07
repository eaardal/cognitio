// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures::channel::mpsc::Sender;
use futures::channel::oneshot::channel;
use futures::SinkExt;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use tauri::App;
use tauri::Manager;

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
    // let handle = thread::spawn(|| {
    //     watch_cognitio_dir();
    // });

    run_tauri();

    // handle.join().unwrap();
}

#[derive(Clone, serde::Serialize)]
struct FileChangedPayload {
    path: String,
}

type FileChangedCallback = dyn Fn(&str, &str) -> Result<()>;

fn run_tauri() {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greeting,
            load_cheatsheet,
            list_cheatsheet_directories
        ])
        .setup(|app| {
            let h = app.app_handle();
            tauri::async_runtime::spawn(watch_cognitio_dir(sender));
            tauri::async_runtime::spawn(async move {
                for msg in receiver.iter() {
                    println!("Received file changed: {}", msg);
                    // app.emit_all("file-changed", FileChangedPayload { path: msg });
                    h.emit_all("file-changed", FileChangedPayload { path: msg });
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn watch_cognitio_dir(on_file_changed: std::sync::mpsc::Sender<String>) {
    let home = cognitio_home_dir();
    if let Err(error) = watch(home, on_file_changed) {
        println!("Error: {error:?}");
    }
}

fn watch<P: AsRef<Path>>(
    path: P,
    on_file_changed: std::sync::mpsc::Sender<String>,
) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("Change: {event:?}");
                on_file_changed.send(event.paths[0].to_str().unwrap_or_default().into());
            }

            Err(error) => println!("Error: {error:?}"),
        }
    }

    Ok(())
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
