use math_parser::equation::{EquationValue, EquationEval};

use super::terminal_graph::{Coord, TerminalGraph};

pub trait Widget {
    fn pre_draw(&mut self, size: &(usize, usize), scale: f32, offset: &Coord);
    fn char_at(&self, coord: Coord, terminal_coord: Coord) -> Option<char>;
}

pub struct Axes {
    center: Coord
}

#[allow(dead_code)]
impl Axes {
    pub fn default() -> Self {
        Axes { center: (0, 0) }
    }

    pub fn with_center(center: Coord) -> Self {
        Axes { center }
    }
}

impl Widget for Axes {
    fn char_at(&self, coord: Coord, _terminal_coord: Coord) -> Option<char> {
        match ((self.center.0 == coord.0), (self.center.1 == coord.1)) {
            (false, false) => None,
            (true, false) => Some('|'),
            (false, true) => Some('―'),
            (true, true) => Some('+')
        }
    }

    fn pre_draw(&mut self, _size: &(usize, usize), _scale: f32, _offset: &Coord) {}
}


pub struct GraphFunction {
    function: &'static dyn Fn(f64) -> f64,
    line_char: char,
    values: Option<Vec<isize>>
}

impl GraphFunction {
    pub fn with_function(function: &'static dyn Fn(f64) -> f64) -> Self {
        GraphFunction { function, line_char: '#', values: None }
    }
    pub fn set_char(&mut self, c: char) {
        self.line_char = c
    }
}

impl Widget for GraphFunction {
    fn pre_draw(&mut self, size: &(usize, usize), scale: f32, offset: &Coord) {
        // evaluate at each terminal x value
        let center = TerminalGraph::get_screen_center(size, offset);

        let mut temp_x = vec![];
        for x_i in 0..size.0 as isize {
            // convert from terminal to graph space, calculate, then convert back
            let x = TerminalGraph::terminal_space_to_graph_space(&(x_i, 0), &center, scale).0;
            let mut y = (self.function)(x as f64);
            if y.is_nan() | y.is_infinite() {
                y = f64::INFINITY;
            }
            temp_x.push(TerminalGraph::graph_space_to_terminal_space(&(0, y as isize), &center, scale).1)
        }
        self.values = Some(temp_x);
    }

    fn char_at(&self, _coord: Coord, terminal_coord: Coord) -> Option<char> {
        if let Some(y_values) = self.values.as_ref() {
            if let Some(y) = y_values.get(terminal_coord.0 as usize) {
                if y == &terminal_coord.1 {
                    return Some(self.line_char)
                }
            }
        }
        None
    }
}

pub struct GraphFunctionEquationValue {
    eq: EquationValue,
    line_char: char,
    values: Option<Vec<isize>>
}

impl GraphFunctionEquationValue {
    pub fn with_equation(eq: EquationValue) -> Self {
        GraphFunctionEquationValue { eq, line_char: '#', values: None }
    }
    pub fn set_char(&mut self, c: char) {
        self.line_char = c
    }
}

impl Widget for GraphFunctionEquationValue {
    fn pre_draw(&mut self, size: &(usize, usize), scale: f32, offset: &Coord) {
        // evaluate at each terminal x value
        let center = TerminalGraph::get_screen_center(size, offset);

        let mut temp_x = vec![];
        for x_i in 0..size.0 as isize {
            // convert from terminal to graph space, calculate, then convert back
            let x = TerminalGraph::terminal_space_to_graph_space(&(x_i, 0), &center, scale).0;
            let mut y = self.eq.evaluate(Some(x as f64));
            if y.is_nan() | y.is_infinite() {
                y = f64::INFINITY;
            }
            temp_x.push(TerminalGraph::graph_space_to_terminal_space(&(0, y as isize), &center, scale).1)
        }
        self.values = Some(temp_x);
    }

    fn char_at(&self, _coord: Coord, terminal_coord: Coord) -> Option<char> {
        if let Some(y_values) = self.values.as_ref() {
            if let Some(y) = y_values.get(terminal_coord.0 as usize) {
                if y == &terminal_coord.1 {
                    return Some(self.line_char)
                }
            }
        }
        None
    }
}
