use tokio::task;
mod web;

mod app;
use app::App;
use color_eyre::Result;
#[rocket::main]
async fn main() -> Result<()> {
    // Disable Rocket logging
    std::env::set_var("ROCKET_LOG_LEVEL", "off");

    // Install error reporting
    color_eyre::install()?;

    // Spawn the Rocket web server in an async task
    let web_task = task::spawn(async move {
        web::rocket().launch().await.unwrap(); // Run Rocket web server
    });

    // Spawn the TUI application in a separate thread
    let tui_task = std::thread::spawn(|| {
        let terminal = ratatui::init(); // Initialize the terminal
        let app_result = App::default().run(terminal); // Run the TUI
        ratatui::restore(); // Restore terminal settings
        app_result // Return the result
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