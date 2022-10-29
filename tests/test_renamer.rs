use renamer::renamer::Renamer;
use std::fs::File; use std::io::Write;
use tempdir::TempDir;

#[test]
fn test_normalizer() {
    let test_renamer = Renamer {
        path: ".".to_string(),
        extension: "mp4".to_string(),
        show: Some("The Office".to_string())
    };

    let normalized_name = test_renamer.normalize_episodes("./some/random/dir/HonkyTonkyDonkyEpisodeS04E01.mp4");
    assert_eq!(normalized_name, "./some/random/dir/The Office-S04E01.mp4".to_string());
}

#[test]
#[allow(unused_must_use)]
fn test_map_episodes() {
    // create temporary directory to test renaming on
    let temp_dir = TempDir::new("test_temp").unwrap();
    let test_path_1 = temp_dir.path().join("DoggyWoggyDayCareS01E45BAWWIWIWITIT.mp3.mp4");
    let test_path_2 = temp_dir.path().join("asdflahhs04e01youknow_what_it_is.mp4");
    let temp_test_1 = File::create(test_path_1);
    let temp_test_2 = File::create(test_path_2);
    writeln!(temp_test_1.expect("Temp test file 1 not created."), "something something something");
    writeln!(temp_test_2.unwrap(), "something something something");

    let test_renamer = Renamer {
        path: temp_dir.path().to_str().unwrap().to_string(),
        extension: "mp4".to_string(),
        show: Some("The Office".to_string())
    };

    let renaming_map = test_renamer.map_episodes();
    assert_eq!(renaming_map[0].1, temp_dir.path().join("The Office-S01E45.mp4").to_str().unwrap().to_string());
    assert_eq!(renaming_map[1].1, temp_dir.path().join("The Office-s04e01.mp4").to_str().unwrap().to_string());
}
