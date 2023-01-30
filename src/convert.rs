use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

use crate::config::{download_svg, svg_to_png, ICONS_DIR};

const SET_ICON: &str = r#"
use framework "Cocoa"

on run argv
    set sourcePath to item 1 of argv
    set destPath to item 2 of argv

    set imageData to (current application's NSImage's alloc()'s initWithContentsOfFile:sourcePath)
    (current application's NSWorkspace's sharedWorkspace()'s setIcon:imageData forFile:destPath options:2)
end run
"#;

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

pub fn convert(icon_name: &str, directory_or_file_path: &Path) -> Result<(), Box<dyn Error>> {
    let icon = search_icon(icon_name)?;

    let repository_name = &icon.repository_name;
    let image_path = format!(
        "{}/images/defaults/{repository_name}/{icon_name}.png",
        ICONS_DIR,
        repository_name = repository_name,
        icon_name = icon_name
    );
    if std::path::Path::new(&image_path).exists() {
        set_icon(&image_path, directory_or_file_path.to_str().unwrap());
        return Ok(());
    }

    let svg_path = format!(
        "https://raw.githubusercontent.com/{owner_name}/{repository_name}/{branch_name}/{icon_path}/{icon_name}.svg",
        owner_name = icon.owner_name,
        repository_name = repository_name,
        branch_name = icon.branch_name,
        icon_path = icon.icon_path,
        icon_name = icon_name
    );
    let icon_path = format!(
        "{}/icons/defaults/{repository_name}/{icon_name}.svg",
        ICONS_DIR,
        repository_name = repository_name,
        icon_name = icon_name
    );
    download_svg(&svg_path, &icon_path);
    svg_to_png(&icon_path, &image_path);
    set_icon(&image_path, directory_or_file_path.to_str().unwrap());
    Ok(())
}

pub fn convert_all(directory_or_file_path: &Path, ignore_list: Vec<String>) {
    println!("{}", directory_or_file_path.to_str().unwrap());
    println!("{:?}", ignore_list);
}

fn set_icon(image_path: &str, directory_or_file_path: &str) {
    Command::new("osascript")
        .args(&["-e", SET_ICON, image_path, directory_or_file_path])
        .output()
        .expect("failed to execute process");
}

fn search_icon(icon_name: &str) -> Result<Default, Box<dyn Error>> {
    let icons: Icons =
        serde_json::from_str(&read_to_string(&format!("{}/icons.json", ICONS_DIR)).unwrap())
            .unwrap();
    let icon = icons
        .default_icons
        .iter()
        .find(|icon| icon.icons.iter().any(|icon| icon == icon_name));

    match icon {
        Some(icon) => Ok(icon.clone()),
        None => Err("Icon not found".into()),
    }
}
