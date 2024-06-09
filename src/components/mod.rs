use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub mod graph;
pub mod prediction_details;
pub mod prediction_list;
// use crate::{action::Action, event::Event, terminal::Frame};

/// `Component` is a trait that represents a visual and interactive element of the user interface.
/// Implementors of this trait can be registered with the main application loop and will be able to receive events,
/// update state, and be rendered on the screen.
pub trait Component {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    // fn handle_events(&mut self, event: Option<Event>) -> Action {
    fn handle_events(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    // {
    //     match event {
    //         // Some(Event::Quit) => Action::Quit,
    //         // Some(Event::Tick) => Action::Tick,
    //         Some(Event::Key(key_event)) => self.handle_key_events(key_event),
    //         // Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
    //         // Some(Event::Resize(x, y)) => Action::Resize(x, y),
    //         // Some(_) => Action::Noop,
    //         // None => Action::Noop,
    //         Some(_) => {},
    //         None => {}
    //     }
    // }

    fn handle_key_events(&mut self, _key: KeyEvent) {}

    // fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
    //   Action::Noop
    // }
    // fn update(&mut self, action: Action) -> Action {
    //   Action::Noop
    // }

    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// * `area` - The area in which the component should be drawn.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()>;
}
