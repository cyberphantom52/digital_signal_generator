use super::AnalogSignal;
use crate::Model;
use nannou::prelude::{App, Draw};

pub trait Modulate {
    fn draw_modulation(&self, model: &Model, app: &App, draw: &Draw) {
        unimplemented!()
    }

    fn modulate(&self, signal: AnalogSignal, from: i32, to: i32) -> Vec<i8> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modulation {
    PCM,
    DM,
}

pub struct PCM;
pub struct DM;

impl Modulate for DM {}
impl Modulate for PCM {}
