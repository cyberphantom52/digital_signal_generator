use std::fmt::Debug;

use super::{AnalogSettings, AnalogSignal};
use crate::Model;
use nannou::prelude::{pt2, App, Draw, ORANGE, PI, STEELBLUE};

pub trait Modulate: Debug {
    fn signal(&self, x: f32, settings: &AnalogSettings) -> f32 {
        let signal: Box<dyn Fn(f32) -> f32> = match settings.analog_signal {
            AnalogSignal::Sine => {
                Box::new(|x: f32| settings.parameters.amplitude * (2.0 * PI * settings.parameters.frequency * x).sin())
            }
            AnalogSignal::SawTooth => Box::new(|x: f32| {
                (settings.parameters.frequency * settings.parameters.amplitude) * (x % (1.0 / settings.parameters.frequency))
            }),
            AnalogSignal::Square => Box::new(|x: f32| {
                -(x % (1.0 / settings.parameters.frequency) - 0.5 / settings.parameters.frequency).signum() * settings.parameters.amplitude
            }),
        };

        signal(x)
    }

    fn draw_modulation(&self, model: &Model, app: &App, draw: &Draw) {
        let window = app.main_window();
        let win = window.rect();
        let width = win.w();
        let settings = &model.settings.analog;

        /*Draw the signal*/
        let end = win.right() - win.left();
        let mut points = Vec::with_capacity((2.0 * end) as usize);
        for x in (0..).map(|x| x as f32 / 0.5).take_while(|&x| x < end) {
            points.push((pt2(win.left() + x, self.signal(x, settings)), STEELBLUE));
        }
        draw.polyline().weight(2.0).points_colored(points);

        let encoded = &settings.result;
        let bit_length = width / encoded.len() as f32;
        let mut height = 0.0;
        let points = encoded.iter().enumerate().flat_map(|(i, &x)| {
            height += x as f32 * settings.parameters.delta;
            let start = pt2(win.left() + bit_length * i as f32, height);
            let end = pt2(win.left() + bit_length * (i + 1) as f32, height);
            [(start, ORANGE), (end, ORANGE)]
        });

        draw.polyline().weight(1.0).points_colored(points);
    }

    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct PCM;
#[derive(Debug)]
pub struct DM;

impl Modulate for DM {
    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        let mut result = Vec::new();
        let mut cursor = 0.0;
        for iteraror in (0..).map(|i| i as f32 / settings.parameters.sampling_rate).take_while(|&x| x < to) {
            let sample = self.signal(iteraror, settings);
            let bit = if sample > cursor { 1 } else { -1 };
            result.push(bit);
            cursor += (bit as f32) * settings.parameters.delta;
        }
        result
    }
}

impl Modulate for PCM {}
