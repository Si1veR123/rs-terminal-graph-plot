use std::io::{Write, self};

type Coord = (isize, isize);

pub trait Widget {
    fn pre_draw(&mut self, size: &(usize, usize), scale: f32, offset: &Coord);
    fn char_at(&self, coord: Coord, terminal_coord: Coord) -> Option<char>;
}


pub struct TerminalScreen {
    widgets: Vec<Box<dyn Widget>>,
    screen_offset: Coord,
    screen_scale: i8,
}

impl TerminalScreen {
    // terminal space: (0, 0) is top left
    //               : +x is right
    //               : +y is down
    //    graph space: (0, 0) is axes center (has offset)
    //               : +x is right
    //               : +y is up

    pub fn default_axes() -> Self {
        Self { widgets: vec![Box::new(Axes {center: (0, 0)})], screen_offset: (0, 0), screen_scale: 0 }
    }

    fn get_scale_factor(&self) -> f32 {
        // (1/2)^scale = scale factor
        // -1 is zooming out, 0 is normal, 1 is zooming in
        0.5f32.powi(self.screen_scale.into())
    }

    fn terminal_space_to_graph_space(terminal_space: &Coord, center: &Coord, scale: f32) -> Coord {
        // convert a coord from terminal to graph space
        // center is in terminal space
        (
            ((terminal_space.0 - center.0) as f32 * scale) as isize,
            -(((terminal_space.1 - center.1) as f32 * scale) as isize)
        )
    }

    fn graph_space_to_terminal_space(graph_space: &Coord, center: &Coord, scale: f32) -> Coord {
        (
            ((graph_space.0 as f32)/scale) as isize + center.0,
            ((-graph_space.1 as f32)/scale) as isize + center.1
        )
    }

    fn get_screen_center(size: &(usize, usize), offset: &Coord) -> Coord {
        // given a terminal size and offset, return the center in terminal space
        (((size.0) as isize / 2)+offset.0, ((size.1 as isize) / 2)-offset.1)
    }

    pub fn draw<T: Write>(&mut self, terminal: &mut T, size: (usize, usize)) -> Result<(), io::Error> {
        let scale_factor = self.get_scale_factor();

        for widget in &mut self.widgets {
            widget.pre_draw(&size, scale_factor, &self.screen_offset);
        }

        // +1 for new line chars
        let mut buffer = String::with_capacity(size.0 * (size.1 + 1));

        // in terminal space
        let center = TerminalScreen::get_screen_center(&size, &self.screen_offset);

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
        let graph_widg = GraphFunction {
            function,
            values: None,
            line_char: line_char.unwrap_or('#')
        };

        self.widgets.insert(0, Box::new(graph_widg));
    }
}

impl Widget for TerminalScreen {
    fn pre_draw(&mut self, _size: &(usize, usize), _scale: f32, _offset: &Coord) {}

    fn char_at(&self, coord: Coord, terminal_coord: Coord) -> Option<char> {
        for widget in &self.widgets {
            if let Some(c) = widget.char_at(coord.clone(), terminal_coord.clone()) {
                return Some(c)
            }
        }
        return None
    }
}

struct Axes {
    center: Coord
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


struct GraphFunction {
    function: &'static dyn Fn(f64) -> f64,
    line_char: char,
    values: Option<Vec<isize>>
}

impl Widget for GraphFunction {
    fn pre_draw(&mut self, size: &(usize, usize), scale: f32, offset: &Coord) {
        // evaluate at each terminal x value
        let center = TerminalScreen::get_screen_center(size, offset);

        let mut temp_x = vec![];
        for x_i in 0..size.0 as isize {
            // convert from terminal to graph space, calculate, then convert back
            let x = TerminalScreen::terminal_space_to_graph_space(&(x_i, 0), &center, scale).0;
            let mut y = (self.function)(x as f64);
            if y.is_nan() | y.is_infinite() {
                y = f64::INFINITY;
            }
            temp_x.push(TerminalScreen::graph_space_to_terminal_space(&(0, y as isize), &center, scale).1)
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
