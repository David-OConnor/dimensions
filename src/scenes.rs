// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;
use rand;
use simdnoise;

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};

const τ: f32 = 2. * PI;
const SHAPE_OP: f32 = 0.3;

const base_lighting: Lighting = Lighting {
            ambient_intensity: 0.6,
            diffuse_intensity: 0.9,
            specular_intensity: 0.2,
            ambient_color: [1.0, 1.0, 1.0, 0.4],
            diffuse_color: [0., 1., 0., 0.2],
            diffuse_direction: [0., 0., -1., 0.],
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
            far: 600.,
            strange: 1.,
        },
        cam_type: CameraType::Single,
        color_max: 0.4,
        lighting: base_lighting.clone(),
    }
}

pub fn hypercube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_hypercube(1., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn fivecell_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_5cell(2., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn cube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_cube(1., array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn pyramid_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, shape_maker::make_rectangular_pyramid((1., 1., 1.), array![0., 0., 0., 0.],
        Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn world_scene(aspect: f32) -> Scene {
    let terrain_res = 200;
    let terrain_size = 300.;
    let max_alt = 10.;
    let n_shapes = 50;
    let max_size = 10.;
    let max_rot_speed = 0.3;

    // https://docs.rs/simdnoise/2.3.1/simdnoise/enum.NoiseType.html
    let noise_type1 = simdnoise::NoiseType::Fbm {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 3,
    };

    let noise_type2 = simdnoise::NoiseType::Fbm {
        freq: 0.02,
        lacunarity: 0.5,
        gain: 3.0,
        octaves: 3,
    };

    let height_map = simdnoise::get_2d_scaled_noise(
        0., terrain_res, 0., terrain_res, noise_type1, -15., 15.
    );
    let spiss_map = simdnoise::get_2d_scaled_noise(
        0., terrain_res, 0., terrain_res, noise_type2, -10., 10.
    );

    let height_map_2d = Array::from_shape_vec((terrain_res, terrain_res), height_map).unwrap();
    let spiss_map_2d = Array::from_shape_vec((terrain_res, terrain_res), spiss_map).unwrap();

    let mut shape_list = Vec::new();
    shape_list.push(shape_maker::make_terrain((terrain_size, terrain_size), terrain_res as u32,
                                              height_map_2d, spiss_map_2d, array![0., -1., 0., 0.], 1.));

    for i in 0..n_shapes {
        let shape_type = rand::random::<f32>();
        let position = array![
            (rand::random::<f32>() - 0.5) * terrain_size,
            (rand::random::<f32>() - 0.5) * max_alt * 2.,
            (rand::random::<f32>() - 0.5) * terrain_size,
            (rand::random::<f32>() - 0.5) * max_alt * 2.
        ];

        let rotation = array![
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
        ];

        if shape_type < 0.25 {
            let lens = (
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size
            );
            shape_list.push(shape_maker::make_box(lens, position,
                                  Array::zeros(6), rotation, SHAPE_OP))
        } else if shape_type < 0.5 {
            let lens = (
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size,
            );
            shape_list.push(shape_maker::make_rectangular_pyramid(lens, position,
                                                  rotation, Array::zeros(6), SHAPE_OP))
        } else if shape_type < 0.75 {
            let lens = (
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size,
                rand::random::<f32>() * max_size
            );
            shape_list.push(shape_maker::make_hyperrect(lens, position,
                                        Array::zeros(6), rotation, SHAPE_OP))
        } else {
//            shape_list.push(shape_maker::make_5cell(rand::random::<f32>() * max_size, position,
//                                    Array::zeros(6), rotation, SHAPE_OP))
        }
    }

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

    let grid_size: usize = 16;
    let grid = Array3::zeros((grid_size, grid_size, grid_size));
    let shapes = shape_maker::make_hypergrid((200., 200., 200.), grid_size as u32,
                                              grid, Array::zeros(6), SHAPE_OP);

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