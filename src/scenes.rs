// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;

const base_lighting: Lighting = Lighting {
            ambient_intensity: 0.6,
            diffuse_intensity: 0.4,
            specular_intensity: 0.3,
            ambient_color: [1.0, 1.0, 1.0, 0.4],
            diffuse_color: [0., 1., 0., 0.2],
            diffuse_direction: [-1., 0., 0., 0.],
};

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
        color_max: 0.4,
        lighting: base_lighting.clone(),
    }
}

pub fn hypercube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_hypercube(1., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6)))
}

pub fn fivecell_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_5cell(2., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6)))
}

pub fn cube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_cube(1., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6)))
}

pub fn world_scene(aspect: f32) -> Scene {
    let shape_list = vec![
//        shape_maker.make_terrain([20, 20], 10, height_map, spiss_map, Array::zeros(6)),

        shape_maker::make_box((1., 2., 1.), array![-1., 3., 4., 1.],
                              Array::zeros(6), array![0.1, 0.07, 0., 0., 0., 0.]),
        shape_maker::make_rectangular_pyramid((2., 2., 2.), array![-2., 3., 3., -1.],
                                              array![τ/6., τ/3., 0., 0., 0., 0.], Array::zeros(6)),
        shape_maker::make_cube(1., array![2., 0., 5., 2.],
                               Array::zeros(6), array![0.2, 0., 0., 0., 0., 0.]),
        // On ana of other cube.
        shape_maker::make_cube(1., array![2., 0., 5., 10.],
                               Array::zeros(6), array![0.2, 0., 0., 0., 0., 0.]),
        shape_maker::make_hypercube(1., array![3., 3., 3., 0.],
                                    Array::zeros(6), array![0., 0., 0., 0.05, 0.05, 0.1]),
        shape_maker::make_hypercube(1., array![-3., 1., 0., 1.5],
                                    Array::zeros(6), Array::zeros(6)),
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
            fov: τ / 5.,
            aspect,
            aspect_4: 1.,
            near: 0.1,
            far: 200.,
            strange: 1.,
        },
        cam_type: CameraType::Free,
        color_max: 10.,
        lighting: base_lighting.clone(),
    }
}

//fn make_3d_grid_empty(size: u32) -> Array3<f32> {
//
//    let mut result = Array3::zeros((size, size, size));
//
//    let mut outer = Vec::new();
//    for i in 0..size {
//        let mut middle = Vec::new();
//        for j in 0..size {
//            let mut inner = Vec::new();
//            for k in 0..size {
//                inner.push(0);
//                result[[i, j, k]] =
//            }
//            middle.push(inner)
//        }
//        outer.push(middle)
//    }
//   Array3::from_vec(outer)
//}

pub fn grid_scene(aspect: f32) -> Scene {

    let grid_size: usize = 140;
    let grid = Array3::zeros((grid_size, grid_size, grid_size));
    let shapes = shape_maker::make_hypergrid((2000., 2000., 2000.), grid_size as u32,
                                              grid, Array::zeros(6));

    Scene {
        id: 3,
        shapes,
        cam_start: Camera {
            position: array![0., 0., -1., 0.],
            θ: array![0., 0., 0., 0., 0., 0.],
            fov: τ / 5.,
            aspect,
            aspect_4: 1.,
            near: 0.1,
            far: 2000.,
            strange: 1.,
        },
        cam_type: CameraType::Free,
        color_max: 100.,
        lighting: base_lighting.clone(),
    }

}