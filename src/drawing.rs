// Code for displaying the rendered product on the screen go in this
// module.
// example: https://github.com/ggez/ggez/blob/master/examples/drawing.rs

use std::collections::HashMap;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{Point2};
use ggez::timer;

use types::{Node, Edge};


struct MainState {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    zoomlevel: f32,
}

impl MainState {
    fn new(_ctx: &mut Context, projected_nodes: Vec<Node>, edges: Vec<Edge>) -> GameResult<MainState> {  
        
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
    // Draw a set of of connected lines.
    let mb = &mut graphics::MeshBuilder::new();

    const SCALER: f32 = 100.;
    const OFFSET: f32 = 200.;

    // create a map of nodes we can query from edges.  Perhaps this extra
    // data structure is unecessary, or that hashmaps should be the primary
    // way of storing nodes.
    let mut node_map = HashMap::new();
    for node in nodes.iter() {
        node_map.insert(node.id, node.clone());
    }

    for edge in edges.iter() {
        let start: &Node = node_map.get(&edge.node1).unwrap();
        let end: &Node = node_map.get(&edge.node2).unwrap();

        let points = &[
            Point2::new(
                OFFSET + start.a[0] as f32 * SCALER, 
                OFFSET + start.a[1] as f32 * SCALER
            ),
            Point2::new(
                OFFSET + end.a[0] as f32 * SCALER, 
                OFFSET + end.a[1] as f32 * SCALER
            ),
        ];

        mb.line(
            points,
            3.0,  // line width.
        );
    }
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
    // Render lines using ggez.
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();
    
    println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx, projected_nodes, edges).unwrap();
    
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}