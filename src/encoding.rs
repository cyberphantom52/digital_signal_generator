use nannou::prelude::{pt2, Rect, Draw, STEELBLUE};
pub trait Encoding {
    fn encode(&self, window: &Rect, binary_stream: &String, draw: &Draw) {
        let width = window.w();
        let bit_length = width / binary_stream.len() as f32;
        let mut previous_end = pt2(window.left(), -0.0);
        for (i, c) in binary_stream.chars().enumerate() {
          let height = if c == '0' { -50.0 } else { 50.0 };
          let start = pt2(window.left() + bit_length * i as f32, height);
          let end = pt2(window.left() + bit_length * (i + 1) as f32, height);
          if previous_end != start {
            draw.line().start(previous_end).end(start).weight(4.0).color(STEELBLUE);
          }
          previous_end = end;
          draw.line().start(start).end(end).weight(4.0).color(STEELBLUE);
        }
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

pub enum _Scramblings {
    B8ZS,
    HDB3,
}

pub struct NRZL;
pub struct NRZI;
pub struct Manchester;
pub struct ManchesterDifferential;
pub struct AMI;

impl Encoding for NRZI {}
impl Encoding for NRZL {}
impl Encoding for Manchester {}
impl Encoding for ManchesterDifferential {}
impl Encoding for AMI {}
