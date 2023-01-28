use reqwest::blocking::get;

const ICONS_DIR: &str = concat!(env!("HOME"), "/.icon");
const ICONS_URL: &str = concat!(
    "https://raw.githubusercontent.com/",
    env!("CARGO_PKG_AUTHORS"),
    "/icon/main/data/icons.json"
);

pub fn setup() {
    let response = get(ICONS_URL).unwrap();
    let icons: serde_json::Value = response.json().unwrap();
    for icon in icons["defaultIcons"].as_array().unwrap() {
        let repository_name = &icon["repositoryName"].as_str().unwrap();
        std::fs::create_dir_all(&format!(
            "{}/images/defaults/{}",
            ICONS_DIR, repository_name
        ))
        .unwrap();
        std::fs::create_dir_all(&format!("{}/icons/defaults/{}", ICONS_DIR, repository_name))
            .unwrap();
    }
    std::fs::create_dir_all(ICONS_DIR).unwrap();
    std::fs::write(&format!("{}/icons.json", ICONS_DIR), icons.to_string()).unwrap();
}
