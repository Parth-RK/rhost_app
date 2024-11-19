// background.rs
use std::process::Command;
use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
use std::os::windows::process::CommandExt;
pub fn process() {
    // background task code
    println!("Background process started...");

    let output = Command::new("python") // or "python3"
        .arg(r"C:\Users\JOHN\Desktop\PRK\rhost_app\sample_data\subdir\rnsm69.py") // specify the path to the script
        .creation_flags(CREATE_NO_WINDOW)
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
