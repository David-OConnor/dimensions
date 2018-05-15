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

use clipping;
use transforms;
use types::{Shape, Camera, Pt2D};

const CANVAS_SIZE: (u32, u32) = (1024, 1024);

const τ: f64 = 2. * PI;

fn DEFAULT_CAMERA() -> Camera {
    // Effectively a global constant.
    Camera {
        position: array![0., 2.0, -5.],
        θ: array![0., τ / 16., 0.],
        fov: τ / 5.,
        f: 30.,
        n: 0.9,
    }
}

struct MainState {
    shapes: Vec<Shape>,
    zoomlevel: f32,
    camera: Camera,
    is_4d: bool,
}

impl MainState {
    fn new(_ctx: &mut Context, shapes: Vec<Shape>, is_4d: bool) -> GameResult<MainState> {  
        let s = MainState {
            shapes,
            zoomlevel: 1.0,
            camera: DEFAULT_CAMERA(),
            is_4d: is_4d,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context, projected_shapes: Vec<Shape>, width: f64) -> GameResult<graphics::Mesh> {
    // Draw a set of of connected lines, given projected nodes and edges.
    let mb = &mut graphics::MeshBuilder::new();

    const OFFSET_X: f32 = (CANVAS_SIZE.0 / 2) as f32;
    const OFFSET_Y: f32 = (CANVAS_SIZE.1 / 2) as f32;

    // Scale to window.
    // Assume the points projected to 0 are at the center of the
    // screen, and that we've projected onto a square window.
    let x_max = width / 2.2;
    let y_max = x_max;
    let x_min = -x_max;
    let y_min = x_min;

    let scaler = (CANVAS_SIZE.0 as f64 / width) as f32;

    for shape in projected_shapes {
        // create a map of projected_nodes we can query from edges.  Perhaps this extra
        // data structure is unecessary, or that hashmaps should be the primary
        // way of storing projected_nodes.
        let mut node_map = HashMap::new();
        for node in &shape.nodes{
            node_map.insert(node.id, node.clone());
        }

        for edge in &shape.edges {
            let start = node_map[&edge.node1];
            let end = node_map[&edge.node2];

            let start_pt = Pt2D {x: start.a[0], y: start.a[1]};
            let end_pt = Pt2D {x: end.a[0], y: end.a[1]};

            let clipped_pt = clipping::clip(
                &start_pt, &end_pt, x_min, x_max, y_min, y_max);
            
            let start_clipped: Pt2D;
            let end_clipped: Pt2D;

            match clipped_pt {
                Some(pt) => {
                    start_clipped = pt.0;
                    end_clipped = pt.1;
                }
                None => continue,
            };
          
            let points = &[
                Point2::new(
                    OFFSET_X + start_clipped.x as f32 * scaler, 
                    OFFSET_Y + start_clipped.y as f32 * scaler,
                ),
                Point2::new(
                    OFFSET_X + end_clipped.x as f32 * scaler, 
                    OFFSET_Y + end_clipped.y as f32 * scaler,
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

fn move_camera(direction: MoveDirection, θ: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1.],
        MoveDirection::Back => array![0., 0., -1.],
        MoveDirection::Left => array![-1., 0., 0.],
        MoveDirection::Right => array![1., 0., 0.],
        MoveDirection::Up => array![0., 1., 0.],
        MoveDirection::Down => array![0., -1., 0.],
    };

    transforms::rotate_3d(θ).dot(&unit_vec)
}

fn normalized_add_angle(angle: f64, amount: f64) -> f64 {
    let mut new_angle = (angle + amount) % τ;
    if new_angle < 0. {
        new_angle += τ;
    }
    new_angle
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

        // Compute R here; we don't need to compute it for each node.

        let projected_shapes;

        if self.is_4d {
            // Invert θ, since we're treating the camera as static, rotating
            // the world around it.
            // Same reason we invert the position transforms in transforms.rs.

            let R = transforms::rotate_4d(&-&self.camera.θ);
            projected_shapes = transforms::project_shapes_4d(
                &self.shapes, &self.camera, &R
            );
        } else {
            let R = transforms::rotate_3d(&-&self.camera.θ);
            projected_shapes = transforms::project_shapes_3d(
                &self.shapes, &self.camera, &R
            );
        }

        let mesh = build_mesh(ctx, projected_shapes, self.camera.width())?;
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
                let move_vec = move_camera(MoveDirection::Forward, &self.camera.θ);
                self.camera.position += &(&move_vec * MOVE_SENSITIVITY);
                // println!("x {}, y {}, z {}", self.camera.position[0], self.camera.position[1], self.camera.position[2]);
                println!("x {}, y {}, z {}", &move_vec[0], &move_vec[1], &move_vec[2]);
                println!("θ {}", &self.camera.θ);
                println!("width: {}", &self.camera.width());
            },
            Keycode::S => {
                let move_vec = move_camera(MoveDirection::Back, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::A => {
                let move_vec = move_camera(MoveDirection::Left, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::D => {
                let move_vec = move_camera(MoveDirection::Right, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::C => {
                let move_vec = move_camera(MoveDirection::Down, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::LCtrl => {
                let move_vec = move_camera(MoveDirection::Down, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::Space => {
                let move_vec = move_camera(MoveDirection::Up, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },

            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            Keycode::Left => self.camera.θ[1] = normalized_add_angle(self.camera.θ[1], -TURN_SENSITIVITY),
            Keycode::Right => self.camera.θ[1] = normalized_add_angle(self.camera.θ[1], TURN_SENSITIVITY),
            // Don't allow us to look greater than τ/4 up or down.
            Keycode::Down => {
                self.camera.θ[0] -= TURN_SENSITIVITY;
                if self.camera.θ[0] <= -τ / 4. {
                    self.camera.θ[0] = -τ / 4.
                }
            },
            Keycode::Up => {
                self.camera.θ[0] += TURN_SENSITIVITY;
                if self.camera.θ[0] >= τ / 4. {
                    self.camera.θ[0] = τ / 4.
                }
            },
            
            Keycode::Q => self.camera.θ[2] = normalized_add_angle(self.camera.θ[2], -TURN_SENSITIVITY),
            Keycode::E => self.camera.θ[2] = normalized_add_angle(self.camera.θ[2], TURN_SENSITIVITY),
            
            Keycode::F => self.camera.n -= 1. * ZOOM_SENSITIVITY,
            Keycode::R => self.camera.n += 1. * ZOOM_SENSITIVITY,

            Keycode::G => self.camera.f -= 1. * ZOOM_SENSITIVITY,
            Keycode::T => self.camera.f += 1. * ZOOM_SENSITIVITY,

            Keycode::Minus => self.camera.fov += 1. * ZOOM_SENSITIVITY,
            Keycode::Equals => self.camera.fov -= 1. * ZOOM_SENSITIVITY,

            // reset
            Keycode::Backspace => self.camera = DEFAULT_CAMERA(),
            _ => (),
        } 
    }
}

pub fn run(shapes: Vec<Shape>, is_4d: bool) {
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

    let state = &mut MainState::new(ctx, shapes, is_4d).unwrap();
    
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}