use chrono::{Local, DateTime};
use std::{
    fs::File,
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

    let filename = format!("{}/{}.txt", path, now.format("%Y%m%d%H%M%S%f"));
    let timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S%.f"));

    println!("{}", filename);
    println!("{}", timestamp);

    let mut file = File::create(filename).unwrap();
    file.write(&timestamp[..].as_bytes()).unwrap();
}

fn main() {
    loop {
        stdin().read_line(&mut String::new()).unwrap();
        lap();
    }
}
