use tokio::task;
mod web;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

mod app;
use app::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Disable Rocket logging
    std::env::set_var("ROCKET_LOG_LEVEL", "off");

    // Install error reporting
    color_eyre::install()?;

    // Create a shared flag to signal shutdown
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let _shutdown_flag_clone = Arc::clone(&shutdown_flag);

    // Spawn the Rocket web server in an async task
    let web_task = task::spawn({
        let shutdown_flag = Arc::clone(&shutdown_flag);
        async move {
            let rocket = web::rocket();
            // Run the Rocket server with a loop checking the shutdown flag
            tokio::select! {
                _ = rocket.launch() => {
                    println!("Rocket server shutting down...");
                }
                _ = async {
                    while !shutdown_flag.load(Ordering::Relaxed) {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                } => {
                    println!("Shutdown signal received, stopping Rocket...");
                }
            }
        }
    });

    // Spawn the TUI application in a separate thread
    let tui_task = std::thread::spawn({
        let shutdown_flag = Arc::clone(&shutdown_flag);
        move || {
            let terminal = ratatui::init(); // Initialize the terminal
            let app_result = App::default().run(terminal); // Run the TUI
            ratatui::restore(); // Restore terminal settings
            // Set the shutdown flag when TUI exits
            shutdown_flag.store(true, Ordering::Relaxed);
            app_result // Return the result
        }
    });

    // Await the web server task
    if let Err(err) = web_task.await {
        eprintln!("Web server error: {:?}", err);
    }

    // Wait for the TUI thread to finish
    if let Err(err) = tui_task.join() {
        eprintln!("TUI error: {:?}", err);
    }

    Ok(())
}