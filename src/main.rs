mod analog;
mod digital;
mod utils;

use crate::digital::{encoding::*, scramble::*};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use utils::{draw_grid, Settings, SignalType};

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    ui: Egui,
    signal_type: SignalType,
    settings: Settings,
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
        signal_type: SignalType::Digital,
        settings: Settings::new(),
    }
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.ui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Simulator Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut model.signal_type, SignalType::Digital, "Digital");
                ui.selectable_value(&mut model.signal_type, SignalType::Analog, "Analog");
            });

            if model.signal_type == SignalType::Digital {
                let settings = &mut model.settings.digital;
                crate::digital::draw_ui(ui, settings);
            } else {
                crate::analog::draw_ui(app, ui, &mut model.signal_type, &mut model.settings);
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

    if model.signal_type == SignalType::Digital {
        model.settings.digital.encoding.draw_encoding(&model.settings.digital.result, &app, &draw)
    } else {
        model.settings.analog.modulation.draw_modulation(&model, &app, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}
