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
use transforms::{MoveDirection, move_camera};
use types::{Shape, Camera, Pt2D};

const CANVAS_SIZE: (u32, u32) = (1024, 1024);

const τ: f64 = 2. * PI;

fn DEFAULT_CAMERA() -> Camera {
    // Effectively a global constant.
    Camera {
        // If 3d, the 4th items for position isn't used.
        position: array![0., 1., -6., -2.],
        θ: array![0., 0., 0., 0., 0., 0.],
        fov: τ / 5.,
        aspect: 1.,
        aspect_4: 1.,
        far: 50.,
        near: 0.1,
        strange: 1.0,
    }
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
            is_4d,
        };

        Ok(s)
    }
}

fn dist_from_edge(pt_0: &Array1<f64>, pt_1: &Array1<f64>, 
                  cam_posit: &Array1<f64>) -> f64 {
    // Find an edge's average distance from the camera, using only
    // the first three dimensions.
    assert![pt_0.len() == 4 && pt_1.len() == 4 && cam_posit.len() == 4];

    let avg_coord = (pt_1 + pt_0) / 2.;
    (
        (cam_posit[0] - -avg_coord[0]).powi(2) +
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
              projected: HashMap<(i32, i32), Array1<f64>>,
              shapes: &HashMap<i32, Shape>,
              cam_posit: &Array1<f64>,
    ) -> GameResult<graphics::Mesh> {
    // Draw a set of of connected lines, given projected nodes and edges.

    let mb = &mut graphics::MeshBuilder::new();

    // Center and scale to window.
    // Assume the points projected to 0 are at the center of the
    // screen, and the projected points range from -1 to +1 on each axis.
    const OFFSET_X: f32 = (CANVAS_SIZE.0 / 2) as f32;
    const OFFSET_Y: f32 = (CANVAS_SIZE.1 / 2) as f32;
    // view_size reflects the -1 -> +1 clipspace cube.
    const VIEW_SIZE: (f32, f32) = (2., 2.);

    let scaler = CANVAS_SIZE.0 as f32 / VIEW_SIZE.0;

    // todo we need to perform an additional 2d clipping step for non-1 aspect
    // todo ratios. Ie for landscape.
    let (mut x_clip, mut y_clip) = (1., 1.);
    if CANVAS_SIZE.0 > CANVAS_SIZE.1 {  // Landscape; clip y.
        y_clip = CANVAS_SIZE.1 as f64 / CANVAS_SIZE.0 as f64;
    } else if CANVAS_SIZE.1 > CANVAS_SIZE.0 {  // Portrait; clip x.
        x_clip = CANVAS_SIZE.0 as f64/ CANVAS_SIZE.1 as f64;
    }
    // Else AA == 1; no 2d clipping required.


    for (shape_id, shape) in shapes {
        for edge in &shape.edges {
            let start = &projected.get(&(*shape_id, edge.node0));
            let end = &projected.get(&(*shape_id, edge.node1));
            let start_unwrapped: &Array1<f64>;
            let end_unwrapped: &Array1<f64>;

            // Absent keys indicate we've clipped the point from the projection.
            match start {
                Some(pt) => start_unwrapped = pt,
                None => continue
            }
            match end {
                Some(pt) => end_unwrapped = pt,
                None => continue
            }

            let points = &[
                // ggez's Points use f32.
                Point2::new(
                    OFFSET_X + start_unwrapped[0] as f32 * scaler,
                    OFFSET_Y + start_unwrapped[1] as f32 * scaler,
                ),
                Point2::new(
                    OFFSET_X + end_unwrapped[0] as f32 * scaler,
                    OFFSET_Y + end_unwrapped[1] as f32 * scaler,
                ),
            ];

            // Set line thickness proportional to inverse distance.
            let dist = dist_from_edge(
                &shape.nodes[&edge.node0].a,
                &shape.nodes[&edge.node1].a,
                cam_posit
            );

            mb.line(
                points,
                find_thickness(0.3, 12.0, 0.1, 10., dist)  // line thickness.
            );
        }
    }

    mb.build(ctx)
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

        // todo for now, try time-based (FPS-based?) rotations here.
        for (id, shape) in &mut self.shapes {
            // += syntax appears not to work with ndarray, at least here.
            shape.orientation = &shape.orientation + &shape.rotation_speed;
        }

        let projected;

        let mesh = build_mesh(ctx,
                              projected,
                              &self.shapes,
                              &self.camera.position,
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


        // todo for now we've removed FPS controls; make the move simply modify
        // todo our position in abs coords. move_θ is unused, in this case.

        // Some of the entries appear for reversed, for reasons I don't
        // understand yet.
        match keycode {
            Keycode::W => {
                let move_vec = move_camera(MoveDirection::Forward, &self.camera.θ);
                self.camera.position += &(&move_vec * MOVE_SENSITIVITY);
                // println!("x {}, y {}, z {}", self.camera.position[0], self.camera.position[1], self.camera.position[2]);
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
            Keycode::F => {
                let move_vec = move_camera(MoveDirection::Kata, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },
            Keycode::R => {
                let move_vec = move_camera(MoveDirection::Ana, &self.camera.θ);
                self.camera.position += &(move_vec * MOVE_SENSITIVITY);
            },

            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            Keycode::Left => {
                self.camera.θ[2] += TURN_SENSITIVITY;  // todo why reverse?
            },
            Keycode::Right => {
                self.camera.θ[2] -= TURN_SENSITIVITY;
            },
            // Don't allow us to look greater than τ/4 up or down.
            Keycode::Down => {
                self.camera.θ[1] -= TURN_SENSITIVITY;
            },
            Keycode::Up => {
                self.camera.θ[1] += TURN_SENSITIVITY;
            },
            
            Keycode::Q => {
                self.camera.θ[0] -= TURN_SENSITIVITY;
            },
            Keycode::E => {
                self.camera.θ[0] += TURN_SENSITIVITY;
            },
            
            // 4d rotations

            Keycode::I => self.camera.θ[3] = add_ang_norm(self.camera.θ[3], TURN_SENSITIVITY),
            Keycode::K => self.camera.θ[3] = add_ang_norm(self.camera.θ[3], -TURN_SENSITIVITY),
            Keycode::O => self.camera.θ[4] = add_ang_norm(self.camera.θ[4], TURN_SENSITIVITY),
            Keycode::L => self.camera.θ[4] = add_ang_norm(self.camera.θ[4], -TURN_SENSITIVITY),
            Keycode::P => self.camera.θ[5] = add_ang_norm(self.camera.θ[5], TURN_SENSITIVITY),
            Keycode::Semicolon => self.camera.θ[5] = add_ang_norm(self.camera.θ[5], -TURN_SENSITIVITY),

            Keycode::V => self.camera.near -= 1. * ZOOM_SENSITIVITY,
            Keycode::B => self.camera.near += 1. * ZOOM_SENSITIVITY,

            Keycode::N => self.camera.far -= 1. * ZOOM_SENSITIVITY,
            Keycode::M => self.camera.far += 1. * ZOOM_SENSITIVITY,

            Keycode::Minus => self.camera.fov += 1. * ZOOM_SENSITIVITY,
            Keycode::Equals => self.camera.fov -= 1. * ZOOM_SENSITIVITY,

            // reset
            Keycode::Backspace => self.camera = DEFAULT_CAMERA(),
            _ => (),
        } 
    }
}

pub fn run(shapes: HashMap<i32, Shape>) {
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