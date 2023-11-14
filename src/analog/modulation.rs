use super::{AnalogSettings, AnalogSignal};
use crate::Model;
use nannou::prelude::{pt2, App, Draw, ORANGE, PI, STEELBLUE};

pub trait Modulate {
    fn draw_modulation(&self, model: &Model, app: &App, draw: &Draw) {
        let window = app.main_window();
        let win = window.rect();
        let width = win.w();
        let settings = &model.settings.analog;

        let signal: Box<dyn Fn(f32) -> f32> = match settings.analog_signal {
            AnalogSignal::Sine => {
                Box::new(|x: f32| settings.amplitude * (2.0 * PI * settings.frequency * x).sin())
            }
            AnalogSignal::SawTooth => Box::new(|x: f32| {
                let f = 2.0 * 100.0 * settings.frequency;
                (settings.amplitude / f) * (x % f)
            }),
        };

        /*Draw the signal*/
        let mut x = signal(0.0);
        let end = win.right() - win.left();
        let mut points = Vec::with_capacity((2.0 * end) as usize);
        while end > x {
            points.push(pt2(win.left() + x, signal(x)));
            x += 0.5;
        }
        draw.polyline()
            .weight(2.0)
            .points_colored(points.into_iter().map(|x| (x, STEELBLUE)));

        let encoded = &settings.result;
        let bit_length = width / encoded.len() as f32;
        let mut previous_end = pt2(win.left(), -0.0);
        let mut height = 0.0;
        for (i, &c) in encoded.iter().enumerate() {
            height += c as f32 * settings.delta;
            let start = pt2(win.left() + bit_length * i as f32, height);
            let end = pt2(win.left() + bit_length * (i + 1) as f32, height);

            if previous_end != start {
                draw.line()
                    .start(previous_end)
                    .end(start)
                    .weight(1.0)
                    .color(ORANGE);
            }
            previous_end = end;
            draw.line().start(start).end(end).weight(1.0).color(ORANGE);
        }
    }

    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modulation {
    PCM,
    DM,
}

pub struct PCM;
pub struct DM;

impl Modulate for DM {
    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        let signal: Box<dyn Fn(f32) -> f32> = match settings.analog_signal {
            AnalogSignal::Sine => {
                Box::new(|x: f32| settings.amplitude * (2.0 * PI * settings.frequency * x).sin())
            }
            AnalogSignal::SawTooth => Box::new(|x: f32| {
                let f = 2.0 * 100.0 * settings.frequency;
                (settings.amplitude / f) * (x % f)
            }),
        };
        let mut result = Vec::new();
        let mut cursor = signal(0.0);
        let mut iteraror = 0.0;
        while iteraror < to {
            let sample = signal(iteraror);
            if sample > cursor {
                result.push(1);
                cursor += settings.delta;
            } else {
                result.push(-1);
                cursor -= settings.delta;
            }
            iteraror += 1.0 / settings.sampling_rate;
        }
        result
    }
}
impl Modulate for PCM {}
