// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time;
use std;

use vulkano::buffer;
use vulkano::command_buffer;
use vulkano::descriptor;
use vulkano::device;
use vulkano::format::{ClearValue, Format};
use vulkano::format;
use vulkano::framebuffer;
use vulkano::image;
use vulkano::instance;
use vulkano::memory;
use vulkano::pipeline;
use vulkano::swapchain;
use vulkano::sync;
use vulkano::sync::GpuFuture;

use cgmath;
//use ndarray::prelude::*;
//use vulkano;
use vulkano_shader_derive;
use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use winit;

use input;
use scenes;
use shaders;
use shape_maker;
use transforms;
use types::{Camera, Shape, Vertex, VertAndExtras, Normal};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const MOVE_SENSITIVITY: f32 = 1.0;  // units per second
const ROTATE_SENSITIVITY: f32 = 0.8;  // radians per second

const τ: f32 = 2. * PI;
//
//mod state {
//    use std::cell;
//    use std::collections::HashMap;
//    use std::f32::consts::PI;
//    use std::sync::{Arc, Mutex, RwLock};
//
//    use ndarray::prelude::*;
//
//    use types::{Camera, Shape};
//
//    const τ: f32 = 2. * PI;
//
//    fn default_camera() -> Camera {
//        // Effectively a global constant.
//        Camera {
//            // If 3d, the 4th items for position isn't used.
//            position: array![0., 1., -6., -2.],
//            θ: array![0., 0., 0., 0., 0., 0.],
//            fov: τ / 5.,
//            aspect: 1.,
//            aspect_4: 1.,
//            far: 50.,
//            near: 0.1,
//            strange: 1.0,
//        }
//    }
//
//    struct State_ {
//        cam: Camera,
//        shapes: HashMap<u32, Shape>,
//    }
//
////    let state1 = State_ {
////        cam: default_camera(),
////        shapes: HashMap::new(),
////    };
//
////    let data = Arc::new(Mutex::new(0));
//
//}

fn print_type_of<T>(_: &T) {
    // For debugging.
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

pub fn make_static_buffers(shapes: &HashMap<u32, Shape>, device: Arc<device::Device>) ->
        Arc<buffer::cpu_access::CpuAccessibleBuffer<[u32]>> {
    let index_buffer = {
        let mut indices = Vec::new();
        let mut indexModifier = 0;

        for (s_id, shape) in shapes {
            let mut tri_indices: Vec<u32> = shape.tris.iter().map(|ind| ind + indexModifier).collect();
            indices.append(&mut tri_indices);
            indexModifier += shape.num_face_verts();

        }
        buffer::cpu_access::CpuAccessibleBuffer::from_iter(device, buffer::BufferUsage::all(),
                                                           indices.iter().cloned())
            .expect("Failed to create index buffer")
    };

    index_buffer
}

pub fn make_per_frame_buffers(shapes: &HashMap<u32, Shape>, cam: &Camera, device: Arc<device::Device>) -> (
        Arc<buffer::cpu_access::CpuAccessibleBuffer<[VertAndExtras]>>,
        Arc<buffer::cpu_access::CpuAccessibleBuffer<[Normal]>>
    ){
//pub fn make_per_frame_buffers(shapes: &HashMap<u32, Shape>, cam: &Camera, device: Arc<device::Device>) -> (

//    ){
    let mut shape_vertices = Vec::new();
    let mut normals = Vec::new();
    for (s_id, shape) in shapes {
        for face in &shape.faces_vert {
            for id in face {
//                shape_vertices.push(shape.vertices[id].unwrap().clone());
                let v = shape.vertices[id].position;
                let info = VertAndExtras {
                    position: (v.0, v.1, v.2, v.3),
                    shape_posit: (shape.position[0], shape.position[1],
                                   shape.position[2], shape.position[3]),
                    cam_posit: (cam.position[0], cam.position[1],
                                 cam.position[2], cam.position[3]),
                };
                shape_vertices.push(info);

                normals.push(shape.normals[id].clone());
            }
        }
    }

     // todo .cloned ?
    let vertex_buffer = buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device.clone(), buffer::BufferUsage::all(), shape_vertices.iter().cloned())
        .expect("failed to create vertex buffer");

    let normals_buffer = buffer::cpu_access::CpuAccessibleBuffer::from_iter(
        device, buffer::BufferUsage::all(), normals.iter().cloned())
        .expect("failed to create buffer");

    (vertex_buffer, normals_buffer)
}


pub fn render(shapes: HashMap<u32, Shape>) {
    // todo for now, we'll keep state in this func.
    // todo sync aspect with window dims.

    let scene = scenes::world_scene();


    let mut cam = scene.cam_start.clone();
    let mut shapes = scene.shapes.clone();

    for (id, shape) in &mut shapes {
        shape.make_tris();
    }

    let mut currently_pressed: Vec<u32> = Vec::new();

    // The first step of any vulkan program is to create an instance.
    let instance = {
        // When we create an instance, we have to pass a list of extensions that we want to enable.
        //
        // All the window-drawing functionalities are part of non-core extensions that we need
        // to enable manually. To do so, we ask the `vulkano_win` crate for the list of extensions
        // required to draw to a window.
        let extensions = vulkano_win::required_extensions();

        // Now creating the instance.
        instance::Instance::new(None, &extensions, None)
            .expect("failed to create Vulkan instance")
    };

    // We then choose which physical device to use.
    //
    // In a real application, there are three things to take into consideration:
    //
    // - Some devices may not support some of the optional features that may be required by your
    //   application. You should filter out the devices that don't support your app.
    //
    // - Not all devices can draw to a certain surface. Once you create your window, you have to
    //   choose a device that is capable of drawing to it.
    //
    // - You probably want to leave the choice between the remaining devices to the user.
    //
    // For the sake of the example we are just going to use the first device, which should work
    // most of the time.
    let physical = instance::PhysicalDevice::enumerate(&instance)
        .next().expect("no device available");
    // Some little debug infos.
    println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

    // The objective of this example is to draw a triangle on a window. To do so, we first need to
    // create the window.
    //
    // This is done by creating a `WindowBuilder` from the `winit` crate, then calling the
    // `build_vk_surface` method provided by the `VkSurfaceBuild` trait from `vulkano_win`. If you
    // ever get an error about `build_vk_surface` being undefined in one of your projects, this
    // probably means that you forgot to import this trait.
    //
    // This returns a `vulkano::swapchain::Surface` object that contains both a cross-platform winit
    // window and a cross-platform Vulkan surface that represents the surface of the window.
    let mut events_loop = winit::EventsLoop::new();
    let surface = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    // The next step is to choose which GPU queue will execute our draw commands.
    //
    // Devices can provide multiple queues to run commands in parallel (for example a draw queue
    // and a compute queue), similar to CPU threads. This is something you have to have to manage
    // manually in Vulkan.
    //
    // In a real-life application, we would probably use at least a graphics queue and a transfers
    // queue to handle data transfers in parallel. In this example we only use one queue.
    //
    // We have to choose which queues to use early on, because we will need this info very soon.
    let queue = physical.queue_families().find(|&q| {
        // We take the first queue that supports drawing to our window.
        q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
    }).expect("couldn't find a graphical queue family");

    // Now initializing the device. This is probably the most important object of Vulkan.
    //
    // We have to pass five parameters when creating a device:
    //
    // - Which physical device to connect to.
    //
    // - A list of optional features and extensions that our program needs to work correctly.
    //   Some parts of the Vulkan specs are optional and must be enabled manually at device
    //   creation. In this example the only thing we are going to need is the `khr_swapchain`
    //   extension that allows us to draw to a window.
    //
    // - A list of layers to enable. This is very niche, and you will usually pass `None`.
    //
    // - The list of queues that we are going to use. The exact parameter is an iterator whose
    //   items are `(Queue, f32)` where the floating-point represents the priority of the queue
    //   between 0.0 and 1.0. The priority of the queue is a hint to the implementation about how
    //   much it should prioritize queues between one another.
    //
    // The list of created queues is returned by the function alongside with the device.
    let (device_, mut queues) = {
        let device_ext = device::DeviceExtensions {
            khr_swapchain: true,
            .. device::DeviceExtensions::none()
        };

        device::Device::new(physical, physical.supported_features(), &device_ext,
                                     [(queue, 0.5)].iter().cloned()).expect("failed to create device")
    };

    // Since we can request multiple queues, the `queues` variable is in fact an iterator. In this
    // example we use only one queue, so we just retreive the first and only element of the
    // iterator and throw it away.
    let queue = queues.next().unwrap();

    // The dimensions of the surface.
    // This variable needs to be mutable since the viewport can change size.
    let mut dimensions;

    // Before we can draw on the surface, we have to create what is called a swapchain. Creating
    // a swapchain allocates the color buffers that will contain the image that will ultimately
    // be visible on the screen. These images are returned alongside with the swapchain.
    let (mut swapchain_, mut images) = {
        // Querying the capabilities of the surface. When we create the swapchain we can only
        // pass values that are allowed by the capabilities.
        let caps = surface.capabilities(physical)
            .expect("failed to get surface capabilities");

        dimensions = caps.current_extent.unwrap_or([WIDTH, HEIGHT]);

        // We choose the dimensions of the swapchain to match the current extent of the surface.
        // If `caps.current_extent` is `None`, this means that the window size will be determined
        // by the dimensions of the swapchain, in which case we just use the width and height defined above.

        let usage = caps.supported_usage_flags;
        // The alpha mode indicates how the alpha value of the final image will behave. For example
        // you can choose whether the window will be opaque or transparent.
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        // Choosing the internal format that the images will have.
        let format = caps.supported_formats[0].0;

        // Please take a look at the docs for the meaning of the parameters we didn't mention.
        swapchain::Swapchain::new(device_.clone(), surface.clone(), caps.min_image_count, format,
                                  dimensions, 1, usage, &queue,
                                  swapchain::SurfaceTransform::Identity, alpha,
                                  swapchain::PresentMode::Fifo, true,
                                  None).expect("failed to create swapchain")
    };

    let depth_buffer = image::attachment::AttachmentImage::transient(
        device_.clone(), dimensions, format::D16Unorm).unwrap();

    let index_buffer = make_static_buffers(&shapes, device_.clone());

    // todo move depth_buffer and unifform buffer to one of the make_buffer funcs.

    let proj_mat = transforms::make_proj_mat4(&cam);
    let proj_mat2 = cgmath::perspective(cgmath::Rad(cam.fov), cam.aspect, cam.near, cam.far);

    let uniform_buffer = buffer::cpu_pool::CpuBufferPool::<shaders::vs::ty::Data>
                         ::new(device_.clone(), buffer::BufferUsage::all());

    // The next step is to create the shaders.
    //
    // The raw shader creation API provided by the vulkano library is unsafe, for various reasons.
    //
    // An overview of what the `VulkanoShader` derive macro generates can be found in the
    // `vulkano-shader-derive` crate docs. You can view them at
    // https://docs.rs/vulkano-shader-derive/*/vulkano_shader_derive/
    //
    // TODO: explain this in details

    let vs = shaders::vs::Shader::load(device_.clone()).expect("failed to create shader module");
    let fs = shaders::fs::Shader::load(device_.clone()).expect("failed to create shader module");

    // At this point, OpenGL initialization would be finished. However in Vulkan it is not. OpenGL
    // implicitely does a lot of computation whenever you draw. In Vulkan, you have to do all this
    // manually.

    // The next step is to create a *render pass*, which is an object that describes where the
    // output of the graphics pipeline will go. It describes the layout of the images
    // where the colors, depth and/or stencil information will be written.
    let render_pass = Arc::new(
        single_pass_renderpass!(device_.clone(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `load: Clear` means that we ask the GPU to clear the content of this
                    // attachment at the start of the drawing.
                    load: Clear,
                    // `store: Store` means that we ask the GPU to store the output of the draw
                    // in the actual image. We could also ask it to discard the result.
                    store: Store,
                    // `format: <ty>` indicates the type of the format of the image. This has to
                    // be one of the types of the `vulkano::format` module (or alternatively one
                    // of your structs that implements the `FormatDesc` trait). Here we use the
                    // generic `vulkano::format::Format` enum because we don't know the format in
                    // advance.
                    format: swapchain_.format(),
                    samples: 1,
                },
                depth: {
                load: Clear,
                store: DontCare,
                format: format::Format::D16Unorm,
                samples: 1,
                }
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {depth}
            }
        ).unwrap()
    );

    // Before we draw we have to create what is called a pipeline. This is similar to an OpenGL
    // program, but much more specific.
    let pipeline_ = Arc::new(pipeline::GraphicsPipeline::start()
        // We need to indicate the layout of the vertices.
        // The type `SingleBufferDefinition` actually contains a template parameter corresponding
        // to the type of each vertex. But in this code it is automatically inferred.
//        .vertex_input_single_buffer()
        .vertex_input(pipeline::vertex::TwoBuffersDefinition::new())
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one. The `main` word of `main_entry_point` actually corresponds to the name of
        // the entry point.
        .vertex_shader(vs.main_entry_point(), ())
        // The content of the vertex buffer describes a list of triangles.
        .triangle_list()
        // Use a resizable viewport set to draw over the entire window
        .viewports_dynamic_scissors_irrelevant(1)
        // See `vertex_shader`.
        .fragment_shader(fs.main_entry_point(), ())
        .depth_stencil_simple_depth()
        // We have to indicate which subpass of which render pass this pipeline is going to be used
        // in. The pipeline will only be usable from this particular subpass.
        .render_pass(framebuffer::Subpass::from(render_pass.clone(), 0).unwrap())
        // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
        .build(device_.clone())
        .unwrap());

    // The render pass we created above only describes the layout of our framebuffers. Before we
    // can draw we also need to create the actual framebuffers.
    //
    // Since we need to draw to multiple images, we are going to create a different framebuffer for
    // each image.
    let mut framebuffers: Option<Vec<Arc<framebuffer::Framebuffer<_,_>>>> = None;

    // Initialization is finally finished!

    // In some situations, the swapchain will become invalid by itself. This includes for example
    // when the window is resized (as the images of the swapchain will no longer match the
    // window's) or, on Android, when the application went to the background and goes back to the
    // foreground.
    //
    // In this situation, acquiring a swapchain image or presenting it will return an error.
    // Rendering to an image of that swapchain will not produce any error, but may or may not work.
    // To continue rendering, we need to recreate the swapchain by creating a new swapchain.
    // Here, we remember that we need to do this for the next loop iteration.
    let mut recreate_swapchain = false;

    // In the loop below we are going to submit commands to the GPU. Submitting a command produces
    // an object that implements the `GpuFuture` trait, which holds the resources for as long as
    // they are in use by the GPU.
    //
    // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
    // that, we store the submission of the previous frame here.
    let mut previous_frame = Box::new(sync::now(device_.clone())) as Box<sync::GpuFuture>;

    let mut prev_frame_start = time::Instant::now();
    loop {
        // delta_time is inverse frame rate. Used for making movements and
        // rotations dependent on time rather than frame rate.
        let frame_start = time::Instant::now();
        let delta_time_raw = frame_start - prev_frame_start;
        let delta_time = delta_time_raw.as_secs() as f32 + delta_time_raw.subsec_nanos() as f32 * 0.000000001;
        prev_frame_start = frame_start;

        // It is important to call this function from time to time, otherwise resources will keep
        // accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU has
        // already processed, and frees the resources that are no longer needed.
        previous_frame.cleanup_finished();

        // If the swapchain needs to be recreated, recreate it
        if recreate_swapchain {
            // Get the new dimensions for the viewport/framebuffers.
            dimensions = surface.capabilities(physical)
                .expect("failed to get surface capabilities")
                .current_extent.unwrap_or([WIDTH, HEIGHT]);

            let (new_swapchain, new_images) = match swapchain_.recreate_with_dimension(dimensions) {
                Ok(r) => r,
                // This error tends to happen when the user is manually resizing the window.
                // Simply restarting the loop is the easiest way to fix this issue.
                Err(swapchain::SwapchainCreationError::UnsupportedDimensions) => {
                    continue;
                },
                Err(err) => panic!("{:?}", err)
            };

            std::mem::replace(&mut swapchain_, new_swapchain);
            std::mem::replace(&mut images, new_images);

            framebuffers = None;

            recreate_swapchain = false;
        }

        // Because framebuffers contains an Arc on the old swapchain, we need to
        // recreate framebuffers as well.
        if framebuffers.is_none() {
            let new_framebuffers = Some(images.iter().map(|image| {
                Arc::new(framebuffer::Framebuffer::start(render_pass.clone())
                    .add(image.clone()).unwrap()
                    .add(depth_buffer.clone()).unwrap()
                    .build().unwrap())
            }).collect::<Vec<_>>());
            std::mem::replace(&mut framebuffers, new_framebuffers);
        }

        // Update the view matrix once per frame.
        let view_mat = transforms::make_view_mat4(&cam);

        let mut model_mats = HashMap::new();
        for (id, shape) in &shapes {
            model_mats.insert(*id as u32, transforms::make_model_mat4(shape));
        }
        // Updadate per-frame buffers
        let (vertex_buffer, normals_buffer) = make_per_frame_buffers(&shapes, &cam, device_.clone());

        let model_mat = transforms::make_model_mat4(&shapes[&0]);

        let uniform_buffer_subbuffer = {
            let uniform_data = shaders::vs::ty::Data {
                model: model_mat,
                view: view_mat,
                proj: proj_mat.into(),
            };

            uniform_buffer.next(uniform_data).unwrap()
        };

        // Rotate shapes.
        for (id, shape) in &mut shapes {
            shape.orientation += &(&shape.rotation_speed * delta_time);
        }

        let set = Arc::new(descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline_.clone(), 0)
            .add_buffer(uniform_buffer_subbuffer).unwrap()
            .build().unwrap()
        );

        // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
        // no image is available (which happens if you submit draw commands too quickly), then the
        // function will block.
        // This operation returns the index of the image that we are allowed to draw upon.
        //
        // This function can block if no image is available. The parameter is an optional timeout
        // after which the function call will return an error.
        let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain_.clone(),
                                                                              None) {
            Ok(r) => r,
            Err(swapchain::AcquireError::OutOfDate) => {
                recreate_swapchain = true;
                continue;
            },
            Err(err) => panic!("{:?}", err)
        };

        // In order to draw, we have to build a *command buffer*. The command buffer object holds
        // the list of commands that are going to be executed.
        //
        // Building a command buffer is an expensive operation (usually a few hundred
        // microseconds), but it is known to be a hot path in the driver and is expected to be
        // optimized.
        //
        // Note that we have to pass a queue family when we create the command buffer. The command
        // buffer will only be executable on that given queue family.
        let command_buffer_ = command_buffer::AutoCommandBufferBuilder::primary_one_time_submit(
            device_.clone(), queue.family())
            .unwrap()
            // Before we can draw, we have to *enter a render pass*. There are two methods to do
            // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
            // not covered here.
            //
            // The third parameter builds the list of values to clear the attachments with. The API
            // is similar to the list of attachments when building the framebuffers, except that
            // only the attachments that use `load: Clear` appear in the list.
            .begin_render_pass(
                framebuffers.as_ref().unwrap()[image_num].clone(), false,
                vec![
                    [0.0, 1.0, 1.0, 1.0].into(),
                1f32.into()
                ]).unwrap()

            // We are now inside the first subpass of the render pass. We add a draw command.
            //
            // The last two parameters contain the list of resources to pass to the shaders.
            // Since we used an `EmptyPipeline` object, the objects have to be `()`.

            .draw_indexed(
                pipeline_.clone(),
                command_buffer::DynamicState {
                    line_width: None,
                    viewports: Some(vec![pipeline::viewport::Viewport {
                        origin: [0.0, 0.0],
                        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                        depth_range: 0.0 .. 1.0,
                    }]),
                    scissors: None,
                },
                (vertex_buffer.clone(), normals_buffer.clone()),
                index_buffer.clone(), set.clone(), ()).unwrap()

            // We leave the render pass by calling `draw_end`. Note that if we had multiple
            // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
            // next subpass.
            .end_render_pass().unwrap()
            // Finish building the command buffer by calling `build`.
            .build().unwrap();

        let future = previous_frame.join(acquire_future)
            .then_execute(queue.clone(), command_buffer_).unwrap()

            // The color output is now expected to contain our triangle. But in order to show it on
            // the screen, we have to *present* the image by calling `present`.
            //
            // This function does not actually present the image immediately. Instead it submits a
            // present command at the end of the queue. This means that it will only be presented once
            // the GPU has finished executing the command buffer that draws the triangle.
            .then_swapchain_present(queue.clone(), swapchain_.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                previous_frame = Box::new(future) as Box<_>;
            }
            Err(sync::FlushError::OutOfDate) => {
                recreate_swapchain = true;
                previous_frame = Box::new(sync::now(device_.clone())) as Box<_>;
            }
            Err(e) => {
                println!("{:?}", e);
                previous_frame = Box::new(sync::now(device_.clone())) as Box<_>;
            }
        }

        // Note that in more complex programs it is likely that one of `acquire_next_image`,
        // `command_buffer::submit`, or `present` will block for some time. This happens when the
        // GPU's queue is full and the driver has to wait until the GPU finished some work.
        //
        // Unfortunately the Vulkan API doesn't provide any way to not wait or to detect when a
        // wait would happen. Blocking may be the desired behavior, but if you don't want to
        // block you should spawn a separate thread dedicated to submissions.

        // Handling the window events in order to close the program when the user wants to close
        // it.
        // todo take framerate into account
        let mut done = false;

        events_loop.poll_events(|ev| {
            match ev {
                winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,

                winit::Event::WindowEvent {
                    event: winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput{
                            // We use Scancode rather than virtual_keycode, since
                            // it's tied to physical key location rather than semantics;
                            // controls like this are a good example of when to
                            // use this approach.
                            scancode: code,
                            state: el_state,
                            ..
                        }, ..
                    }, ..
                } => match el_state {
                    winit::ElementState::Pressed => {
                        if !currently_pressed.contains(&code) { currently_pressed.push(code) }
                    },
                    winit::ElementState::Released => {
                        currently_pressed.remove_item(&code);
                    }
                },
                _ => ()
            }
        });
        if done { return; }

        input::handle_pressed(&currently_pressed, delta_time, MOVE_SENSITIVITY,
                              ROTATE_SENSITIVITY, &mut cam);
    }
}
