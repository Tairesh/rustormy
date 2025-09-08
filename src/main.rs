mod app;
mod cache;
mod config;
mod display;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod weather;

use app::App;
use errors::RustormyError;

fn main() -> Result<(), RustormyError> {
    let mut app = App::new()?;
    app.run();

    Ok(())
}
