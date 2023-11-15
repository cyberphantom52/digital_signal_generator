use super::scramble::{Scramble, Scrambling};
use super::DigitalSettings;
use nannou::prelude::{pt2, App, Draw, STEELBLUE};
use std::fmt::Debug;

pub trait Encode: Debug {
    fn draw_encoding(&self, encoded: &Vec<i8>, app: &App, draw: &Draw) {
        let window = app.main_window();
        let win = window.rect();
        let width = win.w();

        let bit_length = width / encoded.len() as f32;
        let points = encoded.iter().enumerate().flat_map(|(i, &x)| {
            let height = x as f32 * 50.0;
            let start = pt2(win.left() + bit_length * i as f32, height);
            let end = pt2(win.left() + bit_length * (i + 1) as f32, height);
            [(start, STEELBLUE), (end, STEELBLUE)]
        });

        draw.polyline().weight(4.0).points_colored(points);
    }

    fn encode(&self, settings: &DigitalSettings) -> Vec<i8>;
}

#[derive(Debug)]
pub struct NRZL;
#[derive(Debug)]
pub struct NRZI;
#[derive(Debug)]
pub struct Manchester;
#[derive(Debug)]
pub struct ManchesterDifferential;
#[derive(Debug)]
pub struct AMI;

impl Encode for NRZL {
    fn encode(&self, settings: &DigitalSettings) -> Vec<i8> {
        let data = &settings.binary_stream;
        let mut result = Vec::with_capacity(data.len());
        data.chars().for_each(|c| {
            result.push(match c {
                '0' => -1,
                '1' => 1,
                _ => panic!("Invalid input"),
            });
        });
        result
    }
}

impl Encode for NRZI {
    fn encode(&self, settings: &DigitalSettings) -> Vec<i8> {
        let data = &settings.binary_stream;
        let mut encoded_data = Vec::with_capacity(data.len());
        let mut toggle = 1;
        data.chars().for_each(|c| {
            if c == '1' {
                toggle *= -1;
            }
            encoded_data.push(toggle);
        });
        encoded_data
    }
}

impl Encode for Manchester {
    fn encode(&self, settings: &DigitalSettings) -> Vec<i8> {
        let data = &settings.binary_stream;
        let mut encoded_data = Vec::with_capacity(data.len() * 2);
        data.chars().for_each(|c| {
            encoded_data.extend_from_slice(match c {
                '1' => &[-1, 1],
                '0' => &[1, -1],
                _ => panic!("Invalid input"),
            });
        });
        encoded_data
    }
}

impl Encode for ManchesterDifferential {
    fn encode(&self, settings: &DigitalSettings) -> Vec<i8> {
        let data = &settings.binary_stream;
        let mut encoded_data = Vec::with_capacity(data.len() * 2);
        let seq = [[1, -1], [-1, 1]];
        let mut toggle = 0;
        data.chars().for_each(|c| {
            if c == '1' {
                toggle = 1 - toggle;
            }
            encoded_data.extend_from_slice(&seq[toggle]);
        });
        encoded_data
    }
}

impl Encode for AMI {
    fn encode(&self, settings: &DigitalSettings) -> Vec<i8> {
        let data = &settings.binary_stream;
        let encoded_data = match settings.scrambling {
            Scrambling::None => {
                let mut toggle = 1;
                data.chars().fold(Vec::new(), |mut acc, c| {
                    match c {
                        '0' => acc.push(0),
                        '1' => {
                            acc.push(toggle);
                            toggle *= -1;
                        }
                        _ => {}
                    };
                    acc
                })
            }
            _ => self.scramble(data, settings.scrambling),
        };

        encoded_data
    }
}
