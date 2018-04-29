// Code for displaying the rendered product on the screen go in this
// module.

use std::collections::HashMap;

use ndarray::prelude::*;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;

use types::{Node, Edge};


struct MainState {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context, projected_nodes: Vec<Node>, edges: Vec<Edge>) -> GameResult<MainState> {  
        
        let s = MainState {
            // Nodes used here are projected 2d nodes.
            nodes: projected_nodes,
            edges: edges,
            zoomlevel: 1.0,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context, nodes: &Vec<Node>, edges: &Vec<Edge>) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    // create a map of nodes we can query from edges.
    let mut node_map = HashMap::new();
    for node in nodes.iter() {
        node_map.insert(&node.id, &node);
    }

    let mut points = vec![];
    for edge in edges.iter() {
        let start: &Node = node_map.get(&edge.node1).unwrap();
        let end: &Node = node_map.get(&edge.node2).unwrap();
        // ggez::graphics::Point2 accepts f32 only.
        points.push(Point2::new(start.a[0] as f32, start.a[1] as f32));
        points.push(Point2::new(end.a[0] as f32, end.a[1] as f32));
    }

    mb.line(
        &points,
        4.0,
    );

    // mb.ellipse(DrawMode::Fill, Point2::new(600.0, 200.0), 50.0, 120.0, 1.0);
    // mb.circle(DrawMode::Fill, Point2::new(600.0, 380.0), 40.0, 1.0);

    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.zoomlevel += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        
        let mesh = build_mesh(ctx, &self.nodes, &self.edges)?;
        graphics::set_color(ctx, (0, 255, 255).into())?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn render(projected_nodes: Vec<Node>, edges: Vec<Edge>) {
    // Attempting to render lines using ggez
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();
    
    // let points = Point2
    // graphics::line(ctx, points, 200)

    println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx, projected_nodes, edges).unwrap();
    
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}