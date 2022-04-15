use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let width = 720;
    let height = 720;
    let _window = app
        .new_window()
        .view(view)
        .size(width, height)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKSLATEGRAY);

    let offset = -300.0;
    let size = 8.0;
    let gap = 20.0;

    for i in 0..50 {
        let color = match i % 2 {
            0 => GOLDENROD,
            1 => BLACK,
            _ => unreachable!(),
        };

        draw.rect()
            .x_y(offset + gap * i as f32, -(i * 20) as f32)
            .w_h(size * i as f32, size * i as f32)
            .color(color);

        draw.rect()
            .x_y(offset + gap * i as f32, -(i * -20) as f32)
            .w_h(size * i as f32, size * i as f32)
            .color(color);

        draw.rect()
            .x_y(-300.0 + gap * i as f32, i as f32)
            .w_h(size * i as f32, size * i as f32)
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}
