// background.rs
use std::process::Command;
pub fn process() {
    // Your background task code here
    println!("Background process started...");

    let output = Command::new("python") // or "python"
        .arg(r"C:\Users\JOHN\Desktop\PRK\rhost_app\rnsm.py") // specify the path to the script
        .output()
        .expect("Failed to execute Python script");

    // Convert output to a string and print
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    println!("Output: {}", stdout);
    if !stderr.is_empty() {
        eprintln!("Error: {}", stderr);
    }
}
