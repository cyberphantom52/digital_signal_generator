use crate::{Model, utils::AnalogSettings};
use nannou::prelude::{pt2, App, Draw, PI, STEELBLUE};

pub trait Modulate {
    fn draw_modulation(&self, model: &Model, app: &App, draw: &Draw) {
        let window = app.main_window();
        let win = window.rect();
        let width = win.w();
        let settings = &model.settings.analog;

        let signal = |x: f32| settings.amplitude * (2.0 * PI * settings.frequency * x).sin();
        let mut iterator = signal(0.0);
        let end = win.right() - win.left();
        while end > iterator {
            let point = pt2(win.left() + iterator, signal(iterator));
            iterator += 0.5;
            draw.ellipse().color(STEELBLUE).xy(point).w_h(2.0, 2.0);
        }

        let encoded = self.modulate(signal, settings, win.right() - win.left());
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

    fn modulate<F>(&self, signal: F, settings: &AnalogSettings, to: f32) -> Vec<i8>
    where
        F: Fn(f32) -> f32,
    {
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
    fn modulate<F>(&self, signal: F, settings: &AnalogSettings, to: f32) -> Vec<i8>
    where
        F: Fn(f32) -> f32,
    {
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
