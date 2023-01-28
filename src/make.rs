use serde_json::{json, Value};
use std::error::Error;
use std::fs::{read_to_string, write};
use url::Url;

use crate::config::{download_svg, svg_to_png, ICONS_DIR};

pub fn make(icon_name: &str, url_or_path: &str) -> Result<(), Box<dyn Error>> {
    let icon_path = format!(
        "{}/icons/originals/{icon_name}.svg",
        ICONS_DIR,
        icon_name = icon_name
    );
    if Url::parse(url_or_path).is_ok() {
        download_svg(url_or_path, &icon_path);
    }

    let image_path = format!(
        "{}/images/originals/{icon_name}.png",
        ICONS_DIR,
        icon_name = icon_name
    );
    svg_to_png(&icon_path, &image_path);

    let json_str =
        read_to_string(format!("{}/icons.json", ICONS_DIR)).expect("Failed to read icons.json");
    let mut json: Value = serde_json::from_str(&json_str).unwrap();
    let original_icons = json["originalIcons"].as_array_mut().unwrap();
    original_icons.push(json!({
        "name": icon_name,
        "path": url_or_path,
    }));
    write(format!("{}/icons.json", ICONS_DIR), json.to_string())
        .expect("Failed to write icons.json");

    Ok(())
}
