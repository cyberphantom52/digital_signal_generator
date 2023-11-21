use crate::analog::Parameters;
use crate::analog::{modulation::DM, AnalogSettings, AnalogSignal};
use crate::digital::{encoding::*, scramble::Scrambling, DigitalSettings};
use nannou::prelude::{pt2, Draw, Rect, GRAY};

#[derive(PartialEq)]
pub enum SignalType {
    Analog,
    Digital,
}

pub struct Settings {
    pub digital: DigitalSettings,
    pub analog: AnalogSettings,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            digital: DigitalSettings {
                binary_stream: String::new(),
                longest_palindrome: String::new(),
                result: Vec::new(),
                encoding: Box::new(NRZL),
                scrambling: Scrambling::None,
            },
            analog: AnalogSettings {
                analog_signal: AnalogSignal::Sine,
                result: Vec::new(),
                modulation: Box::new(DM),
                parameters: Parameters {
                    amplitude: 100.0,
                    frequency: 0.00125,
                    delta: 1.0,
                    sampling_rate: 1.0,
                },
            },
        }
    }
}

pub fn _validate_input(input: &str) -> bool {
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

pub fn longest_palindrome(s: String) -> String {
    let n = s.len();

    let s: Vec<u8> = "^"
        .bytes()
        .chain(s.into_bytes())
        .flat_map(|u| [u, b'#'])
        .chain("$".bytes())
        .collect();

    let mut center = 2;
    let mut right = 3;
    let mut p = vec![0; 2 * n + 1];
    p[2] = 1;
    let mut max_p = 1;
    let mut max_p_index = 2;

    for i in 3..=2 * n {
        if i < right {
            let i_mirror = 2 * center - i;
            p[i] = p[i_mirror].min(right - i);
        }

        while s[i + p[i] + 1] == s[i - p[i] - 1] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }

        if p[i] > max_p {
            max_p = p[i];
            max_p_index = i;
        }
    }

    s.into_iter()
        .skip(max_p_index - max_p + 1)
        .take(2 * max_p - 1)
        .filter(|u| *u != b'#')
        .map(|u| u as char)
        .collect()
}
