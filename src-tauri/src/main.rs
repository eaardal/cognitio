// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use log::{error, info};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config as Log4rsConfig, Root};
use log4rs::encode::pattern::PatternEncoder;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
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
    pub event: String,
}

#[derive(Clone, serde::Serialize)]
pub struct FileEvent {
    pub path: String,
    pub event: String,
}

#[derive(Clone, serde::Serialize)]
pub struct CognitioConfigChangedPayload {
    pub config: CognitioConfig,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CheatsheetInfo {
    title: String,
    path: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Styling {
    menu: Option<Menu>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Menu {
    width: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CognitioConfig {
    editor: Option<String>,
    cheatsheets: Vec<CheatsheetData>,
    styling: Option<Styling>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum CheatsheetData {
    Simple(String),
    Info(CheatsheetInfo),
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

    let (sender, receiver) = channel::<FileEvent>();
    let cheatsheet_sender = sender.clone();
    let cognitio_config_sender = sender.clone();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_cheatsheet,
            load_cheatsheet_section,
            load_cognitio_config,
            list_cheatsheet_directories,
            edit_directory,
            edit_file,
            edit_cognitio_config
        ])
        .setup(|app| {
            let app_handle = app.app_handle();
            watch_cheatsheet_directories(cheatsheet_sender);
            watch_cognitio_config_file(cognitio_config_sender);
            emit_events_to_frontend_when_files_change(receiver, app_handle);
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

    if editor.is_none() {
        return Err(tauri::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Editor is not configured in Cognitio config file",
        )));
    }

    let cmd_output = std::process::Command::new(editor.unwrap())
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

    if editor.is_none() {
        return Err(tauri::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Editor is not configured in Cognitio config file",
        )));
    }

    let cmd_output = std::process::Command::new(editor.unwrap())
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

    if editor.is_none() {
        return Err(tauri::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Editor is not configured in Cognitio config file",
        )));
    }

    let cmd_output = std::process::Command::new(editor.unwrap())
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
fn load_cognitio_config() -> CognitioConfig {
    return read_cognitio_yaml().unwrap();
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
        .map(|cheatsheet_path| match cheatsheet_path {
            CheatsheetData::Simple(path) => {
                let name = Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                Directory {
                    name,
                    path: path.to_string(),
                    files: Vec::new(),
                    sub_directories: list_subdirectories(&path),
                }
            }
            CheatsheetData::Info(info) => {
                let name = info.title.clone();
                Directory {
                    name,
                    path: info.path.clone(),
                    files: Vec::new(),
                    sub_directories: list_subdirectories(&info.path),
                }
            }
        })
        .collect();
    res
}

fn emit_events_to_frontend_when_files_change(receiver: Receiver<FileEvent>, tauri_app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        for msg in receiver.iter() {
            if msg.path.ends_with("cognitio.yaml") {
                tauri_app
                    .emit_all(
                        "cognitio_config_changed",
                        CognitioConfigChangedPayload {
                            config: read_cognitio_yaml().unwrap(),
                        },
                    )
                    .unwrap();
            } else {
                tauri_app
                    .emit_all(
                        "file_changed",
                        FileChangedPayload {
                            path: msg.path,
                            event: msg.event,
                        },
                    )
                    .unwrap();
            }
        }
    });
}

fn watch_cheatsheet_directories(on_file_event: Sender<FileEvent>) {
    for dir in list_cheatsheet_directories() {
        tauri::async_runtime::spawn(watch_dir(dir.path, on_file_event.clone()));
    }
}

fn watch_cognitio_config_file(on_file_event: Sender<FileEvent>) {
    let home = cognitio_home_dir();
    tauri::async_runtime::spawn(watch_dir(home, on_file_event.clone()));
}

async fn watch_dir(path: String, on_file_event: Sender<FileEvent>) {
    if let Err(error) = watch(path.clone(), on_file_event) {
        error!("Watch {} error: {error:?}", path.to_string());
    }
}

fn watch<P: AsRef<Path>>(path: P, on_file_event: Sender<FileEvent>) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                info!("File event: {:?}", event);

                if let Some(path) = event.paths.first() {
                    let path_str = path.to_str().unwrap_or_default().to_string();

                    match event.kind {
                        EventKind::Create(_) => {
                            on_file_event
                                .send(FileEvent {
                                    path: path_str,
                                    event: "create".to_string(),
                                })
                                .unwrap();
                        }
                        EventKind::Remove(_) => {
                            on_file_event
                                .send(FileEvent {
                                    path: path_str,
                                    event: "remove".to_string(),
                                })
                                .unwrap();
                        }
                        EventKind::Modify(_) => {
                            // We seem to get the Modify event when deleting files.
                            // Check if the file exists and send the correct event type.
                            let action = if check_if_path_exists(path_str.as_str()) {
                                "modify".to_string()
                            } else {
                                "remove".to_string()
                            };

                            on_file_event
                                .send(FileEvent {
                                    path: path_str,
                                    event: action,
                                })
                                .unwrap();
                        }
                        _ => {}
                    }
                }
            }
            Err(error) => error!(
                "Failed to watch {}: {error:?}",
                path.as_ref().to_str().unwrap_or_default()
            ),
        }
    }

    Ok(())
}

fn check_if_path_exists(path_str: &str) -> bool {
    Path::new(path_str).exists()
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
