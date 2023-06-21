use chrono::{Local, DateTime};
use std::{
    fs::File,
    fs::create_dir_all,
    io::Write,
    env::current_exe,
};
use inputbot::{
    MouseButton::LeftButton,
    handle_input_events,
};

fn lap() {
    // get current time
    let now: DateTime<Local> = Local::now();

    // get file path
    let directory = {
        let folder_name = now.format("%Y%m%d").to_string();
        current_exe().unwrap()
            .parent().unwrap()
            .join(folder_name)
    };
    let path = {
        let filename = now.format("%H%M%S%f").to_string();
        directory.join(filename)
    };

    // format time
    let timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S%.f"));

    // make file and write
    create_dir_all(directory).unwrap();
    let mut file = File::create(path.clone()).unwrap();
    file.write(&timestamp[..].as_bytes()).unwrap();

    // log
    println!("{}", path.to_str().unwrap());
}

fn main() {
    // wait for left click
    LeftButton.bind(|| lap());

    // wait for input
    handle_input_events();
}
