use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

const SIZE: &str = "128";
const SET_ICON: &str = r#"
use framework "Cocoa"

on run argv
    set sourcePath to item 1 of argv
    set destPath to item 2 of argv

    set imageData to (current application's NSImage's alloc()'s initWithContentsOfFile:sourcePath)
    (current application's NSWorkspace's sharedWorkspace()'s setIcon:imageData forFile:destPath options:2)
end run
"#;
const ICONS_URL: &str =
    "https://gist.githubusercontent.com/ogty/989b8175f9714a4adedae680a8f019bb/raw/icons.json";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons {
    pub default_icons: Vec<Default>,
    pub original_icons: Vec<Original>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub icons: Vec<String>,
    pub icon_path: String,
    pub owner_name: String,
    pub branch_name: String,
    pub repository_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub name: String,
    pub path: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let icon_name = &args[1];
    let directory_or_file_path = PathBuf::from(&args[2]);

    let home_dir = std::env::var("HOME").unwrap();
    if !std::path::Path::new(&format!("{}/.icon/icons.json", home_dir)).exists() {
        let response = get(ICONS_URL).unwrap();
        let icons: serde_json::Value = response.json().unwrap();
        for icon in icons["defaultIcons"].as_array().unwrap() {
            let repository_name = &icon["repositoryName"].as_str().unwrap();
            std::fs::create_dir_all(&format!(
                "{}/.icon/images/defaults/{}",
                home_dir, repository_name
            ))
            .unwrap();
            std::fs::create_dir_all(&format!(
                "{}/.icon/icons/defaults/{}",
                home_dir, repository_name
            ))
            .unwrap();
        }
        std::fs::create_dir_all(&format!("{}/.icon", home_dir)).unwrap();
        std::fs::write(&format!("{}/.icon/icons.json", home_dir), icons.to_string()).unwrap();
    }

    let icons: Icons = serde_json::from_str(
        &std::fs::read_to_string(&format!("{}/.icon/icons.json", home_dir)).unwrap(),
    )
    .unwrap();
    let matched = icons
        .default_icons
        .iter()
        .find(|icon| icon.icons.iter().any(|icon| icon == icon_name))
        .unwrap();

    if matched.icons.iter().all(|icon| icon != icon_name) {
        return;
    }

    let repository_name = &matched.repository_name;
    let image_path = format!(
        "{}/.icon/images/defaults/{}/{}.png",
        home_dir, repository_name, icon_name
    );
    if std::path::Path::new(&image_path).exists() {
        set_icon(&image_path, directory_or_file_path.to_str().unwrap());
        return;
    }

    let svg_path = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}/{}.svg",
        matched.owner_name, repository_name, matched.branch_name, matched.icon_path, icon_name
    );
    let icon_path = format!(
        "{}/.icon/icons/defaults/{}/{}.svg",
        home_dir, repository_name, icon_name
    );
    download_svg(&svg_path, &icon_path);
    svg_to_png(&icon_path, &image_path);
    set_icon(&image_path, directory_or_file_path.to_str().unwrap());
}

fn set_icon(image_path: &str, directory_or_file_path: &str) {
    Command::new("osascript")
        .args(&["-e", SET_ICON, image_path, directory_or_file_path])
        .output()
        .expect("failed to execute process");
}

fn svg_to_png(icon_path: &str, png_path: &str) {
    Command::new("rsvg-convert")
        .args(&["-w", SIZE, "-h", SIZE, icon_path, "-o", png_path])
        .output()
        .expect("failed to execute process");
}

fn download_svg(svg_path: &str, icon_path: &str) {
    let response = get(svg_path).unwrap();
    std::fs::write(icon_path, response.bytes().unwrap()).unwrap();
}
