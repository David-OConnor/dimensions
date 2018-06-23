// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;

use shape_maker;
use types::{Camera, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;


fn make_single_scene(aspect: f32, shape: Shape) -> Scene {
    let mut shapes = HashMap::new();
    shapes.insert(0, shape);
    Scene {
        id: 0,
        shapes,
        cam_start: Camera {
            position: array![0., 0., -3., 0.],
            θ: array![0., 0., τ / 2., 0., 0., 0.],
            fov: τ / 5.5,
            aspect,
            aspect_4: 1.,
            near: 0.1,
            far: 200.,
            strange: 1.,
        },
        cam_type: CameraType::Single,
        color_max: 0.4
    }
}

pub fn hypercube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_hypercube(1., array![0., 0., 0., 0.],
        array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.]))
}

pub fn fivecell_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_5cell(2., array![0., 0., 0., 0.],
        array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.]))
}

pub fn cube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_cube(1., array![0., 0., 0., 0.],
        array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.]))
}

pub fn world_scene(aspect: f32) -> Scene {
    let shape_list = vec![
    //        shape_maker.make_terrain([20, 20], 10, heightMap, spissMap, EMPTY),

    shape_maker::make_box((1., 2., 1.), array![-1., 3., 4., 1.],
                          array![0., 0., 0., 0., 0., 0.], array![0.1, 0.07, 0., 0., 0., 0.]),
    shape_maker::make_rectangular_pyramid((2., 2., 2.), array![-2., 3., 3., -1.],
                                          array![τ/6., τ/3., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.]),
    shape_maker::make_cube(1., array![2., 0., 5., 2.],
                           array![0., 0., 0., 0., 0., 0.], array![0.2, 0., 0., 0., 0., 0.]),
    // On ana of other cube.
    shape_maker::make_cube(1., array![2., 0., 5., 10.],
                           array![0., 0., 0., 0., 0., 0.], array![0.2, 0., 0., 0., 0., 0.]),
    shape_maker::make_hypercube(1., array![3., 3., 3., 0.],
                                array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0.05, 0.05, 0.1]),
    shape_maker::make_hypercube(1., array![-3., 1., 0., 1.5],
                                array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.]),
        ];

    let mut shapes = HashMap::new();
    for (id, shape) in shape_list.into_iter().enumerate() {
        shapes.insert(id as u32, shape);
    }

    Scene {
        id: 1,
        shapes,
        cam_start: Camera {
            position: array![0., 2., -3., 0.],
            θ: array![0., 0., τ / 2., 0., 0., 0.],
            fov: τ / 4.,
            aspect,
            aspect_4: 1.,
            near: 0.1,
            far: 200.,
            strange: 1.,
        },
        cam_type: CameraType::Free,
        color_max: 10.
    }
}