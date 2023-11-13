pub mod encoding;
pub mod scramble;

use super::{Encoding, Scrambling};
use crate::utils::DigitalSettings;
use nannou_egui::egui;

pub fn draw_ui(ui: &mut egui::Ui, settings: &mut DigitalSettings) {
    ui.vertical(|ui| {
        ui.label("Binary Message:");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut settings.binary_stream);
    });

    ui.vertical(|ui| {
        ui.label("Encoding:");
        ui.add_space(5.0);
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", settings.encoding))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.encoding, Encoding::NRZL, "NRZ-L");
                ui.selectable_value(&mut settings.encoding, Encoding::NRZI, "NRZ-I");
                ui.selectable_value(&mut settings.encoding, Encoding::Manchester, "Manchester");
                ui.selectable_value(
                    &mut settings.encoding,
                    Encoding::ManchesterDifferential,
                    "Differential Manchester",
                );
                ui.selectable_value(&mut settings.encoding, Encoding::AMI, "AMI");
            });
    });

    if settings.encoding == Encoding::AMI {
        ui.vertical(|ui| {
            ui.label("Scrambling:");
            ui.add_space(5.0);
            egui::ComboBox::from_label(" ")
                .selected_text(format!("{:?}", settings.scrambling))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.scrambling, Scrambling::None, "None");
                    ui.selectable_value(&mut settings.scrambling, Scrambling::B8ZS, "B8ZS");
                    ui.selectable_value(&mut settings.scrambling, Scrambling::HDB3, "HDB3");
                });
        });
    }
}
