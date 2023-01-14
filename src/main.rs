mod tui;
mod macros;

use std::io;
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size},
    style::{SetBackgroundColor, Color}
};

fn main() -> Result<(), io::Error> {
    let _ = enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetBackgroundColor(
            Color::Rgb { r: 6, g: 6, b: 30 }
        )
    )?;

    let current_size = size().expect("couldnt get size");
    display_graph!(
        current_size,
        |x| (0.7*(200.0 - x.powi(2)).sqrt())-15.0,
        |x| (-0.7*(200.0 - x.powi(2)).sqrt())-15.0,
        |x| (x*0.03).exp()*15.0,
        |x| (x*0.1).sin()*15.0
    );

    thread::sleep(Duration::from_secs(5));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
