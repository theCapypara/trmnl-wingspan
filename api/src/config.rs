use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub wingsearch: PathBuf,
    pub default_images: String,
    pub images: HashMap<String, ImageSpec>,
    pub new_bird_interval: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageSpec {
    pub path: PathBuf,
    pub token: Option<String>,
}
