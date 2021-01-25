use forge_engine::vulkan::{Engine, EngineCreateInfo};
use forge_engine::vulkan::renderer;
use forge_engine::vulkan::renderer::instance_manager::{InstanceManager, UBO};
use std::ffi::CString;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::platform::unix::WindowExtUnix;
use winit::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use camera::Camera;
use std::time::{Duration, Instant};
use forge_engine::geometry::model::Model;
use cgmath::{Point3, Vector3, Vector4, Matrix4, SquareMatrix, Deg};
fn main() {

    let event_loop = EventLoop::new();

    let window = create_window(&event_loop, "deferred shading", 1920, 1080);
    let window_handle = window.xlib_window().unwrap();
    let display_handle = window.xlib_display().unwrap();

    let mut engine = build_engine();

    let renderer_handle = unsafe{engine.create_renderer(display_handle, window_handle)};

    let model = Model::from_obj(std::path::Path::new("assets/models/bunny.obj"));

    let mesh_id = engine.geometry_manager_.load_mesh(model.meshes_.first().unwrap());

    engine.set_forward_rendering(renderer_handle, "src/shaders/phong.vert", "src/shaders/phong.frag");

    let instance_id = engine.renderers_[renderer_handle as usize].create_instance(mesh_id);

    let instances = vec![instance_id];

    let light_id1 = engine.renderers_[renderer_handle as usize].scene_manager_.create_light();

    let light1 = engine.renderers_[renderer_handle as usize].scene_manager_.get_mut_light(light_id1);

    light1.position_ = Vector4{
        x: 10.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    light1.color_ = Vector4{
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    let light_id2 = engine.renderers_[renderer_handle as usize].scene_manager_.create_light();

    let light2 = engine.renderers_[renderer_handle as usize].scene_manager_.get_mut_light(light_id2);

    light2.position_ = Vector4{
        x: 10.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    light2.color_ = Vector4{
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    };


    let mut camera = Camera::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        },
        -90.0,
        0.0,
        1000.0
    );

    main_loop(event_loop, window, engine, camera, instances);

}

fn build_engine() -> Engine
{
    let validation_layers = CString::new("VK_LAYER_KHRONOS_validation").expect("Could not create CString");
    let validation_layers_ptr = validation_layers.as_ptr();
    let device_extension_names = CString::new("VK_KHR_swapchain").expect("Could not create CString");
    let device_extension_names_ptr = device_extension_names.as_ptr();
    let create_info = EngineCreateInfo{app_name_: CString::new("Hello").expect("Could not create app name"),
        app_version_: 1,
        num_validation_layers_: 1,
        validation_layers_: &validation_layers_ptr,
        num_device_extension_names_: 1,
        device_extension_names_: &device_extension_names_ptr,
    };

    Engine::new(&create_info)
}

fn create_window(event_loop: &EventLoop<()>, header: &str, width: u32, height: u32) -> winit::window::Window
{
    winit::window::WindowBuilder::new()
        .with_title(header)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop)
        .expect("Could not create window")
}

fn main_loop(event_loop: EventLoop<()>, window: winit::window::Window, engine: Engine, camera: Camera, instances: Vec<u64>)
{
    let mut start = Instant::now();


    let mut engine = engine;
    let mut camera = camera;

    event_loop.run(move |event, _, control_flow :& mut ControlFlow|{

        let dt = start.elapsed().as_secs_f32();
        start = Instant::now();

        match event{
            Event::WindowEvent{ event, ..} => {
                match event{
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::KeyboardInput {input, ..} =>{
                        match input{
                            KeyboardInput{ virtual_keycode, state, ..} => {
                                match (virtual_keycode, state) {
                                    (Some(VirtualKeyCode::W), ElementState::Pressed) => {
                                        camera.move_forward(dt);
                                    },
                                    (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                                        camera.move_left(dt);
                                    },
                                    (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                                        camera.move_backward(dt);
                                    },
                                    (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                                        camera.move_right(dt);
                                    },
                                    _ => {},
                                }
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Event::MainEventsCleared => {
              window.request_redraw();
            },
            Event::RedrawRequested(_window_id) => {
                engine.update();
            }
            _ => {}
        }

        camera.update();

        let ubo = UBO{
            model_: Matrix4::identity(),
            view_: camera.view(),
            projection_: {
                let mut proj = cgmath::perspective(Deg(75.0), 1920.0 / 1080.0, 0.1, 1000.0);
                proj[1][1] = proj[1][1] * -1.0;
                proj
            },
        };

        for instance in instances.iter()
        {
            engine.renderers_[0].instance_manager_.update_instance(instance, ubo);
        }

    });
}

