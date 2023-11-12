mod encoding;
mod scramble;
mod utils;

use crate::encoding::*;
use crate::scramble::Scrambling;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use utils::draw_grid;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    ui: Egui,
    binary_stream: String,
    encoding: Encoding,
    scrambling: Scrambling,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .size(800, 800)
        .resizable(false)
        .raw_event(raw_ui_event)
        .build()
        .unwrap();

    let window_id = app.window(main_window).unwrap();
    let ui = Egui::from_window(&window_id);

    Model {
        ui,
        binary_stream: "0".to_string(),
        encoding: Encoding::NRZL,
        scrambling: Scrambling::None,
    }
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.ui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Simulator Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Binary Message:");
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut model.binary_stream);
            });

            let current_encoding = model.encoding;
            ui.vertical(|ui| {
                ui.label("Encoding:");
                ui.add_space(5.0);
                egui::ComboBox::from_label("")
                    .selected_text(format!("{current_encoding:?}"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut model.encoding, Encoding::NRZL, "NRZ-L");
                        ui.selectable_value(&mut model.encoding, Encoding::NRZI, "NRZ-I");
                        ui.selectable_value(
                            &mut model.encoding,
                            Encoding::Manchester,
                            "Manchester",
                        );
                        ui.selectable_value(
                            &mut model.encoding,
                            Encoding::ManchesterDifferential,
                            "Differential Manchester",
                        );
                        ui.selectable_value(&mut model.encoding, Encoding::AMI, "AMI");
                    });
            });

            if model.encoding == Encoding::AMI {
                let current_scrambling = model.scrambling;
                ui.vertical(|ui| {
                    ui.label("Scrambling:");
                    ui.add_space(5.0);
                    egui::ComboBox::from_label(" ")
                        .selected_text(format!("{current_scrambling:?}"))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut model.scrambling,
                                Scrambling::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut model.scrambling,
                                Scrambling::B8ZS,
                                "B8ZS",
                            );
                            ui.selectable_value(
                                &mut model.scrambling,
                                Scrambling::HDB3,
                                "HDB3",
                            );
                        });
                });
            }
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);

    match model.encoding {
        Encoding::NRZI => NRZI.draw_encoding(&model, &app, &draw),
        Encoding::NRZL => NRZL.draw_encoding(&model, &app, &draw),
        Encoding::Manchester => Manchester.draw_encoding(&model, &app, &draw),
        Encoding::ManchesterDifferential => {
            ManchesterDifferential.draw_encoding(&model, &app, &draw)
        }
        Encoding::AMI => AMI.draw_encoding(&model, &app, &draw),
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}
