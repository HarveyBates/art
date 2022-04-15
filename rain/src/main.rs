use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Raindrop {
    x: f32,
    y: f32,
    y_speed: f32,
    width: f32,
    length: f32,
    color: Rgba,
}

impl Raindrop {
    pub fn new() -> Self {
        let y_speed = rand::thread_rng().gen_range(0.2..1.2) as f32;
        let length = y_speed * 10.0;
        Raindrop {
            x: rand::thread_rng().gen_range(-360..360) as f32,
            y: rand::thread_rng().gen_range(-360..720) as f32,
            y_speed: y_speed * 2.0,
            width: rand::thread_rng().gen_range(1.0..3.0) as f32,
            length: length,
            color: rgba(0.5, 0.5, 0.8, y_speed - 0.2),
        }
    }

    pub fn fall(&mut self) {
        self.y = self.y - self.y_speed;
        if self.y < -360.0 {
            self.y = rand::thread_rng().gen_range(360..720) as f32;
        }
    }
}

struct Model {
    raindrops: Vec<Raindrop>,
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
    let mut raindrops = Vec::new();
    for _ in 0..500 {
        raindrops.push(Raindrop::new());
    }
    Model { raindrops, _window }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.raindrops.iter_mut().for_each(|drop| {
        drop.fall();
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.raindrops.iter().for_each(|drop| {
        draw.line()
            .start(pt2(drop.x, drop.y))
            .end(pt2(drop.x, drop.y + drop.length))
            .stroke_weight(drop.width)
            .color(drop.color);
    });

    draw.to_frame(app, &frame).unwrap();
}
