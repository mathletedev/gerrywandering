pub mod boid;
pub mod vector;

use nannou::prelude::*;

struct Model {
    window: WindowId,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(512, 512)
        .title("Gerrywandering")
        .view(view)
        .event(event)
        .build()
        .unwrap();

    Model { window }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("{:?}", event);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.rect().x_y(0.0, 0.0).w(10.0).color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
