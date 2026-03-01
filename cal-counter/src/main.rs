use std::process::Command;
use std::any::type_name_of_val;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo macro"])
            .output()
            .expect("Failed to execute")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo macro")
            .output()
            .expect("Failed to execute")
    };

    let hello = if let Ok(s) = String::from_utf8(output.stdout) {
        s
    } else {
        "Command failed".to_string()
    };
    println!("{}", &hello);
    let curr_path = std::env::current_dir()
        .expect("Failed to get working dir");
    let dir_path = format!("{}/data", curr_path.display());
    let file_path = format!("{}/file.txt", &dir_path);
    match std::fs::exists(&dir_path) {
        Ok(true) => println!("Exists"),
        _ => { std::fs::create_dir(&dir_path).expect("Failed");
               std::fs::File::create(&file_path);
        },
    };
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)?; 

    let bfile = File::open(&file_path)?;
    let last_line = read_lines(&bfile);
    write_to_file(10, 1, &file, &last_line);
    Ok(())
}

use std::fs::File;
use chrono::Utc;
pub fn write_to_file(amount: i16, 
                     mac: u8,
                     mut file: &File,
                     last_line: &String) {
    let date = Utc::now().date_naive();
    if date.to_string() != last_line[0..10] { 
        let content = format!("\n{}_macros", date.to_string());
        file.write_all(content.as_bytes());
        return;
    }
    let macro_txt = match mac {
        0 => "p",
        1 => "c",
        _ => "f",
    };
    let content = format!("{}_{}", amount, macro_txt);
    file.write_all(content.as_bytes());
}

use std::io;
fn read_lines(file: &File) -> String {
    let mut reader = io::BufReader::new(file);
    /*for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }*/
    if let Ok(line) = reader.lines().last().unwrap() {
        println!("{}", &line);
        return line;
    }
    "Nothing".to_string()
}
