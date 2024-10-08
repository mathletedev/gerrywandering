use nannou::prelude::*;

use crate::model::Model;

pub fn event(_app: &App, _model: &mut Model, _event: Event) {}

pub fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
