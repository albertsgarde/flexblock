use simplelog::*;
use std::fs;
use std::path::{Path, PathBuf};

fn log_file_path(index: u32) -> PathBuf {
    let mut result = PathBuf::new();
    result.push(Path::new(&format!("target/logs/log{}.log", index)));
    result
}

fn ensure_log_dir() {
    std::fs::create_dir_all(log_file_path(0).as_path().parent().unwrap())
        .expect("Could not create log dir.");
}

fn log_index() -> u32 {
    (0..)
        .find(|&log_index| {
            let file_path = log_file_path(log_index);
            match file_path.as_path().metadata() {
                Ok(_) => false,
                Err(error) => match error.kind() {
                    std::io::ErrorKind::NotFound => true,
                    _ => panic!("Could not create log file. Error: {:?}", error),
                },
            }
        })
        .unwrap()
}

pub fn log_init() {
    ensure_log_dir();

    let log_file_path = log_file_path(log_index());

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            fs::File::create(log_file_path).expect("Could not create log file"),
        ),
    ])
    .unwrap();
}
