extern crate nannou;

use nannou::prelude::*;
use nannou::geom::rect::Rect as Rect;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particle_x: f32,
    particle_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    acceleration_x: f32,
    acceleration_y: f32,
    particle_mass: f32,
    source_x: f32,
    source_y: f32,
    source_mass: f32,
    source_density: f32,
    source_radius: f32,
    gravitational_constant: f32,
    previous_positions: Vec<(f32, f32)>,
}

fn model(_app: &App) -> Model {
    let density = 1.0;
    let radius = 400.0;
    Model {
        particle_x: 400.0,
        particle_y: 0.0,
        velocity_x: 0.0, 
        velocity_y: 10.0,
        acceleration_x: 0.0,
        acceleration_y: 0.0,
        particle_mass: 1.0,
        source_x: 0.0,
        source_y: 0.0,
        // mass = density * radius^2 * pi
        source_mass: density * radius * radius * PI,
        source_density: density,
        source_radius: radius,
        gravitational_constant: 5.0,
        previous_positions: Vec::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let steps_per_update = 10;
    let delta = 0.05;
    // save up to 100 previous positions
    _model.previous_positions.push((_model.particle_x, _model.particle_y));
    if _model.previous_positions.len() > 100 {
        _model.previous_positions.remove(0);
    }
    
    for _ in 0..steps_per_update {
        integration_step(_model, delta);
    }
}

fn integration_step(_model: &mut Model, delta: f32) {
    let (force_x, force_y) = gravitational_force_on_particle(
        _model.particle_x,
        _model.particle_y,
        _model.source_x,
        _model.source_y,
        _model.particle_mass,
        _model.source_mass,
        _model.gravitational_constant,
        _model.source_radius,
    );

    // use the stormer-verlet integrator
    _model.acceleration_x = force_x / _model.particle_mass;
    _model.acceleration_y = force_y / _model.particle_mass;
    
    _model.velocity_x += _model.acceleration_x * delta;
    _model.velocity_y += _model.acceleration_y * delta;
    
    _model.particle_x += _model.velocity_x * delta;
    _model.particle_y += _model.velocity_y * delta;
}

/**
* Calculate the gravitational force on a particle at (x1, y1) with mass m1
* due to a source at (x2, y2) with mass m2.
* The gravitational constant is g.
* The radius of the source is cbr.
*
* Returns a tuple with the x and y components of the force.
*/
fn gravitational_force_on_particle(x1: f32, y1: f32, x2: f32, y2: f32, m1: f32, m2: f32, g: f32, cbr: f32) -> (f32, f32) {
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx*dx + dy*dy).sqrt();
    
    if distance > cbr {
        let force = g * m1 * m2 / (distance * distance);
        let angle = dy.atan2(dx);
        let force_x = force * angle.cos();
        let force_y = force * angle.sin();
        (force_x, force_y)
    } else {
        let force = g * m1 * m2 * distance / (cbr * cbr * cbr);
        let angle = dy.atan2(dx);
        let force_x = force * angle.cos();
        let force_y = force * angle.sin();
        (force_x, force_y)
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background().color(BLACK);
    
    draw_central_body(&draw, _model);
    draw_particle(&draw, _model);
    draw_accelaration(&draw, _model);
    draw_trace(&draw, _model);
    println!("Distance: {}", (_model.particle_x * _model.particle_x + _model.particle_y * _model.particle_y).sqrt());

    draw.to_frame(_app, &frame).unwrap();
}

fn draw_central_body(draw: &Draw, _model: &Model){
    draw.ellipse()
        .w_h(_model.source_radius * 2.0, _model.source_radius * 2.0)
        .x_y(_model.source_x, _model.source_y)
        .color(ROYALBLUE);
    // draw multiple layers for an earth like appearance
    draw.ellipse()
        .w_h(_model.source_radius * 2.0 * 0.95, _model.source_radius * 2.0 * 0.95)
        .x_y(_model.source_x, _model.source_y)
        .color(SIENNA);
    draw.ellipse()
        .w_h(_model.source_radius * 2.0 * 0.9, _model.source_radius * 2.0 * 0.9)
        .x_y(_model.source_x, _model.source_y)
        .color(RED);
    draw.ellipse()
        .w_h(_model.source_radius * 2.0 * 0.6, _model.source_radius * 2.0 * 0.6)
        .x_y(_model.source_x, _model.source_y)
        .color(ORANGE);
    draw.ellipse()
        .w_h(_model.source_radius * 2.0 * 0.3, _model.source_radius * 2.0 * 0.3)
        .x_y(_model.source_x, _model.source_y)
        .color(YELLOW);
}

fn draw_particle(draw: &Draw, _model: &Model){
    let rect = Rect::<f32>::from_w_h(20.0, 20.0);
    draw.ellipse()
        .wh(rect.wh())
        .x_y(_model.particle_x, _model.particle_y)
        .color(BLUE);
}

fn draw_accelaration(draw: &Draw, _model: &Model){
    let arrow_scaling = 100.0;
    draw.arrow()
        .start(pt2(_model.particle_x, _model.particle_y))
        .end(pt2(_model.particle_x + _model.acceleration_x * arrow_scaling, _model.particle_y + _model.acceleration_y * arrow_scaling))
        .stroke_weight(2.0)
        .color(GREEN);
}

fn draw_trace(draw: &Draw, _model: &Model){
    // draw a fading trace of the particle as a line
    let mut previous_positions = _model.previous_positions.iter();
    let mut previous_position = previous_positions.next();
    while let Some(next_position) = previous_positions.next() {
        let color = rgba(0.0, 0.0, 1.0, 1.0 - (previous_positions.len() as f32 / 100.0));
        draw.line()
            .start(pt2(previous_position.unwrap().0, previous_position.unwrap().1))
            .end(pt2(next_position.0, next_position.1))
            .color(color);
        previous_position = Some(next_position);
    }
}
 