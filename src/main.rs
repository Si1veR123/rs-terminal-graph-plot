use terminal_graph_plot::terminal_graph::TerminalGraph;
use terminal_graph_plot::widgets::GraphFunctionEquationValue;
use math_parser::equation::{EquationEval, EquationValue};

use std::io;
use std::env;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size},
    style::{SetBackgroundColor, Color},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    cursor
};

fn main() -> Result<(), io::Error> {
    let mut equations: Vec<EquationValue> = vec![];

    env::args().skip(1).for_each( |function| {
        equations.push(EquationValue::from(function))
    });

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
    //     |x| (x*0.03).exp()*15.0
    // );

    let mut graph = TerminalGraph::default_axes();

    let possible_chars = ['#', '@', 'o', '+', '$', '%'];
    let mut current_char = 0;

    for eq in equations {
        let mut widg = GraphFunctionEquationValue::with_equation(eq);
        widg.set_char(possible_chars[current_char]);
        current_char = (current_char + 1) % 6;
        graph.widgets.push(Box::new(widg));
    }

    let _r = graph.draw(&mut stdout, current_size).expect("couldnt draw graph");

    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // quit
                break
            },

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // left
                graph.set_offset((graph.screen_offset.0 + 1, graph.screen_offset.1));
            },

            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // up
                graph.set_offset((graph.screen_offset.0, graph.screen_offset.1 - 1));
            },

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // right
                graph.set_offset((graph.screen_offset.0 - 1, graph.screen_offset.1));
            },

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // down
                graph.set_offset((graph.screen_offset.0, graph.screen_offset.1 + 1));
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char('='),
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // zoom in
                graph.set_scale(graph.screen_scale + 1);
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char('-'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // zoom out
                graph.set_scale(graph.screen_scale - 1);
            },

            _ => ()
        }

        graph.draw(&mut stdout, current_size).expect("couldnt draw graph");
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,
    )?;
    Ok(())
}
