// Handles keyboard and mouse input.

use ndarray::prelude::*;

use types::{Camera, CameraType, Shape};

#[derive(Copy, Clone, Debug)]
pub enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
    Ana,
    Kata,
}

pub fn move_camera(direction: MoveDirection, θ: &Array1<f32>, amount: f32) -> Array1<f32> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1., 0.],
        MoveDirection::Back => array![0., 0., -1., 0.],
        MoveDirection::Left => array![-1., 0., 0., 0.],
        MoveDirection::Right => array![1., 0., 0., 0.],
        MoveDirection::Up => array![0., 1., 0., 0.],
        MoveDirection::Down => array![0., -1., 0., 0.],
        MoveDirection::Ana => array![0., 0., 0., 1.],
        MoveDirection::Kata => array![0., 0., 0., -1.],
    };

    unit_vec * amount
    // transforms::rotate_4d(θ).dot(&unit_vec)
}

pub fn handle_pressed(pressed: &[u32], delta_time: f32,
                      move_sensitivity: f32, rotate_sensitivity: f32,
                      cam: &mut Camera, cam_type: &CameraType, shape: &mut Shape) -> () {
    // shape is only used when displaying single shapes.
    // delta_time is in seconds.
    let move_amount = move_sensitivity * delta_time;
    let rotate_amount = rotate_sensitivity * delta_time;

    for code in pressed {
        match *code {
            17 => {  // W
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Forward, &cam.θ, move_amount)
                }
            },
            31 => {  // S
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Back, &cam.θ, move_amount)
                }
            },
            30 => {  // A
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Left, &cam.θ, move_amount)
                }
            },
            32 => {  // D
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Right, &cam.θ, move_amount)
                }
            },
            46 => {  // C
                match cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => cam.position += &move_camera(MoveDirection::Down, &cam.θ, move_amount)
                }
            },
            29 => {  // Lctrl
                match cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => cam.position += &move_camera(MoveDirection::Down, &cam.θ, move_amount)
                }
            },
            57 => {  // Space
                match cam_type {
                    CameraType::Single => (),
                    CameraType::FPS => (),
                    _ => cam.position += &move_camera(MoveDirection::Up, &cam.θ, move_amount)
                }
            },
            33 => {  // F
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Kata, &cam.θ, move_amount)
                }
            },
            19 => {  // R
                match cam_type {
                    CameraType::Single => (),
                    _ => cam.position += &move_camera(MoveDirection::Ana, &cam.θ, move_amount)
                }
            },

            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            75 => {  // Left
                match cam_type {
                    CameraType::Single => shape.orientation[2] -= rotate_amount,
                    _ => cam.θ[2] -= rotate_amount
                }
            },
            77 => {  // Right
                match cam_type {
                    CameraType::Single => shape.orientation[2] += rotate_amount,
                    _ => cam.θ[2] += rotate_amount
                }
            },
            // Don't allow us to look greater than τ/4 up or down.
            80 => {  // Down
                match cam_type {
                    CameraType::Single => shape.orientation[1] -= rotate_amount,
                    _ => cam.θ[1] -= rotate_amount
                }
            },
            72 => {  // Up
                match cam_type {
                    CameraType::Single => shape.orientation[1] += rotate_amount,
                    _ => cam.θ[1] += rotate_amount
                }
            },
            16 => {  // Q
                match cam_type {
                    CameraType::Single => shape.orientation[0] -= rotate_amount,
                    _ => cam.θ[0] -= rotate_amount
                }
            },
            18 => {  // E
                match cam_type {
                    CameraType::Single => shape.orientation[0] += rotate_amount,
                    _ => cam.θ[0] += rotate_amount
                }
            },

            // 4d rotations
            82 => {  // Ins
                match cam_type {
                    CameraType::Single => shape.orientation[3] += rotate_amount,
                    _ => cam.θ[3] += rotate_amount
                }
            },
            83 => {  // Del
                match cam_type {
                    CameraType::Single => shape.orientation[3] -= rotate_amount,
                    _ => cam.θ[3] -= rotate_amount
                }
            },
            71 => {  // Home
                match cam_type {
                    CameraType::Single => shape.orientation[4] += rotate_amount,
                    _ => cam.θ[4] += rotate_amount
                }
            },
            79 => {  // End
                match cam_type {
                    CameraType::Single => shape.orientation[4] -= rotate_amount,
                    _ => cam.θ[4] -= rotate_amount
                }
            },
            73 => {  // Pgup
                match cam_type {
                    CameraType::Single => shape.orientation[5] += rotate_amount,
                    _ => cam.θ[5] += rotate_amount
                }
            },
            81 => {  // Pgdn
                match cam_type {
                    CameraType::Single => shape.orientation[5] -= rotate_amount,
                    _ => cam.θ[5] -= rotate_amount
                }
            },

            // todo reimplement some of these
//            Keycode::V => cam.near -= 1. * ZOOM_SENSITIVITY,
//            Keycode::B => cam.near += 1. * ZOOM_SENSITIVITY,
//
//            Keycode::N => cam.far -= 1. * ZOOM_SENSITIVITY,
//            Keycode::M => cam.far += 1. * ZOOM_SENSITIVITY,
//
//            Keycode::Minus => cam.fov += 1. * ZOOM_SENSITIVITY,
//            Keycode::Equals => cam.fov -= 1. * ZOOM_SENSITIVITY,
//
//            // reset
//            Keycode::Backspace => cam = DEFAULT_CAMERA(),
            _ => (),
        }
    }

}