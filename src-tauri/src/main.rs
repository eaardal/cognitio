// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use log::{error, info};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config as Log4rsConfig, Root};
use log4rs::encode::pattern::PatternEncoder;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use serde::Serialize;
use serde_yaml;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryFile {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub files: Vec<DirectoryFile>,
    pub sub_directories: Vec<Directory>,
}

#[derive(Clone, serde::Serialize)]
pub struct FileChangedPayload {
    pub path: String,
}

#[derive(Debug, Deserialize)]
struct CognitioConfig {
    pub cheatsheets: Vec<String>,
    pub editor: String,
}

fn main() {
    setup_logger();

    if let Err(error) = run_tauri() {
        error!("Application error: {error:?}");
    }
}

fn setup_logger() {
    let home = cognitio_home_dir();
    let log_path = PathBuf::from(home).join("cognitio.log");

    let encoder = PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}");

    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build();

    let file_appender = FileAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build(log_path)
        .unwrap();

    let root = Root::builder()
        .appender("stdout_logger")
        .appender("file_logger")
        .build(LevelFilter::Debug);

    let config = Log4rsConfig::builder()
        .appender(Appender::builder().build("stdout_logger", Box::new(stdout_appender)))
        .appender(Appender::builder().build("file_logger", Box::new(file_appender)))
        .build(root)
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn run_tauri() -> Result<(), tauri::Error> {
    if let Err(error) = fix_path_env::fix() {
        error!("Failed to fix PATH environment variable: {error:?}")
    }

    let (sender, receiver) = channel::<String>();
    let cheatsheet_sender = sender.clone();
    let home_dir_sender = sender.clone();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_cheatsheet,
            load_cheatsheet_section,
            list_cheatsheet_directories,
            edit_directory,
            edit_file,
            edit_cognitio_config
        ])
        .setup(|app| {
            let app_handle = app.app_handle();
            watch_cheatsheet_directories(cheatsheet_sender);
            watch_cognitio_home_dir(home_dir_sender);
            emit_event_to_frontend_when_file_changed(receiver, app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
}

#[tauri::command]
fn edit_cognitio_config() -> Result<String, tauri::Error> {
    let home = cognitio_home_dir();
    let yaml_path = PathBuf::from(home).join("cognitio.yaml");
    let conf = read_cognitio_yaml().unwrap();
    let editor = conf.editor;

    let cmd_output = std::process::Command::new(editor)
        .arg(yaml_path.clone())
        .output();

    match cmd_output {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(error) => {
            error!(
                "Failed to edit Cognitio config {}: {:?}",
                yaml_path.clone().to_str().unwrap(),
                error
            );
            Err(tauri::Error::from(error))
        }
    }
}

#[tauri::command]
fn edit_directory(path: String) -> Result<String, tauri::Error> {
    let conf = read_cognitio_yaml().unwrap();
    let editor = conf.editor;

    let cmd_output = std::process::Command::new(editor)
        .arg(path.clone())
        .output();

    match cmd_output {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(error) => {
            error!("Failed to edit directory {}: {:?}", path.clone(), error);
            Err(tauri::Error::from(error))
        }
    }
}

#[tauri::command]
fn edit_file(path: String) -> Result<String, tauri::Error> {
    let conf = read_cognitio_yaml().unwrap();
    let editor = conf.editor;

    let cmd_output = std::process::Command::new(editor)
        .arg(path.clone())
        .output();

    match cmd_output {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(error) => {
            error!("Failed to edit file {}: {:?}", path.clone(), error);
            Err(tauri::Error::from(error))
        }
    }
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
fn load_cheatsheet_section(path: String) -> HashMap<String, String> {
    let mut cheatsheet: HashMap<String, String> = HashMap::new();
    let file_content = read_file_to_string(&path).unwrap_or_default();
    let file_name = Path::new(&path)
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string()
        .replace(".md", "");
    cheatsheet.insert(file_name, file_content);
    cheatsheet
}

#[tauri::command]
fn list_cheatsheet_directories() -> Vec<Directory> {
    let conf = read_cognitio_yaml().unwrap();
    let res: Vec<Directory> = conf
        .cheatsheets
        .iter()
        .map(|cheatsheet_path| Directory {
            name: Path::new(cheatsheet_path)
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string(),
            path: String::from(cheatsheet_path),
            sub_directories: list_subdirectories(&cheatsheet_path),
            files: Vec::new(),
        })
        .collect();
    // info!("Cheatsheets: {:?}", serde_json::to_string(&res).unwrap());
    res
}

fn emit_event_to_frontend_when_file_changed(receiver: Receiver<String>, tauri_app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        for msg in receiver.iter() {
            tauri_app
                .emit_all("file_changed", FileChangedPayload { path: msg })
                .unwrap();
        }
    });
}

fn watch_cheatsheet_directories(on_file_changed: Sender<String>) {
    for dir in list_cheatsheet_directories() {
        tauri::async_runtime::spawn(watch_dir(dir.path, on_file_changed.clone()));
    }
}

fn watch_cognitio_home_dir(on_file_changed: Sender<String>) {
    let home = cognitio_home_dir();
    tauri::async_runtime::spawn(watch_dir(home, on_file_changed.clone()));
}

async fn watch_dir(path: String, on_file_changed: Sender<String>) {
    if let Err(error) = watch(path.clone(), on_file_changed) {
        error!("Watch {} error: {error:?}", path.to_string());
    }
}

fn watch<P: AsRef<Path>>(path: P, on_file_changed: Sender<String>) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                info!("File changed: {:?}", event.paths);
                on_file_changed
                    .send(event.paths[0].to_str().unwrap_or_default().into())
                    .unwrap();
            }
            Err(error) => error!(
                "Failed to watch {}: {error:?}",
                path.as_ref().to_str().unwrap_or_default()
            ),
        }
    }

    Ok(())
}

fn read_file_to_string(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
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
                    if name.starts_with(".") {
                        continue;
                    }
                    let files = list_files_in_directory(path.to_str().unwrap());
                    directories.push(Directory {
                        name,
                        path: path.to_str().unwrap().to_string(),
                        files,
                        sub_directories: Vec::new(),
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

fn read_cognitio_yaml() -> std::io::Result<CognitioConfig> {
    let home = cognitio_home_dir();
    let yaml_path = PathBuf::from(home).join("cognitio.yaml");
    let mut file = File::open(yaml_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let manifest: CognitioConfig = serde_yaml::from_str(&contents).unwrap();
    Ok(manifest)
}

fn cognitio_home_dir() -> String {
    let home_dir = env::var("HOME").unwrap();
    if home_dir.is_empty() {
        error!("Error reading environment variable HOME: Empty or not exist");
    }

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
