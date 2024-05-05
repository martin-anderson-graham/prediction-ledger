mod app;
mod components;
mod prediction;
mod tui;

fn main() -> color_eyre::eyre::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app::app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
