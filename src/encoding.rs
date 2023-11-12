use nannou::prelude::{pt2, Draw, Rect, STEELBLUE};
use crate::Model;
use crate::scramble::{b8zs_encoder, hdb3_encoder};
pub trait Encoding {
    fn encode(&self,model: &Model, window: &Rect, draw: &Draw);
}

fn encode_helper(window: &Rect, encoded_data: &Vec<usize>, draw: &Draw) {
    let width = window.w();
    let bit_length = width / encoded_data.len() as f32;
    let mut previous_end = pt2(window.left(), -0.0);
    for (i, &encoded_bit) in encoded_data.iter().enumerate() {
        let height = if encoded_bit == 0 { -50.0 } else { 50.0 };
        let start = pt2(window.left() + bit_length * i as f32, height);
        let end = pt2(window.left() + bit_length * (i + 1) as f32, height);
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
pub trait Scrambling {
    fn scramble(&self, binary_stream: &String) -> String {
        binary_stream.to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Encodings {
    NRZL,
    NRZI,
    Manchester,
    ManchesterDifferential,
    AMI,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scramblings {
    B8ZS,
    HDB3,
}

pub struct NRZL;
pub struct NRZI;
pub struct Manchester;
pub struct ManchesterDifferential;
pub struct AMI;

fn nrzi_encoder(bin_data: &Vec<usize>) -> Vec<usize> {
    let mut encoded_data = Vec::new();
    let mut prev = 0;
    for &bit in bin_data.iter() {
        if bit == 1 {
            prev = 1 - prev;
        }
        encoded_data.push(prev);
    }
    encoded_data
}

impl Encoding for NRZI {
    fn encode(&self, model: &Model, window: &Rect, draw: &Draw) {
        let binary_data: Vec<usize> = model.binary_stream
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let encoded_data = nrzi_encoder(&binary_data);

        encode_helper(window, &encoded_data, draw);
    }
}

impl Encoding for NRZL {
    fn encode(&self,model: &Model, window: &Rect,draw: &Draw) {
        let width = window.w();
        let bit_length = width / model.binary_stream.len() as f32;
        let mut previous_end = pt2(window.left(), -0.0);
        for (i, c) in model.binary_stream.chars().enumerate() {
            let height = if c == '0' { -50.0 } else { 50.0 };
            let start = pt2(window.left() + bit_length * i as f32, height);
            let end = pt2(window.left() + bit_length * (i + 1) as f32, height);
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
}

fn manchester_encoder(bin_data: &Vec<usize>) -> Vec<usize> {
    let mut encoded_data = Vec::new();
    for &bit in bin_data.iter() {
        if bit == 0 {
            encoded_data.push(0);
            encoded_data.push(1);
        } else {
            encoded_data.push(1);
            encoded_data.push(0);
        }
    }
    encoded_data
}

impl Encoding for Manchester {
    fn encode(&self,model: &Model, window: &Rect, draw: &Draw) {
        let binary_data: Vec<usize> = model.binary_stream
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let encoded_data = manchester_encoder(&binary_data);
        encode_helper(window, &encoded_data, draw);
    }
}

fn manchester_differential_encoder(bin_data: &Vec<usize>) -> Vec<usize> {
    let mut encoded_data = Vec::new();
    let mut prev = 0;

    for &bit in bin_data.iter() {
        if bit == 0 {
            encoded_data.push(prev);
            encoded_data.push(!prev);
        } else {
            encoded_data.push(!prev);
            encoded_data.push(prev);
            prev = !prev;
        }
    }

    encoded_data
}

impl Encoding for ManchesterDifferential {
    fn encode(&self,model: &Model, window: &Rect, draw: &Draw) {
        let binary_data: Vec<usize> = model.binary_stream
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let encoded_data = manchester_differential_encoder(&binary_data);
        encode_helper(window, &encoded_data, draw);
    }
}

fn ami_encoder(bin_data: &Vec<usize>) -> Vec<i32> {
    let mut encoded_data = Vec::new();
    let mut prev_polarity = 1;

    for &bit in bin_data.iter() {
        if bit == 0 {
            encoded_data.push(0);
        } else {
            encoded_data.push(if prev_polarity == 1 { 1 } else { -1 });
            prev_polarity = -prev_polarity;
        }
    }

    encoded_data
}

impl Encoding for AMI {
    fn encode(&self,model: &Model, window: &Rect, draw: &Draw) {
        let width = window.w();
        let mut previous_end = pt2(window.left(), -0.0);

        let binary_data: Vec<usize> = model.binary_stream
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let encoded_data:Vec<i32>;
        if model.scrambling == true {
            if model.scrambling_type == Scramblings::B8ZS {
                encoded_data = b8zs_encoder(&binary_data);
            }
            else {
               encoded_data = hdb3_encoder(&binary_data); 
            }
        }
        else {
            encoded_data = ami_encoder(&binary_data);
        }
        let bit_length = width / encoded_data.len() as f32;

        for (i, &encoded_bit) in encoded_data.iter().enumerate() {
            let height = if encoded_bit == 0 {
                0.0
            } else if encoded_bit == -1 {
                -50.0
            } else {
                50.0
            };
            let start = pt2(window.left() + bit_length * i as f32, height);
            let end = pt2(window.left() + bit_length * (i + 1) as f32, height);

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
}
