mod app;
mod cache;
mod cli;
mod config;
mod display;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod tools;
mod weather;

use app::App;
use errors::RustormyError;

fn main() -> Result<(), RustormyError> {
    let mut app = App::new()?;
    app.run();

    Ok(())
}
