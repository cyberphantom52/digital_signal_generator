use nannou::prelude::{pt2, Rect, GRAY};
use nannou::Draw;

use crate::analog::AnalogSignal;
use crate::analog::modulation::Modulation;
use crate::digital::encoding::Encoding;
use crate::digital::scramble::Scrambling;

#[derive(PartialEq)]
pub enum SignalType {
    Analog,
    Digital,
}

pub struct Settings {
    pub digital: DigitalSettings,
    pub analog: AnalogSettings,
}

pub struct DigitalSettings {
    pub binary_stream: String,
    pub encoding: Encoding,
    pub scrambling: Scrambling,
}

pub struct AnalogSettings {
    pub analog_signal: AnalogSignal,
    pub modulation: Modulation,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            digital: DigitalSettings {
                binary_stream: String::new(),
                encoding: Encoding::NRZL,
                scrambling: Scrambling::None,
            },
            analog: AnalogSettings {
                analog_signal: AnalogSignal::Sine,
                modulation: Modulation::PCM,
            },
        }
    }
}
pub fn validate_input(input: &str) -> bool {
    input.chars().into_iter().all(|x| x == '0' || x == '1')
}

pub fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()))
            .color(GRAY);
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y))
            .color(GRAY);
    }
}
