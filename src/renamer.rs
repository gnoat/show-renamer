use clap::{Arg, Command};
use regex::Regex;
use std::fs::rename;
use std::vec::Vec;
use walkdir::WalkDir;

pub struct Renamer {
    pub path: String,
    pub extension: String,
    pub show: Option<String>,
}

impl Renamer {
    pub fn extract() -> Self {
        let app = Command::new("renamer")
            .version("0.1.0")
            .author("shalst")
            .about("Rename movie files")
            .arg(Arg::new("path").short('p').long("path").required(false))
            .arg(
                Arg::new("extension")
                    .short('e')
                    .long("extension")
                    .required(false),
            )
            .arg(Arg::new("show").short('s').long("show").required(false));
        let matches = app.get_matches();
        let path = match matches.get_one::<String>("path") {
            Some(m) => m.to_string(),
            None => ".".to_string(),
        };
        let ext = match matches.get_one::<String>("extension") {
            Some(m) => m.to_string(),
            None => "mp4".to_string(),
        };

        Renamer {
            path: path,
            extension: ext,
            show: matches.get_one::<String>("show").map(|s| s.to_string()),
        }
    }

    pub fn map_episodes(&self) -> Vec<(String, String)> {
        let mut file_vec: Vec<String> = Vec::new();
        for file in WalkDir::new(&self.path).into_iter()
            .filter_map(|file| file.ok())
            .filter(|file| match file.path().extension() {
                None => "",
                Some(f) => f.to_str().unwrap(),
            } == self.extension) {
                file_vec.push(file.path()
                              .to_str()
                              .unwrap()
                              .to_string());
            }
        let name_map: Vec<(String, String)> = file_vec
            .iter()
            .map(|f| (f.to_string(), self.normalize_episodes(f)))
            .collect();

        name_map
    }

    pub fn normalize_episodes<T: AsRef<str>>(&self, file_name: T) -> String {
        let ep_re: Regex = Regex::new(r"([sS]\d+[eE]\d+)").unwrap();
        let ep_first = ep_re.captures(file_name.as_ref()).unwrap();
        let ep_info = ep_first.get(1).unwrap().as_str().to_string();
        let ext: &str = &file_name
            .as_ref()
            .split(".")
            .collect::<Vec<&str>>()
            .split_last()
           .unwrap()
            .0;
        let full_path: Vec<String> = file_name
            .as_ref()
            .split("/")
            .map(|s| s.to_string())
            .collect();
        let partial_path = &full_path.split_last().unwrap().1.join("/");
        let new_path = match &self.show {
            None => format!("{}/{}.{}", partial_path, ep_info, ext),
            Some(f) => format!("{}/{}-{}.{}", partial_path, f, ep_info, ext),
        };

        new_path
    }

    pub fn rename_files(&self) {
        for episode in self.map_episodes().iter() {
            println!("Renaming: {} -> {}", episode.0, episode.1);
            rename(&episode.0, &episode.1).unwrap();
        }
    }
}
