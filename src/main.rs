use chrono::{Local, DateTime};
use std::{
    fs::File,
    fs::create_dir_all,
    io::Write,
    io::stdin,
    env::current_exe,
};

fn lap() {
    let now: DateTime<Local> = Local::now();

    let path: String;

    match current_exe() {
        Ok(exe_path) => {
            path = exe_path.parent().unwrap().display().to_string();
        },
        Err(_) => {return;}
    }

    let directory = format!("{}/{}", path, now.format("%Y%m%d"));
    let filename = format!("{}/{}.txt", directory, now.format("%H%M%S%f"));
    let timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S%.f"));

    println!("{}", filename);
    println!("{}", timestamp);

    create_dir_all(directory).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write(&timestamp[..].as_bytes()).unwrap();
}

fn main() {
    loop {
        stdin().read_line(&mut String::new()).unwrap();
        lap();
    }
}
