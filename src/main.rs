use std::thread;
use std::time::Duration;

mod ui;
mod app;

fn main() {
    // Spawn a thread to run the UI
    let ui_thread = thread::spawn(|| {
        println!("Starting UI...");
        ui::start_ui();  // This function will run in its own thread
    });

    // Spawn a separate thread to run the background processing
    let bg_thread = thread::spawn(|| {
        println!("Starting background process...");
        app::process();  // This function will run in its own thread
    });

    // Wait for both threads to finish
    ui_thread.join().unwrap();
    bg_thread.join().unwrap();

    println!("Both UI and background processing finished.");
}
