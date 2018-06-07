fn run() {
    // Basic initialization. See the triangle example if you want more details about this.
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };
    let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
                            .next().expect("no device available");

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    let dimensions = {
        let (width, height) = window.window().get_inner_size().unwrap();
        [width, height]
    };

    let queue = physical.queue_families().find(|&q| {
        q.supports_graphics() && window.is_supported(q).unwrap_or(false)
    }).expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        Device::new(physical, physical.supported_features(), &device_ext,
                    [(queue, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let (mut swapchain, mut images) = {
        let caps = window.capabilities(physical)
                         .expect("failed to get surface capabilities");
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        Swapchain::new(device.clone(), window.clone(), caps.min_image_count, format,
                       dimensions, 1, caps.supported_usage_flags, &queue,
                       SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                       None).expect("failed to create swapchain")
    };


    // Here is the basic initialization for the deferred system.
    let mut frame_system = frame::FrameSystem::new(queue.clone(), swapchain.format());
    let triangle_draw_system = triangle_draw_system::TriangleDrawSystem::new(queue.clone(),
                                                                             frame_system.deferred_subpass());

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

    loop {
        previous_frame_end.cleanup_finished();

        if recreate_swapchain {
            let dimensions = {
                let (new_width, new_height) = window.window().get_inner_size().unwrap();
                [new_width, new_height]
            };

            let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => {
                    continue;
                },
                Err(err) => panic!("{:?}", err)
            };

            mem::replace(&mut swapchain, new_swapchain);
            mem::replace(&mut images, new_images);
            recreate_swapchain = false;
        }

        let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(),
                                                                              None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                recreate_swapchain = true;
                continue;
            },
            Err(err) => panic!("{:?}", err)
        };

        let future = previous_frame_end.join(acquire_future);
        let mut frame = frame_system.frame(future, images[image_num].clone(), Matrix4::identity());
        let mut after_future = None;
        while let Some(pass) = frame.next_pass() {
            match pass {
                frame::Pass::Deferred(mut draw_pass) => {
                    let cb = triangle_draw_system.draw(draw_pass.viewport_dimensions());
                    draw_pass.execute(cb);
                },
                frame::Pass::Lighting(mut lighting) => {
                    lighting.ambient_light([0.1, 0.1, 0.1]);
                    lighting.directional_light(Vector3::new(0.2, -0.1, -0.7), [0.6, 0.6, 0.6]);
                    lighting.point_light(Vector3::new(0.5, -0.5, -0.1), [1.0, 0.0, 0.0]);
                    lighting.point_light(Vector3::new(-0.9, 0.2, -0.15), [0.0, 1.0, 0.0]);
                    lighting.point_light(Vector3::new(0.0, 0.5, -0.05), [0.0, 0.0, 1.0]);
                },
                frame::Pass::Finished(af) => {
                    after_future = Some(af);
                },
            }
        }

        let after_frame = after_future.unwrap()
            .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
            .then_signal_fence_and_flush().unwrap();
        previous_frame_end = Box::new(after_frame) as Box<_>;

        let mut done = false;
        events_loop.poll_events(|ev| {
            match ev {
                winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
                winit::Event::WindowEvent { event: winit::WindowEvent::Resized(_, _), .. } => recreate_swapchain = true,
                _ => ()
            }
        });
        if done { return; }
    }
}