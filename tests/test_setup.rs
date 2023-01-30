use dirs;
use std::fs::metadata;

use icon::config::setup;

#[test]
fn test_setup() {
    let home_dir = dirs::home_dir().unwrap();

    let icons_json_path = home_dir.join(".icon/icons.json");
    let originals_icons_path = home_dir.join(".icon/icons/originals");
    let originals_images_path = home_dir.join(".icon/images/originals");
    let defaults_icons_path = home_dir.join(".icon/icons/defaults");
    let defaults_images_path = home_dir.join(".icon/images/defaults");

    setup();

    assert!(metadata(icons_json_path).unwrap().is_file());
    assert!(metadata(originals_icons_path).unwrap().is_dir());
    assert!(metadata(originals_images_path).unwrap().is_dir());
    assert!(metadata(defaults_icons_path).unwrap().is_dir());
    assert!(metadata(defaults_images_path).unwrap().is_dir());
}
