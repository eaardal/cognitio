use clap::{Parser, Subcommand};
use log::LevelFilter;
use log::{error, info};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config as Log4rsConfig, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryFile {
    pub name: String,
    pub path: String,
    pub shorthand_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub files: Vec<DirectoryFile>,
    pub sub_directories: Vec<Directory>,
    pub shorthand_id: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    #[clap(alias = "ls")]
    List {},
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

    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::List {}) => {
            list_tree_down_to_snippet_names();
        }
        None => {}
    }

    // Continued program logic goes here...
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

fn list_tree_down_to_snippet_names() {
    let dirs = list_cheatsheet_directories();
    let mut sorted_dirs = sort_directories_and_files(&dirs);
    enrich_directories_with_shorthand_ids(&mut sorted_dirs);
    print_directory_tree(&dirs)
}

fn print_directory_tree(dirs: &Vec<Directory>) {
    dirs.iter().for_each(|dir| {
        println!("{}", dir.name);
        dir.files.iter().for_each(|file| {
            println!("  {}", file.name);
        });
        dir.sub_directories.iter().for_each(|sub_dir| {
            println!(
                "  {} ({})",
                sub_dir.name,
                make_short_id_from_two_first_letters_of_path_parts(&sub_dir.path, 2)
            );
            sub_dir.files.iter().for_each(|file| {
                println!(
                    "    {} ({})",
                    file.name,
                    make_short_id_from_two_first_letters_of_path_parts(&file.path, 3),
                );
            });
        });
    });
}

fn enrich_directories_with_shorthand_ids(dirs: &mut Vec<Directory>) {
    dirs.iter_mut().for_each(|dir| {
        dir.shorthand_id = make_short_id_from_two_first_letters_of_path_parts(&dir.path, 2);
        dir.files.iter_mut().for_each(|file| {
            file.shorthand_id = make_short_id_from_two_first_letters_of_path_parts(&file.path, 3);
        });
        enrich_directories_with_shorthand_ids(&mut dir.sub_directories);
    });
}

fn make_short_id_from_two_first_letters_of_path_parts(
    file_path: &str,
    dirs_backwards: usize,
) -> String {
    let path = Path::new(file_path);
    let mut id_parts = Vec::new();
    let mut count = 0;

    for component in path.components().rev() {
        if count == dirs_backwards {
            break;
        }
        if let Some(name) = component.as_os_str().to_str() {
            // Take first two letters of name and push them to id_parts
            id_parts.push(name.to_lowercase().chars().take(2).collect::<String>());
            count += 1;
        }
    }

    // Reverse the ID parts and join them together to the final ID-string
    id_parts.reverse();
    id_parts.join("")
}

fn sort_directories_and_files(dirs: &Vec<Directory>) -> Vec<Directory> {
    let mut sorted_dirs = dirs.clone();
    sorted_dirs.sort_by(|a, b| a.name.cmp(&b.name));
    sorted_dirs.iter_mut().for_each(|dir| {
        dir.files.sort_by(|a, b| a.name.cmp(&b.name));
        dir.sub_directories = sort_directories_and_files(&dir.sub_directories);
    });
    sorted_dirs.to_vec()
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
                    shorthand_id: "".to_string(),
                }
            }
            CheatsheetData::Info(info) => {
                let name = info.title.clone();
                Directory {
                    name,
                    path: info.path.clone(),
                    files: Vec::new(),
                    sub_directories: list_subdirectories(&info.path),
                    shorthand_id: "".to_string(),
                }
            }
        })
        .collect();
    res
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
                        shorthand_id: "".to_string(),
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
                        shorthand_id: "".to_string(),
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
