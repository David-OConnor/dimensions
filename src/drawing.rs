// Code for displaying the rendered product on the screen go in this
// module.
// example: https://github.com/ggez/ggez/blob/master/examples/drawing.rs

use std::collections::HashMap;
use std::f64::consts::PI;

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

const CANVAS_SIZE: (u32, u32) = (1024, 768);

fn DEFAULT_CAMERA() -> Camera {
    // Effectively a global constant.
    Camera {
        c: Array::from_vec(vec![-0., 0., 0.]),
        position: array![0., 5.0, 5.],
        theta: array![0., 0., 0.],
        // e: arr1(&[0., 0., 5.]),
        fov: (2. * PI) / 5., 
    }
}

struct MainState {
    shapes: Vec<Shape>,
    zoomlevel: f32,
    camera: Camera,
}

impl MainState {
    fn new(_ctx: &mut Context, shapes: Vec<Shape>) -> GameResult<MainState> {  
        let s = MainState {
            shapes: shapes,
            zoomlevel: 1.0,
            camera: DEFAULT_CAMERA(),
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context, projected_shapes: Vec<Shape>) -> GameResult<graphics::Mesh> {
    // Draw a set of of connected lines, given projected nodes and edges.
    let mb = &mut graphics::MeshBuilder::new();

    const SCALER: f32 = 100.;
    const OFFSET_X: f32 = (CANVAS_SIZE.0 / 2) as f32;
    const OFFSET_Y: f32 = (CANVAS_SIZE.1 / 2) as f32;

    // Scale to window.

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
                    OFFSET_X + start.a[0] as f32 * SCALER, 
                    OFFSET_Y + start.a[1] as f32 * SCALER
                ),
                Point2::new(
                    OFFSET_X + end.a[0] as f32 * SCALER, 
                    OFFSET_Y + end.a[1] as f32 * SCALER
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

fn move_camera(direction: MoveDirection, theta: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1.],
        MoveDirection::Back => array![0., 0., -1.],
        MoveDirection::Left => array![-1., 0., 0.],
        MoveDirection::Right => array![1., 0., 0.],
        MoveDirection::Up => array![0., 1., 0.],
        MoveDirection::Down => array![0., -1., 0.],
    };
    transforms::rotate_3d(theta).dot(&unit_vec)
}

// fn _clip_to_screen(points: Vec<Point2>) -> Vec<Shape> {
//     // Examine each point; if it 
// }

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

        // Compute R here; we don't need to compute it for each node.
        let R = transforms::rotate_3d(&self.camera.theta);
        let projected_shapes = transforms::project_shapes(
            &self.shapes, &self.camera, R, (640., 480.)
        );

        // todo clip here??

        let mesh = build_mesh(ctx, projected_shapes)?;
        graphics::set_color(ctx, (0, 255, 255).into())?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
   
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        const MOVE_SENSITIVITY: f64 = 0.05;
        const TURN_SENSITIVITY: f64 = 0.05;
        const ZOOM_SENSITIVITY: f64 = 0.02;

        // Some of the entries appear for reversed, for reasons I don't
        // understand yet.
        match keycode {
            Keycode::W => {
                let move_vec = move_camera(MoveDirection::Forward, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::S => {
                let move_vec = move_camera(MoveDirection::Back, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::A => {
                let move_vec = move_camera(MoveDirection::Left, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::D => {
                let move_vec = move_camera(MoveDirection::Right, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::C => {
                let move_vec = move_camera(MoveDirection::Down, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::LCtrl => {
                let move_vec = move_camera(MoveDirection::Down, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::Space => {
                let move_vec = move_camera(MoveDirection::Up, &self.camera.theta);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },

            Keycode::Left => self.camera.theta[1] -= 1. * TURN_SENSITIVITY,
            Keycode::Right => self.camera.theta[1] += 1. * TURN_SENSITIVITY,
            Keycode::Up => self.camera.theta[0] += 1. * TURN_SENSITIVITY,
            Keycode::Down => self.camera.theta[0] -= 1. * TURN_SENSITIVITY,
            Keycode::Q => self.camera.theta[2] -= 1. * TURN_SENSITIVITY,
            Keycode::E => self.camera.theta[2] += 1. * TURN_SENSITIVITY,

            Keycode::Equals => self.camera.fov -= 1. * ZOOM_SENSITIVITY,
            Keycode::Minus => self.camera.fov += 1. * ZOOM_SENSITIVITY,

            // reset
            Keycode::Backspace => self.camera = DEFAULT_CAMERA(),
            _ => (),
        } 
    }
}

pub fn run(shapes: Vec<Shape>) {
    // Render lines using ggez.
    let c = conf::Conf {
        window_mode: conf::WindowMode {
            width: CANVAS_SIZE.0,
            height: CANVAS_SIZE.1,
            borderless: false,
            fullscreen_type: conf::FullscreenType::Off,
            vsync: true,
            min_width: 640,
            min_height: 480,
            max_width: 1024,
            max_height: 1024,
    },
        window_setup: conf::WindowSetup {
            title: String::from("4D visualization"),
            icon: String::from(""),
            resizable: false,
            allow_highdpi: true,
            samples: conf::NumSamples::Eight  // ie anti-aliasing.
    },
        backend: conf::Backend::OpenGL {
            major: 3,
            minor: 2,
        }
    };

    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();
    
    println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx, shapes).unwrap();
    
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}