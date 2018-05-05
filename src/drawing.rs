// Code for displaying the rendered product on the screen go in this
// module.
// example: https://github.com/ggez/ggez/blob/master/examples/drawing.rs

use std::collections::HashMap;
use ndarray::prelude::*;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{Point2};
use ggez::timer;

use transforms;
use types::{Node, Shape, Camera};


struct MainState {
    shapes: Vec<Shape>,
    zoomlevel: f32,
    camera: Camera,
}

impl MainState {
    fn new(_ctx: &mut Context, shapes: Vec<Shape>) -> GameResult<MainState> {  
        
        let default_camera = Camera {
            c: Array::from_vec(vec![-0., 0., 0.]),
            position: array![0., 5.0, 5.],
            theta: array![0., 0., 0.],
            e: arr1(&[0., 0., -5.]),
        };

        let s = MainState {
            shapes: shapes,
            zoomlevel: 1.0,
            camera: default_camera,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context, projected_shapes: Vec<Shape>) -> GameResult<graphics::Mesh> {
    // Draw a set of of connected lines, given projected nodes and edges.
    let mb = &mut graphics::MeshBuilder::new();

    const SCALER: f32 = 100.;
    const OFFSET: f32 = 200.;

    for shape in projected_shapes {
        // create a map of projected_nodes we can query from edges.  Perhaps this extra
        // data structure is unecessary, or that hashmaps should be the primary
        // way of storing projected_nodes.
        let mut node_map = HashMap::new();
        for node in shape.nodes.iter() {
            node_map.insert(node.id, node.clone());
        }

        for edge in shape.edges.iter() {
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
                2.0,  // line width.
            );
        }
    }

    mb.build(ctx)
}

enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
}

fn move_camera(direction: MoveDirection, cam: &Camera) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1.],
        MoveDirection::Back => array![0., 0., -1.],
        MoveDirection::Left => array![-1., 0., 0.],
        MoveDirection::Right => array![1., 0., 0.],
        MoveDirection::Up => array![0., 1., 0.],
        MoveDirection::Down => array![0., -1., 0.],
    };

    transforms::rotate_3d(cam).dot(&unit_vec)
}

// fn _crop_to_screen(points: Vec<Point2>) -> Vec<Shape> {
//     // Examine each point; if it 
// }

fn add_vectors(vec1: &Array1<f64>, vec2: &Array1<f64>) -> Array1<f64> {
    // Work around borrow issues by creating a new vector with elements
    // of the arguments.
    array![vec1[0] + vec2[0], vec1[1] + vec2[1], vec1[2] + vec2[2]]
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
        
        // nodes are projected from 3d space into 2d space. Node associations
        // with edges are not affected by the transformation.
        let projected_shapes = transforms::project_shapes(&self.shapes, &self.camera);

        let mesh = build_mesh(ctx, projected_shapes)?;
        graphics::set_color(ctx, (0, 255, 255).into())?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
   
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        const MOVE_SENSITIVITY: f64 = 0.05;
        const TURN_SENSITIVITY: f64 = 0.05;

        // Some of the entries appear for reversed, for reasons I don't
        // understand yet.
        match keycode {
            // Keycode::A => self.camera.position += MOVE_SENSITIVITY * move_camera(MoveDirection::Left, &self.camera),

            Keycode::D => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Right, &self.camera)
            ),
            Keycode::S => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Back, &self.camera)
            ),
            Keycode::W => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Forward, &self.camera)
            ),
            Keycode::C => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Down, &self.camera)
            ),
            Keycode::LCtrl => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Down, &self.camera)
            ),
            Keycode::Space => self.camera.position = MOVE_SENSITIVITY * add_vectors(
                &self.camera.position, 
                &move_camera(MoveDirection::Up, &self.camera)
            ),
            // Keycode::D => self.camera.position += move_camera(MoveDirection::Right, &self.camera) * MOVE_SENSITIVITY,
            // Keycode::S => self.camera.position += move_camera(MoveDirection::Back, &self.camera) * MOVE_SENSITIVITY,
            // Keycode::W => self.camera.position += move_camera(MoveDirection::Forward, &self.camera) * MOVE_SENSITIVITY,
            // Keycode::C => self.camera.position += move_camera(MoveDirection::Down, &self.camera) * MOVE_SENSITIVITY,
            // Keycode::LCtrl => self.camera.position += move_camera(MoveDirection::Down, &self.camera) * MOVE_SENSITIVITY,
            // Keycode::Space => self.camera.position += move_camera(MoveDirection::Up, &self.camera) * MOVE_SENSITIVITY,
            
            Keycode::Left => self.camera.theta[1] -= 1. * TURN_SENSITIVITY,
            Keycode::Right => self.camera.theta[1] += 1. * TURN_SENSITIVITY,
            Keycode::Up => self.camera.theta[0] += 1. * TURN_SENSITIVITY,
            Keycode::Down => self.camera.theta[0] -= 1. * TURN_SENSITIVITY,
            Keycode::Q => self.camera.theta[2] -= 1. * TURN_SENSITIVITY,
            Keycode::E => self.camera.theta[2] += 1. * TURN_SENSITIVITY,

            Keycode::J => self.camera.e[0] -= 1. * 5. * TURN_SENSITIVITY,
            Keycode::L => self.camera.e[0] += 1. * 5. * TURN_SENSITIVITY,
            Keycode::I => self.camera.e[1] -= 1. * 5. * TURN_SENSITIVITY,
            Keycode::K => self.camera.e[1] += 1. * 5. * TURN_SENSITIVITY,
            Keycode::U => self.camera.e[2] -= 1. * 5. * TURN_SENSITIVITY,
            Keycode::O => self.camera.e[2] += 1. * 5. * TURN_SENSITIVITY,

            // reset
            Keycode::Backspace => {
                self.camera = Camera {
                    c: Array::from_vec(vec![0., 0., 0.]),
                    position: array![0., 0., 0.],
                    theta: array![0., 0., 0.],
                    e: arr1(&[0., 0., -5.]),
                };
            },

            _ => (),
        } 
    }
}

pub fn run(shapes: Vec<Shape>) {
    // Render lines using ggez.
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();
    
    println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx, shapes).unwrap();
    
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}