use std::fmt::Debug;

use super::{AnalogSettings, AnalogSignal};
use crate::Model;
use nannou::prelude::{pt2, App, Draw, ORANGE, PI, STEELBLUE};

pub trait Modulate: Debug {
    fn signal(&self, x: f32, settings: &AnalogSettings) -> f32 {
        let signal: Box<dyn Fn(f32) -> f32> = match settings.analog_signal {
            AnalogSignal::Sine => Box::new(|x: f32| {
                settings.parameters.amplitude * (2.0 * PI * settings.parameters.frequency * x).sin()
            }),
            AnalogSignal::SawTooth => Box::new(|x: f32| {
                (settings.parameters.frequency * settings.parameters.amplitude)
                    * (x % (1.0 / settings.parameters.frequency))
            }),
            AnalogSignal::Square => Box::new(|x: f32| {
                -(x % (1.0 / settings.parameters.frequency) - 0.5 / settings.parameters.frequency)
                    .signum()
                    * settings.parameters.amplitude
            }),
            AnalogSignal::Sinc => Box::new(|x: f32| {
                let i = 2.0 * PI * settings.parameters.frequency * (x - 400.0); // shift by 400 to show more of the sinc function
                settings.parameters.amplitude * (i).sin() / i
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
        for x in (0..).map(|x| x as f32).take_while(|&x| x < end) {
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

    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8>;
}

#[derive(Debug)]
pub struct PCM;
#[derive(Debug)]
pub struct DM;

impl PCM {
    fn nearest_pow_of_two(&self, n: f32) -> i32 {
        // Ref: https://graphics.stanford.edu/%7Eseander/bithacks.html#RoundUpPowerOf2
        let flag = if n >= 0.0 { 1 } else { -1 };
        let mut i = n.abs() as i32;
        i += (i == 0) as i32;
        i -= 1;
        i |= i >> 1;
        i |= i >> 2;
        i |= i >> 4;
        i |= i >> 8;
        i |= i >> 16;
        i += 1;
        let p = i >> 1;
        if n - p as f32 > i as f32 - n {
            flag * i
        } else {
            flag * p
        }
    }
}
impl Modulate for DM {
    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        let mut result = Vec::new();
        let mut cursor = 0.0;
        for iteraror in (0..)
            .map(|i| i as f32 / settings.parameters.sampling_rate)
            .take_while(|&x| x < to)
        {
            let sample = self.signal(iteraror, settings);
            let bit = if sample > cursor { 1 } else { -1 };
            result.push(bit);
            cursor += (bit as f32) * settings.parameters.delta;
        }
        result
    }
}

impl Modulate for PCM {
    fn modulate(&self, settings: &AnalogSettings, to: f32) -> Vec<i8> {
        let mut result = Vec::new();
        //assume 8 bits;
        let max_level = 2i32.pow(8);
        for iteraror in (0..)
            .map(|i| i as f32 / settings.parameters.sampling_rate)
            .take_while(|&x| x < to)
        {
            let sample = self.signal(iteraror, settings);
            let mut i = self.nearest_pow_of_two(sample);
            if i.abs() > max_level {
                i = (i / i.abs()) * max_level;
            }
            format!("{:08b}", i)
                .chars()
                .for_each(|c| result.push(c.to_digit(2).unwrap() as i8));
        }
        result
    }
}
