// Generative art test
// Learning from this video <https://www.youtube.com/watch?v=hwaBaoAPOU0>
// Template from this link
// <https://github.com/nannou-org/nannou/blob/master/examples/templates/template_app.rs>
use nannou::noise::*;
use nannou::prelude::*;

struct Point {
    coordinates: Point2,
    linewidth: f32,
    color: Rgba,
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Point>,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let width = 720;
    let height = 720;
    let _window = app
        .new_window()
        .size(width, height)
        .view(view)
        .build()
        .expect("Error creating window.");

    let mut points = Vec::new();

    let density = 30.0;
    let space = width as f32 / density;

    for x in (0..width).step_by(space as usize) {
        for y in (0..height).step_by(space as usize) {
            let coordinates = pt2(
                x as f32 - (width as f32 * 0.5),
                y as f32 - (height as f32 * 0.5),
            );
            let color = rgba(0.0, random_f32(), random_f32(), 0.2);
            let mut linewidth = random_f32() * 6.0;
            if linewidth < 1.0 {
                linewidth = 0.0;
            }
            points.push(Point {
                coordinates,
                linewidth,
                color,
            });
        }
    }

    Model { points, _window }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Perlin noise
    let noise = Perlin::new();
    let m = 0.002;

    model.points.iter_mut().for_each(|point| {
        let noise_value = noise.get([
            m * point.coordinates.x as f64,
            m * point.coordinates.y as f64,
        ]) as f32;
        let noise_value_map = deg_to_rad(map_range(noise_value, -1.0, 1.0, -270.0, 360.0));
        point.coordinates += pt2(noise_value_map.sin(), noise_value_map.cos());
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let origin = pt2(0.0, 0.0);
    for point in &model.points {
        if point.coordinates.distance(origin) < 300.0 {
            draw.ellipse()
                .xy(point.coordinates)
                .w_h(point.linewidth, point.linewidth)
                .color(point.color);
        }
    }

    draw.to_frame(app, &frame).expect("Error creating frame.")
}
