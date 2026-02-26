use std::process::Command;
use std::any::type_name_of_val;

fn main() {

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
    match std::fs::exists(&dir_path) {
        Ok(true) => println!("Exists"),
        _ => std::fs::create_dir(dir_path).expect("Failed"),
    }
}
