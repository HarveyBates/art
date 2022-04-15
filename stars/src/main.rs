use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Star {
    x: f32,
    y: f32,
    z: f32,
    width: f32,
}

impl Star {
    pub fn new() -> Self {
        Star {
            x: rand::thread_rng().gen_range(-360..360) as f32,
            y: rand::thread_rng().gen_range(-360..360) as f32,
            width: rand::thread_rng().gen_range(0.0..3.0) as f32,
            z: rand::thread_rng().gen_range(0..720) as f32,
        }
    }

    pub fn shift(&mut self) {
        self.z = self.z - 5.0;
        if self.z < 1.0 {
            self.z = rand::thread_rng().gen_range(0..720) as f32;
        }
    }
}

struct Model {
    stars: Vec<Star>,
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
    let mut stars = Vec::new();
    for _ in 0..300 {
        stars.push(Star::new());
    }
    Model { stars, _window }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.stars.iter_mut().for_each(|star| {
        star.shift();
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.stars.iter().for_each(|star| {
        draw.ellipse()
            .x_y((star.x / star.z) * 180.0, (star.y / star.z) * 180.0)
            .w_h(star.width, star.width)
            .color(WHITE);
    });

    draw.to_frame(app, &frame).unwrap();
}
