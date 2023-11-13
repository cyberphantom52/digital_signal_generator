pub mod encoding;
pub mod scramble;

use super::*;
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
        egui::ComboBox::from_id_source(0)
            .selected_text(format!("{:?}", settings.encoding))
            .show_ui(ui, |ui| {
                for encoding in vec!["NRZ-L", "NRZ-I", "Manchester", "Differential Manchester", "AMI"] {
                    if ui.selectable_label(false, encoding).clicked() {
                        settings.encoding = match encoding {
                            "NRZ-L" => Box::new(NRZL),
                            "NRZ-I" => Box::new(NRZI),
                            "Manchester" => Box::new(Manchester),
                            "Differential Manchester" => Box::new(ManchesterDifferential),
                            "AMI" => Box::new(AMI),
                            _ => unimplemented!(),
                        };
                    
                    }
                }
            });
    });

    if format!("{:?}", settings.encoding) == "AMI" {
        ui.vertical(|ui| {
            ui.label("Scrambling:");
            ui.add_space(5.0);
            egui::ComboBox::from_id_source(1)
                .selected_text(format!("{:?}", settings.scrambling))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.scrambling, Scrambling::None, "None");
                    ui.selectable_value(&mut settings.scrambling, Scrambling::B8ZS, "B8ZS");
                    ui.selectable_value(&mut settings.scrambling, Scrambling::HDB3, "HDB3");
                });
        });
    }
}
