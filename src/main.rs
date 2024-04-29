use std::io;

mod prediction;
mod tui;
mod app;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app::app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
