use std::process::Command;
use std::any::type_name_of_val;
use std::io::prelude::*;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 { 
        return Ok(()) 
    };
    //println!("{}", args.first().unwrap());
    let a: u8 = match args[1].as_str() {
        "p" => 0,
        "c" => 1,
        "f" => 2,
        "cal" => 3,
        "tot" => 4,
        _ => 5
    };

    if a == 5 {
        println!("Enter valid macronutrient");
        return Ok(());
    };
    if a == 4 {
        // perform some checks per last arg
    };
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
    println!("{}", &file_path);
    match std::fs::exists(&dir_path) {
        Ok(true) => println!("Exists"),
        _ => { std::fs::create_dir(&dir_path).expect("Failed");
               std::fs::File::create(&file_path);
        },
    };
    let bfile = File::open(&file_path)?;
    let file_lines = read_lines(&bfile);  
    let mut lines_string = Vec::new();
    for line in file_lines.lines().map_while(Result::ok) {
        lines_string.push(line);    
    }; 
    if a == 4 {
        calculate_specific_total(args.last().unwrap(), lines_string);
        return Ok(());
    }
    let b = if let Some(num) = 
            args.last().unwrap().chars().nth(0).unwrap().to_digit(10) {
                num as i16
    } else {
            println!("Enter a numerical value");
            return Ok(());
     };

    write_to_file(b, a, &file_path, lines_string);
    Ok(())
}

use std::fs::File;
use chrono::Utc;
pub fn write_to_file(amount: i16, 
                     mac: u8,
                     mut file_path: &str,
                     mut lines_string: Vec<String>) -> io::Result<()> {
    
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;
    let date = Utc::now().date_naive();
    let mut p = 0;
    let mut c = 0;
    let mut f = 0;
    let mut cal = 0;
    match mac {
        0 => p += amount,
        1 => c += amount,
        2 => f += amount,
        _ => cal += amount,
    };
    let last_line = lines_string.pop().expect("gyat");
    let content: String = if date.to_string() == last_line[0..10] {   
        let change = match mac {
            0 => last_line.chars().nth(11).unwrap(),
            1 => last_line.chars().nth(13).unwrap(),
            2 => last_line.chars().nth(15).unwrap(),
            _ => last_line.chars().nth(16).unwrap(),
        };
        let digit = change.to_digit(10).unwrap() as i16 + amount;
        match mac {
            0 => format!("{}p{}{}", date.to_string(), 
                digit.to_string(), last_line[12..].to_string()),
            1 => format!("{}{}{}", last_line[0..13].to_string(),
                                 digit.to_string(),
                                 last_line[14..].to_string()),
            2 => format!("{}{}", last_line[0..15].to_string(),
                                digit.to_string()),
            _ => format!("{}{}", last_line[0..16].to_string(),
                                 digit.to_string()),
        } 
    } else {
        format!("{}\n{}p{}c{}f{}{}", 
            last_line,
            date.to_string(), 
            p.to_string(), 
            c.to_string(), 
            f.to_string(),
            cal.to_string())
    };
    for l in lines_string {
        file.write(format!("{}\n",l).as_bytes());
    }
    println!("{}", calculate_calories(last_line).to_string());
    file.write(content.as_bytes());
    Ok(())
}

use std::io;
fn read_lines(file: &File) -> io::BufReader<&std::fs::File> {
    let reader = io::BufReader::new(file);
    reader
}

fn calculate_calories(entry: String) -> f32 {
    let mut result = 0.0; 
    // handle edge cases
    result += entry.chars().nth(11).unwrap().to_digit(10).unwrap() as f32
        * 4.0;
    result += entry.chars().nth(13).unwrap().to_digit(10).unwrap() as f32
        * 4.0;
    result += entry.chars().nth(15).unwrap().to_digit(10).unwrap() as f32
        * 9.0;
    result += entry.chars().nth(16).unwrap().to_digit(10).unwrap() as f32;
    result
}

pub fn calculate_specific_total(date: &String,
                                mut lines: Vec<String>
) {
    let mut line = "None".to_string();    
    for l in lines {
        if l[0..10] == *date {
            println!("Found");
            line = l.clone();
            break;
        }
    };
    if line == "None" {
        println!("{}", line);
    } else {
        println!("{}", calculate_calories(line).to_string());
    }

}
