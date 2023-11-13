use super::scramble::Scramble;
use crate::Model;
use nannou::prelude::{pt2, App, Draw, STEELBLUE};

pub trait Encode {
    fn draw_encoding(&self, model: &Model, app: &App, draw: &Draw) {
        let window = app.main_window();
        let win = window.rect();
        let width = win.w();

        let encoded = if model.settings.digital.encoding != Encoding::AMI {
            self.encode(&model.settings.digital.binary_stream)
        } else {
            AMI.scramble(&model.settings.digital.binary_stream, model.settings.digital.scrambling)
        };

        let bit_length = width / encoded.len() as f32;
        let mut previous_end = pt2(win.left(), -0.0);
        for (i, &c) in encoded.iter().enumerate() {
            let height = c as f32 * 50.0;
            let start = pt2(win.left() + bit_length * i as f32, height);
            let end = pt2(win.left() + bit_length * (i + 1) as f32, height);

            if previous_end != start {
                draw.line()
                    .start(previous_end)
                    .end(start)
                    .weight(4.0)
                    .color(STEELBLUE);
            }
            previous_end = end;
            draw.line()
                .start(start)
                .end(end)
                .weight(4.0)
                .color(STEELBLUE);
        }
    }

    fn encode(&self, data: &str) -> Vec<i8>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Encoding {
    NRZL,
    NRZI,
    Manchester,
    ManchesterDifferential,
    AMI,
}

pub struct NRZL;
pub struct NRZI;
pub struct Manchester;
pub struct ManchesterDifferential;
pub struct AMI;

impl Encode for NRZL {
    fn encode(&self, data: &str) -> Vec<i8> {
        let mut result = Vec::new();
        for char in data.chars().into_iter() {
            result.push(
                char.to_string()
                    .parse::<i8>()
                    .expect("Error parsing binary stream"),
            );
        }
        result
    }
}

impl Encode for NRZI {
    fn encode(&self, data: &str) -> Vec<i8> {
        let mut encoded_data = Vec::new();
        let mut state = 0;
        for bit in data.chars().into_iter() {
            match bit {
                '1' => state = 1 - state,
                _ => {}
            }
            encoded_data.push(state);
        }
        encoded_data
    }
}

impl Encode for Manchester {
    fn encode(&self, data: &str) -> Vec<i8> {
        let mut encoded_data = Vec::new();
        for bit in data.chars().into_iter() {
            let mut seq = vec![];
            match bit {
                '1' => seq = vec![0, 1],
                '0' => seq = vec![1, 0],
                _ => {}
            }
            encoded_data.append(&mut seq);
        }
        encoded_data
    }
}

impl Encode for ManchesterDifferential {
    fn encode(&self, data: &str) -> Vec<i8> {
        let mut encoded_data = Vec::new();
        let mut prev = 1;
        for bit in data.chars().into_iter() {
            match bit {
                '0' => {
                    encoded_data.push(prev);
                    encoded_data.push((prev + 1) % 2);
                }
                '1' => {
                    encoded_data.push((prev + 1) % 2);
                    encoded_data.push(prev);
                    prev = (prev + 1) % 2;
                }
                _ => {}
            }
        }

        encoded_data
    }
}

impl Encode for AMI {
    fn encode(&self, data: &str) -> Vec<i8> {
        let mut encoded_data = Vec::new();
        let mut toggle = 1;
        for bit in data.chars().into_iter() {
            match bit {
                '0' => encoded_data.push(0),
                '1' => {
                    encoded_data.push(toggle * 1);
                    toggle *= -1;
                }
                _ => {}
            }
        }
        encoded_data
    }
}
