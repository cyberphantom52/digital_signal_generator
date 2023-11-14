pub mod modulation;

use self::modulation::{Modulate, DM, PCM};
use crate::utils::{Settings, SignalType};
use nannou::prelude::App;
use nannou_egui::egui;

#[derive(PartialEq)]
pub enum AnalogSignal {
    Sine,
    SawTooth,
}

pub struct AnalogSettings {
    pub analog_signal: AnalogSignal,
    pub result: Vec<i8>,
    pub modulation: Box<dyn Modulate>,
    pub amplitude: f32,
    pub frequency: f32,
    pub delta: f32,
    pub sampling_rate: f32,
}

pub fn draw_ui(app: &App, ui: &mut egui::Ui, signal_type: &mut SignalType, s: &mut Settings) {
    let win = app.main_window().rect();
    let settings = &mut s.analog;
    ui.horizontal(|ui| {
        ui.radio_value(&mut settings.analog_signal, AnalogSignal::Sine, "Sin(x)");
        ui.radio_value(
            &mut settings.analog_signal,
            AnalogSignal::SawTooth,
            "Saw Tooth",
        );
    });

    ui.add_space(5.0);
    egui::ComboBox::from_id_source(3)
        .selected_text(format!("{:?}", settings.modulation))
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "Delta Modulation").clicked() {
                settings.modulation = Box::new(DM);
            }
            if ui.selectable_label(false, "Pulse Code Modulation").clicked() {
                settings.modulation = Box::new(PCM);
            }
        });

    ui.add(egui::Slider::new(&mut settings.amplitude, -400.0..=400.0).text("Amplitude"));
    ui.add(egui::Slider::new(&mut settings.frequency, 0.001..=1.000).text("Frequency"));
    ui.add(egui::Slider::new(&mut settings.delta, 1.0..=100.0).text("Delta"));
    ui.add(egui::Slider::new(&mut settings.sampling_rate, 0.01..=3.00).text("Sampling Rate"));

    settings.result = settings.modulation.modulate(settings, win.right() - win.left());

    if ui.button("Encode").clicked() {
        s.digital.binary_stream = settings
            .result
            .iter()
            .map(|x| if *x == 1 { '1' } else { '0' })
            .collect();
        *signal_type = crate::utils::SignalType::Digital;
    }
}
