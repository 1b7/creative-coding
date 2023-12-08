use nannou::prelude::*;
use rand::random;

const NODES: usize = 1000;
const NODE_EDGES: usize = 2;
const DRAW_EDGES: bool = false;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model { node_set: [Node; NODES], edge_set: [usize; NODES * NODE_EDGES] }

fn model(app: &App) -> Model {
    let width = app.window_rect().x.end * 2.;
    let height = app.window_rect().y.end * 2.;

    let node_set: [Node; NODES] = (0..NODES).map(|_|
        Node::new(
            random::<f32>() * width - width/2., 
            random::<f32>() * height - height/2., 
            random::<f32>() * 10.0 - 5.0,
            random::<f32>() * 10.0 - 5.0
        )
    ).collect::<Vec<_>>().try_into().unwrap();

    let mut edge_set = [0; NODES * NODE_EDGES];

    // Naive distance calculation for drawing edges to nearest nodes;
    // Manhattan distance is used as a 'good enough' approximation.
    if DRAW_EDGES {
        for i in 0..NODES {
            let mut distances = [(f32::INFINITY, 0); NODES];
            for j in 0..NODES {
                if i == j { continue }
                let dist_x = node_set[i].x - node_set[j].x;
                let dist_y = node_set[i].y - node_set[j].y;
                let dist = dist_x + dist_y;
                distances[j] = (dist, j)
            }
            distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for e in 0..NODE_EDGES {
                edge_set[(NODE_EDGES * i) + e] = distances[e].1;
            }
        }
    }
    
    Model { node_set, edge_set }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    for mut node in &mut model.node_set {
        node.x += node.ax * update.since_last.as_secs_f32();
        node.y += node.ay * update.since_last.as_secs_f32();
    }

    for i in 0..NODES {
        let mut distances = [(f32::INFINITY, 0); NODES];
        for j in 0..NODES {
            if i == j { continue }
            let dist_x = (model.node_set[i].x - model.node_set[j].x).pow(2);
            let dist_y = (model.node_set[i].y - model.node_set[j].y).pow(2);
            let dist = ((dist_x + dist_y) as f32).sqrt();
            distances[j] = (dist, j)
        }

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for e in 0..NODE_EDGES {
            model.edge_set[(NODE_EDGES * i) + e] = distances[e].1;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    // network(&draw, model);
    fireflies(&draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn network(draw: &Draw, model: &Model) {
    if DRAW_EDGES {
        for (i, node) in model.node_set.iter().enumerate() {
            for e in i..(i + NODE_EDGES) {
                let other = model.node_set[model.edge_set[i + e]];
                let start = pt2(node.x, node.y);
                let end = pt2(other.x, other.y);
    
                draw.line()
                    .start(start)
                    .end(end)
                    .weight(1.)
                    .rgb(0.2, 0.2, 0.2);
            }
        }
    }

    for node in model.node_set {
        draw.ellipse()
            .color(WHITE)
            .w_h(5., 5.)
            .x_y(node.x, node.y);
    }
}

fn fireflies(draw: &Draw, model: &Model) {
    for node in model.node_set {
        draw.ellipse()
            .hsla(node.hsla.0, node.hsla.1, node.hsla.2 / 3., node.hsla.3 / 3.)
            .w_h(node.r * 1.5, node.r * 1.5)
            .x_y(node.x, node.y);
        
        draw.ellipse()
            .hsla(node.hsla.0, node.hsla.1, node.hsla.2, node.hsla.3)
            .w_h(node.r, node.r)
            .x_y(node.x, node.y);
    }
}

#[derive(Debug, Clone, Copy)]
struct Node { x: f32, y: f32, ax: f32, ay: f32, hsla: (f32, f32, f32, f32), r: f32 }

impl Node {
    pub fn new(x: f32, y: f32, ax: f32, ay: f32) -> Self { 
        Self {
            x,
            y,
            ax,
            ay,
            hsla: (
                rand::random::<f32>() * (30.0/360.0) + (10.0/360.0), 
                1., 
                rand::random::<f32>() * 0.4 + 0.1, 
                rand::random::<f32>() * 0.8 + 0.1
            ), 
            r: rand::random::<f32>() * 3.0 + 4.0 
        } 
    }
}