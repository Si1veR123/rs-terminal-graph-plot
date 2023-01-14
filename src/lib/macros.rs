#[macro_export]
macro_rules! display_graph {
    ($current_size:ident, $($function:expr),*) => {
        let possible_chars = ['#', '@', 'o', '+', '$', '%'];
        let mut terminal_screen = $crate::terminal_graph::TerminalGraph::default_axes();
        let mut current_char: usize = 0;
        $(
            terminal_screen.add_graph(&$function, Some(possible_chars[current_char]));
            current_char = (current_char + 1)%6;
        )*

        terminal_screen.draw(&mut std::io::stdout(), ($current_size.0 as usize, $current_size.1 as usize)).expect("couldnt draw");
    };
}