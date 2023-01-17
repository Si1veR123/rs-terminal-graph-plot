use super::widgets::{Widget, GraphFunction, Axes};
use std::io::{self, Write};

pub type Coord = (isize, isize);

pub struct TerminalGraph {
    pub widgets: Vec<Box<dyn Widget>>,
    pub screen_offset: Coord,
    pub screen_scale: i8,
}

impl TerminalGraph {
    // a widget that draws all other widgets
    // terminal space: (0, 0) is top left
    //               : +x is right
    //               : +y is down
    //    graph space: (0, 0) is axes center (can have offset)
    //               : +x is right
    //               : +y is up

    pub fn default_axes() -> Self {
        Self { widgets: vec![Box::new(Axes::default())], screen_offset: (0, 0), screen_scale: 0 }
    }

    pub fn set_offset(&mut self, offset: Coord) {
        self.screen_offset = offset
    }

    pub fn set_scale(&mut self, scale: i8) {
        self.screen_scale = scale
    }

    pub fn get_scale_factor(&self) -> f32 {
        // (1/2)^scale = scale factor
        // -1 is zooming out, 0 is normal, 1 is zooming in
        0.5f32.powi(self.screen_scale.into())
    }

    pub fn terminal_space_to_graph_space(terminal_space: &Coord, center: &Coord, scale: f32) -> Coord {
        // convert a coord from terminal to graph space
        // center is in terminal space
        (
            ((terminal_space.0 - center.0) as f32 * scale) as isize,
            -(((terminal_space.1 - center.1) as f32 * scale) as isize)
        )
    }

    pub fn graph_space_to_terminal_space(graph_space: &Coord, center: &Coord, scale: f32) -> Coord {
        (
            ((graph_space.0 as f32)/scale) as isize + center.0,
            ((-graph_space.1 as f32)/scale) as isize + center.1
        )
    }

    pub fn get_screen_center(size: &(usize, usize), offset: &Coord) -> Coord {
        // given a terminal size and offset, return the center in terminal space
        (((size.0) as isize / 2)+offset.0, ((size.1 as isize) / 2)-offset.1)
    }

    pub fn draw<T: Write>(&mut self, terminal: &mut T, size: (usize, usize)) -> Result<(), io::Error> {
        let scale_factor = self.get_scale_factor();

        // call pre draw on all widgets
        for widget in &mut self.widgets {
            widget.pre_draw(&size, scale_factor, &self.screen_offset);
        }

        // +1 for new line chars
        let mut buffer = String::with_capacity(size.0 * (size.1 + 1));

        // in terminal space
        let center = TerminalGraph::get_screen_center(&size, &self.screen_offset);

        // iterate over all terminal spaces, if a widget wants to draw a character here, add it to buffer
        for y in 0..size.1 as isize {
            for x in 0..size.0 as isize {
                let coordinate = Self::terminal_space_to_graph_space(&(x, y), &center, scale_factor);

                if let Some(c) = self.char_at(coordinate, (x, y)) {
                    buffer.push(c)
                } else {
                    buffer.push(' ')
                }
            }
            buffer.push('\n');
        }

        write!(terminal, "{}", buffer)?;
        Ok(())
    }

    pub fn add_graph(&mut self, function: &'static dyn Fn(f64) -> f64, line_char: Option<char>) {
        let mut graph_widg = GraphFunction::with_function(function);
        graph_widg.set_char(line_char.unwrap_or('#'));

        self.widgets.insert(0, Box::new(graph_widg));
    }
}

impl Widget for TerminalGraph {
    fn pre_draw(&mut self, _size: &(usize, usize), _scale: f32, _offset: &Coord) {}

    fn char_at(&self, coord: Coord, terminal_coord: Coord) -> Option<char> {
        // iterate over all widgets, checking if they want to draw a char at the coord
        for widget in &self.widgets {
            if let Some(c) = widget.char_at(coord.clone(), terminal_coord.clone()) {
                return Some(c)
            }
        }
        return None
    }
}