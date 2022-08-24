use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct PiHoleCtlConfig {
    /// Named HostKeyPair definitions for each Pi-Hole
    pub hosts: HashMap<String, HostKeyPair>,
    /// Named groups which map to named hosts defined in the `hosts` section
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostKeyPair {
    pub host: String,
    pub key: Option<String>,
}

pub fn get_config_file(override_path: &Option<PathBuf>, verbose: bool) -> PiHoleCtlConfig {
    // Use the override path instead of the default config if set
    let project_paths = ProjectDirs::from("com", "scratchcat1", "piholectl")
        .expect("Unable to determine the default config location");
    let mut default_config_path = project_paths.config_dir().to_path_buf();
    default_config_path.push("config.json");
    let config_file_path = override_path.clone().unwrap_or(default_config_path);

    if verbose {
        println!("Using configuration path {}", config_file_path.display());
    }

    let f = File::open(config_file_path);

    match f {
        Ok(reader) => serde_json::from_reader(&reader).expect("Failed to parse config file"),
        r @ Err(_) => {
            // Always error if the user provided path did not work
            // If using the default path, ignore file not found errors.
            if override_path.is_some()
                || r.as_ref().err().unwrap().kind() != std::io::ErrorKind::NotFound
            {
                r.expect("Error reading config file");
            }
            PiHoleCtlConfig::default()
        }
    }
}
