use reqwest::blocking::get;
use std::fs::{create_dir_all, read_to_string, remove_dir_all, write};
use std::path::Path;
use std::process::Command;

pub const ICONS_DIR: &str = concat!(env!("HOME"), "/.icon");
const SIZE: &str = "128";
const ICONS_URL: &str = concat!(
    "https://raw.githubusercontent.com/",
    env!("CARGO_PKG_AUTHORS"),
    "/icon/main/data/icons.json"
);

pub fn update() {
    let response = get(ICONS_URL).unwrap();
    let icons: serde_json::Value = response.json().unwrap();
    let mut icons = icons;

    let original_icons = read_to_string(&format!("{}/icons.json", ICONS_DIR)).unwrap();
    let original_icons: serde_json::Value = serde_json::from_str(&original_icons).unwrap();

    icons["originalIcons"] = original_icons["originalIcons"].clone();

    std::fs::write(&format!("{}/icons.json", ICONS_DIR), icons.to_string()).unwrap();
}

pub fn setup() {
    let response = get(ICONS_URL).unwrap();
    let icons: serde_json::Value = response.json().unwrap();

    if Path::new(ICONS_DIR).exists() {
        remove_dir_all(ICONS_DIR).unwrap();
    }

    for icon in icons["defaultIcons"].as_array().unwrap() {
        let repository_name = &icon["repositoryName"].as_str().unwrap();
        create_dir_all(&format!(
            "{}/images/defaults/{repository_name}",
            ICONS_DIR,
            repository_name = repository_name
        ))
        .unwrap();
        create_dir_all(&format!(
            "{}/icons/defaults/{repository_name}",
            ICONS_DIR,
            repository_name = repository_name
        ))
        .unwrap();
    }

    create_dir_all(&format!("{}/images/originals", ICONS_DIR)).unwrap();
    create_dir_all(&format!("{}/icons/originals", ICONS_DIR)).unwrap();
    create_dir_all(ICONS_DIR).unwrap();
    write(&format!("{}/icons.json", ICONS_DIR), icons.to_string()).unwrap();
}

pub fn svg_to_png(icon_path: &str, png_path: &str) {
    Command::new("rsvg-convert")
        .args(&["-w", SIZE, "-h", SIZE, icon_path, "-o", png_path])
        .output()
        .expect("failed to execute process");
}

pub fn download_svg(svg_path: &str, icon_path: &str) {
    let response = get(svg_path).unwrap();
    write(icon_path, response.bytes().unwrap()).unwrap();
}
