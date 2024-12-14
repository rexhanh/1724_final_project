use tokio::task;
mod web;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

mod app;
use app::App;
use color_eyre::Result;

use fern::Dispatch;
use log::{error, info};

fn setup_logging() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info) // Log levels: Error, Warn, Info, Debug, Trace
        .chain(fern::log_file("errors.log")?) // Log to a file named `errors.log`
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Disable Rocket logging
    std::env::set_var("ROCKET_LOG_LEVEL", "off");
    setup_logging().expect("Failed to initialize logging");

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
                result = rocket.launch() => {
                    if let Err(err) = result {
                        // Handle errors that occur during the Rocket launch process
                        error!("Rocket server encountered an error: {}", err);
                    } else {
                        info!("Rocket server shutting down gracefully...");
                    }
                }
                _ = async {
                    while !shutdown_flag.load(Ordering::Relaxed) {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                } => {
                    info!("Shutdown signal received, stopping Rocket...");
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
        error!("Web server error: {:?}", err);
    }

    // Wait for the TUI thread to finish
    if let Err(err) = tui_task.join() {
        error!("TUI error: {:?}", err);
    }

    Ok(())
}
