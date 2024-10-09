use nannou::prelude::*;

use crate::{
    boid::Party,
    gerrymander::{favours, Node},
    model::Model,
    settings::BOID_SIZE,
};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.boids.iter().for_each(|boid| {
        let mut color = WHITE;

        if let Some(party) = &boid.party {
            color = match party {
                Party::RED => RED,
                Party::BLUE => BLUE,
            }
        }

        draw.tri()
            .points(
                boid.position + boid.velocity.normalize() * BOID_SIZE,
                boid.position + boid.velocity.rotate(TAU * 0.4).normalize() * BOID_SIZE,
                boid.position + boid.velocity.rotate(TAU * 0.6).normalize() * BOID_SIZE,
            )
            .color(color);
    });

    draw_districts(&model.districts_tree, &draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_districts(node: &Option<Box<Node>>, draw: &Draw) {
    let node = match node {
        Some(node) => node,
        None => return,
    };

    // draw.text(format!("Red: {}\nBlue: {}", node.party_count.0, node.party_count.1).as_str())
    //     .x(node.bounds.left + node.bounds.width / 2.0)
    //     .y(node.bounds.bottom + node.bounds.height / 2.0);

    draw_districts(&node.left, draw);
    draw_districts(&node.right, draw);

    if node.left.is_some() || node.right.is_some() {
        return;
    }

    let total = node.party_count.iter().sum::<u32>();

    draw.rect()
        .x(node.bounds.left + node.bounds.width / 2.0)
        .y(node.bounds.bottom + node.bounds.height / 2.0)
        .w(node.bounds.width)
        .h(node.bounds.height)
        .color(Rgba8 {
            color: match favours(node.party_count) {
                Some(Party::RED) => RED,
                Some(Party::BLUE) => BLUE,
                None => BLACK,
            },
            alpha: if total == 0 {
                0
            } else {
                (node.party_count.iter().max().unwrap() / total * 100) as u8
            },
        })
        .stroke_weight(2.0)
        .stroke(WHITE);
}
