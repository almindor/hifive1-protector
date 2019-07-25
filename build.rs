use std::{env, fs};
use std::path::PathBuf;

fn main() {
    // Put the memory definitions somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let boards: Vec<_> = env::vars().filter_map(|(key, _value)| {
        if key.starts_with("CARGO_FEATURE_BOARD") {
            Some(key[20..].to_ascii_lowercase())  // Strip 'CARGO_FEATURE_BOARD_'
        } else {
            None
        }
    }).collect();

    if boards.is_empty() {
        panic!("No board features selected");
    }
    if boards.len() > 1 {
        panic!("More than one board feature selected: {:?}", boards);
    }

    let board = boards.first().unwrap();

    match board.as_str() {
        "hifive1" => {
            fs::copy("bootloader-hifive1.x", out_dir.join("bootloader-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=bootloader-hifive1.x");
        }
        "hifive1_revb" => {
            fs::copy("bootloader-hifive1-revb.x", out_dir.join("bootloader-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=bootloader-hifive1-revb.x");
        }
        "lofive" => {}

        other => panic!("Unknown board: {}", other),
    }

    fs::copy("bootloader-link.x", out_dir.join("bootloader-link.x")).unwrap();
}
