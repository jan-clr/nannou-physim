use nannou::geom::{pt2, Rect, Vec2};

/// Use Stomer-Verlet integration to update the positions and velocities of the particles.
pub fn integration_step(positions: &mut Vec<Vec2>, velocities: &mut Vec<Vec2>, accelerations: &Vec<Vec2>, delta: f32) {
    let n = positions.len();
    for i in 0..n {
        let acceleration = accelerations[i];
        velocities[i] += acceleration * delta;
        positions[i] += velocities[i] * delta;
    }
}

pub fn draw_particles(draw: &nannou::draw::Draw, positions: &Vec<Vec2>, color: nannou::color::Rgb, radius: f32) {
    for position in positions {
        draw.ellipse()
            .x_y(position.x, position.y)
            .radius(radius)
            .color(color);
    }
}

pub fn draw_grid(draw: &nannou::draw::Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}

pub fn draw_vectors(draw: &nannou::draw::Draw, positions: &Vec<Vec2>, vectors: &Vec<Vec2>, color: nannou::color::Rgb, scale: f32) {
    for (i, position) in positions.iter().enumerate() {
        draw.arrow()
            .start(pt2(position.x, position.y))
            .end(pt2(position.x + vectors[i].x * scale, position.y + vectors[i].y * scale))
            .stroke_weight(2.0)
            .color(color);
    }
}

pub fn draw_point(draw: &nannou::draw::Draw, position: Vec2, color: nannou::color::Rgb, radius: f32) {
    draw.ellipse()
        .x_y(position.x, position.y)
        .radius(radius)
        .color(color);
}

pub fn draw_interaction_radius(draw: &nannou::draw::Draw, positions: &Vec<Vec2>, radius: f32) {
    for position in positions {
        draw.ellipse()
            .x_y(position.x, position.y)
            .radius(radius)
            .stroke_weight(1.0)
            .color(nannou::color::WHITE)
            .no_fill();
    }
}