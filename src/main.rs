mod widgets;
mod terminal_graph;
mod macros;

use std::io;
use std::thread;
use std::time::Duration;

use crossterm::{
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
        SetBackgroundColor(
            Color::Rgb { r: 6, g: 6, b: 30 }
        )
    )?;

    let current_size = size().expect("couldnt get size");
    let current_size = (current_size.0 as usize, current_size.1 as usize);

    // macro for easier use
    // display_graph!(
    //     current_size,
    //     |x| (x*0.03).exp()*15.0,
    // );

    let mut graph = terminal_graph::TerminalGraph::default_axes();
    graph.add_graph(&|x| (x*0.03).exp()*15.0, Some('#'));
    graph.add_graph(
        &|x| (x*0.1).sin() * 10.0, 
        Some('@'));
    let _r = graph.draw(&mut stdout, current_size).expect("couldnt draw graph");

    thread::sleep(Duration::from_secs(5));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,
    )?;
    Ok(())
}
