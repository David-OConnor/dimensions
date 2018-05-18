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
use types::{Shape, Edge, Camera, Pt2D};

const CANVAS_SIZE: (u32, u32) = (1024, 1024);

const τ: f64 = 2. * PI;

fn DEFAULT_CAMERA() -> Camera {
    // Effectively a global constant.
    Camera {
        // If 3d, the 4th items for position isn't used.
        position: array![0., 3., -6., -2.],
        θ_3d: array![0., 0., 0.],
        θ_4d: array![0., 0., 0., 0., 0., 0.],
        fov_hor: τ / 5.,
        fov_vert: τ / 5.,
        f: 30.,
        n: 0.9,
    }
}

enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
    Fourup,
    Fourdown,
}

struct MainState {
    shapes: HashMap<i32, Shape>,
    zoomlevel: f32,
    camera: Camera,
    is_4d: bool,
}

impl MainState {
    fn new(_ctx: &mut Context, shapes: HashMap<i32, Shape>, is_4d: bool) -> GameResult<MainState> {  
        let s = MainState {
            shapes,
            zoomlevel: 1.0,
            camera: DEFAULT_CAMERA(),
            is_4d: is_4d,
        };

        Ok(s)
    }
}

fn dist_from_edge(pt_0: &Array1<f64>, pt_1: &Array1<f64>, 
                  cam_posit: &Array1<f64>) -> f64 {
    // Find an edge's average distance from the camera, using only
    // the first three dimensions.
    assert![pt_0.len() == 4 && pt_1.len() == 4 && cam_posit.len() == 4];

    let avg_coord = (pt_1 - pt_0) / 2.;
    
    (
        (cam_posit[0] - avg_coord[0]).powi(2) + 
        (cam_posit[1] - avg_coord[1]).powi(2) + 
        (cam_posit[2] - avg_coord[2]).powi(2)
    ).sqrt()
}

fn find_thickness(min_width: f32, max_width: f32,
                  min_dist: f32, max_dist: f32, dist: f64) -> f32 {
    // Edges closer to the user (Z axis) are thicker.
    let mut portion_through = (dist as f32 - min_dist) / (max_dist - min_dist);

    if portion_through > 1. {
        portion_through = 1.
    } else if portion_through < 0. {
        portion_through = 0.
    }
    max_width - portion_through * (max_width - min_width)
}

// fn set_color(start_color: f32, end_color: f32,
//              min_dist: f32, max_dist: f32, dist: f64) -> f32 {
//     // Edges vary in color, depending on fourth-dimension (U axis) distance.
//     let portion_through = (dist as f32 - min_dist) / (max_dist - min_dist);
//     min_width + portion_through * (max_width - min_width)
// }

fn build_mesh(ctx: &mut Context, 
              projected_shapes: HashMap<i32, Shape>,
              raw_shapes: &HashMap<i32, Shape>,
              cam_posit: &Array1<f64>, 
              width: f64
    ) -> GameResult<graphics::Mesh> {
    // Draw a set of of connected lines, given projected nodes and edges.

    let mb = &mut graphics::MeshBuilder::new();

    const OFFSET_X: f32 = (CANVAS_SIZE.0 / 2) as f32;
    const OFFSET_Y: f32 = (CANVAS_SIZE.1 / 2) as f32;

    // Scale to window.
    // Assume the points projected to 0 are at the center of the
    // screen, and that we've projected onto a square window.
    let x_max = width / 2.;
    let y_max = x_max;
    let x_min = -x_max;
    let y_min = x_min;

    let scaler = (CANVAS_SIZE.0 as f64 / width) as f32;

    for (shape_id, shape) in projected_shapes {
        for edge in &shape.edges {
            let start = &shape.nodes[&edge.node1];
            let end = &shape.nodes[&edge.node2];

            let start_pt = Pt2D {x: start.a[0], y: start.a[1]};
            let end_pt = Pt2D {x: end.a[0], y: end.a[1]};

            let clipped_pt = clipping::clip(
                &start_pt, &end_pt, x_min, x_max, y_min, y_max
            );
            
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

            let dist = dist_from_edge(
                &raw_shapes[&shape_id].nodes[&edge.node1].a,
                &raw_shapes[&shape_id].nodes[&edge.node2].a,
                cam_posit
            );

            println!("THICK: {}", find_thickness(0.1, 10.0, 1.0, 10., dist));

            mb.line(
                points,
                find_thickness(0.3, 10.0, 0.5, 5., dist)  // line width.
            );
        }
    }

    mb.build(ctx)
}

fn move_camera_3d(direction: MoveDirection, θ: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    assert!(θ.len() == 3);

    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1.],
        MoveDirection::Back => array![0., 0., -1.],

        // Reversed for mirror effect
        MoveDirection::Left => -array![-1., 0., 0.],
        MoveDirection::Right => -array![1., 0., 0.],

        MoveDirection::Up => array![0., 1., 0.],
        MoveDirection::Down => array![0., -1., 0.],
        // For 4d move inputs, don't do anything.
        MoveDirection::Fourup => array![0., 0., 0.],
        MoveDirection::Fourdown => array![0., 0., 0.],
    };

    // Position always uses 3d vectors, with an unused fourth element.
    // stack![Axis(0), transforms::rotate_3d_2(θ).dot(&unit_vec), array![0.]]
    stack![Axis(0), unit_vec, array![0.]]
}

fn move_camera_4d(direction: MoveDirection, θ: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    assert!(θ.len() == 6);

    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1., 0.],
        MoveDirection::Back => array![0., 0., -1., 0.],
        // todo not sure why I negatee left/Righ tmovement to flip world... mirror effect?
        MoveDirection::Left => -array![-1., 0., 0., 0.],
        MoveDirection::Right => -array![1., 0., 0., 0.],
        
        MoveDirection::Up => array![0., 1., 0., 0.],
        MoveDirection::Down => array![0., -1., 0., 0.],
        MoveDirection::Fourup => array![0., 0., 0., 1.],
        MoveDirection::Fourdown => array![0., 0., 0., -1.],
    };

    unit_vec
    // transforms::rotate_4d(θ).dot(&unit_vec)
}

fn add_ang_norm(angle: f64, amount: f64) -> f64 {
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

            let R_4d = transforms::rotate_4d(&-&self.camera.θ_4d);
            let R_3d = transforms::rotate_3d(&-&self.camera.θ_3d);
            projected_shapes = transforms::project_shapes_4d(
                &self.shapes, &self.camera, &R_4d, &R_3d
            );
        } else {
            // let R = transforms::rotate_3d(&-&self.camera.θ_3d);
            let R = transforms::rotate_3d(&self.camera.θ_3d);
            projected_shapes = transforms::project_shapes_3d(
                &self.shapes, &self.camera, &R
            );
        }

        let mesh = build_mesh(ctx, 
                              projected_shapes, 
                              &self.shapes,
                              &self.camera.position,
                              self.camera.width()
        )?;

        graphics::set_color(ctx, (0, 255, 255).into())?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
   
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        const MOVE_SENSITIVITY: f64 = 0.05;
        const TURN_SENSITIVITY: f64 = 0.05;
        const ZOOM_SENSITIVITY: f64 = 0.02;

        let move_func = match self.is_4d {
            true => move_camera_4d,
            false => move_camera_3d,
        };

        let move_θ = &match self.is_4d {
            true => self.camera.θ_4d.clone(),
            false => self.camera.θ_3d.clone(),
        };

        // Some of the entries appear for reversed, for reasons I don't
        // understand yet.
        match keycode {
            Keycode::W => {
                let move_vec = move_func(MoveDirection::Forward, move_θ);
                self.camera.position += &(&move_vec * MOVE_SENSITIVITY);
                // println!("x {}, y {}, z {}", self.camera.position[0], self.camera.position[1], self.camera.position[2]);
                println!("x {}, y {}, z {}", &move_vec[0], &move_vec[1], &move_vec[2]);
                println!("Posit: {}", &self.camera.position);
                println!("θ {}", &self.camera.θ_3d);
                println!("width: {}", &self.camera.width());
            },
            Keycode::S => {
                let move_vec = move_func(MoveDirection::Back, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },

            Keycode::A => {
                let move_vec = move_func(MoveDirection::Left, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::D => {
                let move_vec = move_func(MoveDirection::Right, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::C => {
                let move_vec = move_func(MoveDirection::Down, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::LCtrl => {
                let move_vec = move_func(MoveDirection::Down, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::Space => {
                let move_vec = move_func(MoveDirection::Up, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::A => {
                let move_vec = move_func(MoveDirection::Left, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::D => {
                let move_vec = move_func(MoveDirection::Right, move_θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::F => {
                let move_vec = move_func(MoveDirection::Fourdown, &self.camera.θ_4d);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::R => {
                let move_vec = move_func(MoveDirection::Fourup, &self.camera.θ_4d);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },

            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            Keycode::Left => self.camera.θ_3d[1] = add_ang_norm(self.camera.θ_3d[1], -TURN_SENSITIVITY),
            Keycode::Right => self.camera.θ_3d[1] = add_ang_norm(self.camera.θ_3d[1], TURN_SENSITIVITY),
            // Don't allow us to look greater than τ/4 up or down.
            Keycode::Down => {
                self.camera.θ_3d[0] -= TURN_SENSITIVITY;
                if self.camera.θ_3d[0] <= -τ / 4. {
                    self.camera.θ_3d[0] = -τ / 4.
                }
            },
            Keycode::Up => {
                self.camera.θ_3d[0] += TURN_SENSITIVITY;
                if self.camera.θ_3d[0] >= τ / 4. {
                    self.camera.θ_3d[0] = τ / 4.
                }
            },
            
            Keycode::Q => self.camera.θ_3d[2] = add_ang_norm(self.camera.θ_3d[2], -TURN_SENSITIVITY),
            Keycode::E => self.camera.θ_3d[2] = add_ang_norm(self.camera.θ_3d[2], TURN_SENSITIVITY),
            
            // 4d rotations
            Keycode::T => self.camera.θ_4d[0] = add_ang_norm(self.camera.θ_4d[0], TURN_SENSITIVITY),
            Keycode::G => self.camera.θ_4d[0] = add_ang_norm(self.camera.θ_4d[0], -TURN_SENSITIVITY),
            Keycode::Y => self.camera.θ_4d[1] = add_ang_norm(self.camera.θ_4d[1], TURN_SENSITIVITY),
            Keycode::H => self.camera.θ_4d[1] = add_ang_norm(self.camera.θ_4d[1], -TURN_SENSITIVITY),
            Keycode::U => self.camera.θ_4d[2] = add_ang_norm(self.camera.θ_4d[2], TURN_SENSITIVITY),
            Keycode::J => self.camera.θ_4d[2] = add_ang_norm(self.camera.θ_4d[2], -TURN_SENSITIVITY),
            Keycode::I => self.camera.θ_4d[3] = add_ang_norm(self.camera.θ_4d[3], TURN_SENSITIVITY),
            Keycode::K => self.camera.θ_4d[3] = add_ang_norm(self.camera.θ_4d[3], -TURN_SENSITIVITY),
            Keycode::O => self.camera.θ_4d[4] = add_ang_norm(self.camera.θ_4d[4], TURN_SENSITIVITY),
            Keycode::L => self.camera.θ_4d[4] = add_ang_norm(self.camera.θ_4d[4], -TURN_SENSITIVITY),
            Keycode::P => self.camera.θ_4d[5] = add_ang_norm(self.camera.θ_4d[5], TURN_SENSITIVITY),
            Keycode::Semicolon => self.camera.θ_4d[5] = add_ang_norm(self.camera.θ_4d[5], -TURN_SENSITIVITY),

            Keycode::F => self.camera.n -= 1. * ZOOM_SENSITIVITY,
            Keycode::R => self.camera.n += 1. * ZOOM_SENSITIVITY,

            Keycode::G => self.camera.f -= 1. * ZOOM_SENSITIVITY,
            Keycode::T => self.camera.f += 1. * ZOOM_SENSITIVITY,

            Keycode::Minus => self.camera.fov_hor += 1. * ZOOM_SENSITIVITY,
            Keycode::Equals => self.camera.fov_hor -= 1. * ZOOM_SENSITIVITY,

            // reset
            Keycode::Backspace => self.camera = DEFAULT_CAMERA(),
            _ => (),
        } 
    }
}

pub fn run(shapes: HashMap<i32, Shape>, is_4d: bool) {
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