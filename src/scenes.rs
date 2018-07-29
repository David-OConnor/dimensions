// Set up different combinations of shapes, camera, adn other variables.
use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;
use rand;
//use simdnoise;
use noise::{NoiseFn, Perlin};

use shape_maker;
use types::{Camera, Lighting, Scene, Shape, CameraType};
use util;

const τ: f32 = 2. * PI;
const SHAPE_OP: f32 = 0.3;

const base_lighting: Lighting = Lighting {
        ambient_intensity: 0.8,
        diffuse_intensity: 0.6,
        ambient_color: [1.0, 1.0, 1.0, 0.6],
        diffuse_color: [1., 1., 1., 1.0],
        diffuse_direction: [0., 0., -1., 0.],
        sources: Vec::new(),
};

fn base_camera() -> Camera {
    // function instead of a const, due to the ndarrays.
    Camera {
        position: Array::zeros(4),
        θ: Array::zeros(6),
        fov: τ / 5.,
        aspect: 1.,
        aspect_4: 1.,
        near: 0.05,
        far: 600.,
        fourd_proj_dist: 0.,
    }
}

fn make_single_scene(aspect: f32, shape: Shape) -> Scene {
    let mut shapes = HashMap::new();
    shapes.insert(0, shape);
    Scene {
        shapes,
        cam: Camera {
            position: array![0., 0., -4., 0.],
            θ: array![0., 0., τ / 2., 0., 0., 0.],
            fov: τ / 5.5,
            aspect,
            fourd_proj_dist: 0.5,
            ..base_camera()
        },
        cam_type: CameraType::Single,
        color_max: 0.4,
        lighting: base_lighting,
        sensitivities: (0., 0.5, 0.2),
    }
}

pub fn hypercube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::make_hypercube(1.), Array::zeros(4),
        Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn fivecell_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::fivecell(2.), Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn twentyfourcell_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::twentyfourcell(2.), Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn spherinder_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::spherinder((3., 0.5), 64),
                                         Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn origin_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::origin((1., 0.1), 32),
                                         Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn cube_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::cube(1.), Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn pyramid_scene(aspect: f32) -> Scene {
    make_single_scene(aspect, Shape::new(shape_maker::rect_pyramid((1., 1., 1.)), Array::zeros(4),
                                         Array::zeros(6), Array::zeros(6), SHAPE_OP))
}

pub fn world_scene(aspect: f32) -> Scene {
    let terrain_res = 100;
    let terrain_size = 400.;
    let max_alt = 10.;
    let max_spiss = 40.;
    let n_shapes = 10;
    let max_size = 10.;
    let max_rot_speed = 0.2;

    // simdnoise is incompatible with wasm-32-unknown-unknown.
    // It appears to be a nicer library than noise.
//    // https://docs.rs/simdnoise/2.3.1/simdnoise/enum.NoiseType.html
//    let noise_type1 = simdnoise::NoiseType::Fbm {
//        freq: 0.04,
//        lacunarity: 0.5,
//        gain: 2.0,
//        octaves: 3,
//    };
//
//    let noise_type2 = simdnoise::NoiseType::Fbm {
//        freq: 0.02,
//        lacunarity: 0.5,
//        gain: 3.0,
//        octaves: 3,
//    };

//    let height_map = simdnoise::get_2d_scaled_noise(
//        0., terrain_res, 0., terrain_res, noise_type1, -15., 15.
//    );
//    let spiss_map = simdnoise::get_2d_scaled_noise(
//        0., terrain_res, 0., terrain_res, noise_type2, -10., 10.
//    );
//
//    let height_map_2d = Array::from_shape_vec((terrain_res, terrain_res), height_map).unwrap();
//    let spiss_map_2d = Array::from_shape_vec((terrain_res, terrain_res), spiss_map).unwrap();

//
////    let noise1 = noise::OpenSimplex::new();
//    let noise1a = Perlin::new();
//    let noise2a = Perlin::new();

//    let test = noise1a.get([1., 1.]);


//    let perlin = noise1a.set_seed(1);
//    println!("PERLIN: {:?}", &noise1a);
//    noise::utils::PlaneMapBuilder::new(&perlin).build();

//    let height_map_2d = Array::zeros((terrain_size as u32, terrain_size as u32));
//    let spiss_map_2d = Array::zeros((terrain_size as u32, terrain_size as u32));
//
//    for i in 0..terrain_size as u32 {
//        for j in 0..terrain_size as u32 {  // todo is this right? Docs for noise aren't great.
//            height_map_2d[[i, j]] = noise1.get([i as f32, j as f32]);
//            spiss_map_2d[[i, j]] = noise2.get([i as f32, j as f32]);
//        }
//    }

    let height_map_2d = Array::zeros((terrain_res, terrain_res));
    let spiss_map_2d = Array::zeros((terrain_res, terrain_res));

    let mut shape_list = Vec::new();
    shape_list.push(Shape::new(shape_maker::terrain((terrain_size, terrain_size), terrain_res as u32,
                                                    height_map_2d, spiss_map_2d),
                    array![0., -1., 0., 0.], Array::zeros(6), Array::zeros(6), 1.));

    for i in 0..n_shapes {
//        let shape_type = rand::random::<f32>();
//        let shape_type = wbg_rand::wasm_rng().gen::<f32>();
//        let position = array![
//            (rand::random::<f32>() - 0.5) * terrain_size,
//            (rand::random::<f32>() - 0.5) * max_alt * 2.,
//            (rand::random::<f32>() - 0.5) * terrain_size,
//            (rand::random::<f32>() - 0.5) * max_spiss * 2.
//        ];
//
//        let rotation = array![
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//            (rand::random::<f32>() - 0.5) * max_rot_speed * 2.,
//        ];
//
//        if shape_type < 0.2 {
//            let lens = (
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size
//            );
//            shape_list.push(Shape::new(shape_maker::box_(lens), position,
//                                       Array::zeros(6), rotation, SHAPE_OP))
//        } else if shape_type < 0.4 {
//            let lens = (
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//            );
//            shape_list.push(Shape::new(shape_maker::rect_pyramid(lens), position,
//                                       rotation, Array::zeros(6), SHAPE_OP))
//        } else if shape_type < 0.6 {
//            let lens = (
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size
//            );
//            shape_list.push(Shape::new(shape_maker::hyperrect(lens), position,
//                                       Array::zeros(6), rotation, SHAPE_OP))
//        } else if shape_type < 0.8 {
//            let lens = (
//                rand::random::<f32>() * max_size,
//                rand::random::<f32>() * max_size,
//            );
//            shape_list.push(Shape::new(shape_maker::spherinder(
//                lens, 20),
//                position,
//                Array::zeros(6), rotation, SHAPE_OP)
//            )
//        } else {
//            shape_list.push(Shape::new(shape_maker::fivecell(rand::random::<f32>() * max_size), position,
//                                       Array::zeros(6), rotation, SHAPE_OP))
//        }
    }

    let mut shapes = HashMap::new();
    for (id, shape) in shape_list.into_iter().enumerate() {
        shapes.insert(id as u32, shape);
    }

    Scene {
        shapes,
        cam: Camera {
            position: array![0., 0., 0., 0.],
            θ: array![0., 0., 0., 0., 0., 0.],
            aspect,
            ..base_camera()
        },
        cam_type: CameraType::Free,
        color_max: 10.,
        lighting: base_lighting,
        sensitivities: (5., 0.2, 0.2),
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

    let grid_size: usize = 12;
    let grid = Array3::zeros((grid_size, grid_size, grid_size));
    let shapes = shape_maker::hypergrid((200., 200., 200.), grid_size as u32,
                                        grid);

    Scene {
        shapes,
        cam: Camera {
            position: array![0., 0., -1., 0.],
            aspect,
            ..base_camera()
        },
        cam_type: CameraType::Free,
        color_max: 100.,
        lighting: base_lighting,
        sensitivities: (5., 0.5, 0.2),
    }
}

pub fn grid_scene_4d(aspect: f32) -> Scene {
    let grid_size: usize = 6;
    let shapes = shape_maker::grid_4d((200., 200., 200., 200.), grid_size as u32);

    Scene {
        shapes,
        cam: Camera {
            position: array![0., 0., -1., 0.],
            aspect,
            ..base_camera()
        },
        cam_type: CameraType::Free,
        color_max: 150.,
        lighting: base_lighting,
        sensitivities: (5., 0.5, 0.2),
    }
}

fn mult_cplx(num1: (f32, f32), num2: (f32, f32)) -> (f32, f32) {
    let real = num1.0 * num2.0 - num1.1 * num2.1;
    let im = num1.0 * num2.1 + num1.1 * num2.0;
    (real, im)
}

fn pow_cplx(num: (f32, f32), pow: i32) -> (f32, f32) {
    let mut result = (1., 0.);
    for i in 0..pow {
        result = mult_cplx(result, num);
    }
    result
}

pub fn plot_scene(aspect: f32) -> Scene {
    // Plot a 4d function, with 2 inputs and 2 outputs.
    // Test func: f(x) = x^2 + 1, in complex plane.
    // X: input real
    // Y: input imag
    // Z: output real
    // W: output imag

    fn f(r: f32, im: f32) -> (f32, f32) {
        pow_cplx((r, im), 2)
    }

    let res = 100;

    let (mut height_grid, mut spiss_grid) = (
        Array2::zeros((res, res)),
        Array2::zeros((res, res))
    );

    let scaler = 0.2;

    // These ranges correspond to function inputs, not spacial dim. as do the
    // x and y vars in the grid-generating loop below.
    let x_range = (-4., 4.);
    let y_range = (-4., 4.);

    for i in 0..res {
        let x = util::value_from_grid(i as u32, res as u32, x_range);
        for j in 0..res {
            let y = util::value_from_grid(j as u32, res as u32, y_range);
            let result = f(x, y);
            height_grid[[i, j]] = result.0 * scaler;
            spiss_grid[[i, j]] = result.1 * scaler;
        }
    }

    let mut plot = shape_maker::terrain((10., 10.), res as u32,
                                                      height_grid, spiss_grid);

    let origin = shape_maker::origin((4., 0.1), 10);
    plot = shape_maker::combine_meshes(plot, vec![(origin, [0., 0., 0., 0.])]);

    let mut shapes = HashMap::new();
        shapes.insert(0, Shape::new(plot, Array::zeros(4), Array::zeros(6), Array::zeros(6), 1.));


    Scene {
        shapes,
        cam: Camera {
            aspect,
            position: array![0., 0., -15., 0.],
            θ: array![0., 0., τ / 2., 0., 0., 0.],
            ..base_camera()
        },
        cam_type: CameraType::Single,
        color_max: 10.,
        lighting: base_lighting,
        sensitivities: (5., 0.5, 0.2),
    }
}