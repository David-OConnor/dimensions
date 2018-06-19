// Handles keyboard and mouse input.

use ndarray::prelude::*;

use types::Camera;

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

pub fn handle_pressed(pressed: &Vec<u32>, delta_t: f32,
                      move_sensitivity: f32, rotate_sensitivity: f32,
                      cam: &mut Camera) -> () {
    // Add if it's not already there.
    let move_amount = move_sensitivity * delta_t;
    let rotate_amount = rotate_sensitivity * delta_t;

    for code in pressed {
        // Some of the entries appear for reversed, for reasons I don't
        // understand yet.
        match *code {
            17 => {  // W
                cam.position += &move_camera(MoveDirection::Forward, &cam.θ, move_amount);
            },
            31 => {  // S
                cam.position += &move_camera(MoveDirection::Back, &cam.θ, move_amount);
            },
            30 => {  // A
                cam.position += &move_camera(MoveDirection::Left, &cam.θ, move_amount);
            },
            32 => {  // D
                cam.position += &move_camera(MoveDirection::Right, &cam.θ, move_amount);
            },
            46 => {  // C
                cam.position += &move_camera(MoveDirection::Down, &cam.θ, move_amount);
            },
            29 => {  // Lctrl
                cam.position += &move_camera(MoveDirection::Down, &cam.θ, move_amount);
            },
            57 => {  // Space
                cam.position += &move_camera(MoveDirection::Up, &cam.θ, move_amount);
            },
            33 => {  // F
                cam.position += &move_camera(MoveDirection::Kata, &cam.θ, move_amount);
            },
            19 => {  // R
                cam.position += &move_camera(MoveDirection::Ana, &cam.θ, move_amount);
            },

            // Rotations around Y and Z range from 0 to τ. (clockwise rotation).
            // X rotations range from -τ/4 to τ/4 (Looking straight down to up)
            75 => {  // Left
                cam.θ[2] += rotate_amount;  // todo why reverse?
            },
            77 => {  // Right
                cam.θ[2] -= rotate_amount;
            },
            // Don't allow us to look greater than τ/4 up or down.
            80 => {  // Down
                cam.θ[1] -= rotate_amount;
            },
            72 => {  // Up
                cam.θ[1] += rotate_amount;
            },

            16 => {  // Q
                cam.θ[0] -= rotate_amount;
            },
            18 => {  // E
                cam.θ[0] += rotate_amount;
            },

            // 4d rotations
            82 => cam.θ[3] += rotate_amount,  // Ins
            83 => cam.θ[3] -= rotate_amount,  // Del
            71 => cam.θ[4] += rotate_amount,  // Home
            79 => cam.θ[4] -= rotate_amount,  // End
            73 => cam.θ[5] += rotate_amount,  // PgUp
            81 => cam.θ[5] -= rotate_amount,  // PgDn

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