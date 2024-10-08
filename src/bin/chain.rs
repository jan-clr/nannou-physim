extern crate nannou;

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

use nannou_physim::{integration_step, draw_particles, draw_grid, draw_vectors, draw_point, draw_interaction_radius};


const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

const GRAVITY: f32 = 10.0;
const BIAS_FACTOR: f32 = 0.0;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

struct Model {
    egui: Egui,
    start: Vec2,
    end: Vec2,
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    accelerations: Vec<Vec2>,
    previous_positions: Vec<Vec2>,
    nr_chain_links: usize,
    link_length: f32,
    spring_constant: f32,
}

fn model(_app: &App) -> Model {
    let window_id = _app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = _app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let nr_chain_links = 15;
    // init velocities and accelerations with zero vectors
    let velocities = vec![Vec2::new(0.0, 0.0); nr_chain_links];
    let accelerations = vec![Vec2::new(0.0, -GRAVITY); nr_chain_links];

    let start = Vec2::new(-400.0, 100.0);
    let end = Vec2::new(400.0, 100.0);
    let d = end - start;
    let start_distance = d.length() / (nr_chain_links as f32 + 1.0);
    let d_norm = d.normalize();
    // space the chain links evenly along the x-axis with the middle link at the origin across the gap
    let mut positions = vec![];
    for i in 0..nr_chain_links {
        let position = start + d_norm * start_distance * (i as f32 + 1.0);
        positions.push(position);
    }

    Model {
        egui,
        start,
        end,
        positions: positions.clone(),
        velocities,
        accelerations,
        previous_positions: positions,
        nr_chain_links,
        link_length: 50.0,
        spring_constant: 1.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let egui = &mut _model.egui;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    
    let delta = 0.05;
    let nr_steps = 5;
    let effective_delta = delta / nr_steps as f32;
    
    for _ in 0..nr_steps {
        //calculate_accelerations(&_model.positions, &mut _model.accelerations, _model.link_length);
        integration_step(&mut _model.positions, &mut _model.velocities, &_model.accelerations, effective_delta);
        distance_constraints(_model.start, _model.end, &mut _model.positions, &mut _model.velocities, _model.link_length, effective_delta);
    }

    egui::Window::new("Settings").show(&ctx, |ui| {
        let clicked = ui.button("Reset").clicked();
        if clicked {
            _model.positions = _model.previous_positions.clone();
            _model.velocities = vec![Vec2::new(0.0, 0.0); _model.nr_chain_links];
        }
    });
}

fn calculate_accelerations(positions: &Vec<Vec2>, accelerations: &mut Vec<Vec2>, link_length: f32) {
    let n = positions.len();
    for i in 0..n {
        let mut acceleration = Vec2::new(0.0, -GRAVITY);
        accelerations[i] = acceleration;
    }
}

fn distance_constraints(start: Vec2, end: Vec2, positions: &mut Vec<Vec2>, velocities: &mut Vec<Vec2>, link_length: f32, delta: f32) {
    let n = positions.len();
    
    // apply distance constrain between start and first chain link
    let d = positions[0] - start;
    let d_norm = d.normalize();
    let distance = d.length();
    let diff = distance - link_length;
    if diff > 0.0 {
        let relative_velocity = velocities[0];
        let bias = -BIAS_FACTOR * diff / delta;
        let lambda = - relative_velocity.dot(d_norm) + bias;
        let impulse = d_norm * lambda;
        velocities[0] += impulse;
    }
    
    
    for i in 0..n - 1 {
        let d = positions[i + 1] - positions[i];
        let d_norm = d.normalize();
        let distance = d.length();
        
        let diff = distance - link_length;
        if diff > 0.0 {
            let relative_velocity = velocities[i + 1] - velocities[i];
            let bias = -BIAS_FACTOR * diff / delta;
            let lambda = - relative_velocity.dot(d_norm) + bias;
            let impulse = d_norm * lambda / 2.0;
            velocities[i] -= impulse;
            velocities[i + 1] += impulse;
        }
    }
    
    // apply distance constrain between end and last chain link
    let d = end - positions[n - 1];
    let d_norm = d.normalize();
    let distance = d.length();
    let diff = distance - link_length;
    if diff > 0.0 {
        let relative_velocity = velocities[n - 1];
        let bias = -BIAS_FACTOR * diff / delta;
        let lambda = - relative_velocity.dot(d_norm) + bias;
        let impulse = d_norm * lambda;
        velocities[n - 1] += impulse;
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background().rgb(0.11, 0.12, 0.13);
    
    let win = _app.window_rect();
    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);
    
    draw_point(&draw, _model.start, rgb(1.0, 0.0, 0.0), 5.0);
    draw_point(&draw, _model.end, rgb(1.0, 0.0, 0.0), 5.0);

    draw_particles(&draw, &_model.positions, rgb(0.0, 1.0, 1.0), 5.0);
    draw_vectors(&draw, &_model.positions, &_model.velocities, rgb(0.0, 1.0, 0.0), 1.0);
    draw_interaction_radius(&draw, &_model.positions, _model.link_length);

    draw.to_frame(_app, &frame).unwrap();
    _model.egui.draw_to_frame(&frame).unwrap();
}