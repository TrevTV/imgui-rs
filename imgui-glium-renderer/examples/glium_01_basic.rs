use glium::glutin::surface::WindowSurface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use glium::{Surface, Display};
use winit::platform::windows::WindowExtWindows;
use winit::window::Window;

const TITLE: &str = "Hello, imgui-rs!";

fn main() {
    // Common setup for creating a winit window and imgui context, not specifc
    // to this renderer at all except that glutin is used to create the window
    // since it will give us access to a GL context
    let (event_loop, display, window) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    // Create renderer from this crate
    let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui_context, &display)
        .expect("Failed to initialize renderer");

    // Timer for FPS calculation
    let mut last_frame = std::time::Instant::now();

    // Standard winit event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            println!("{:?}", now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            winit_platform
                .prepare_frame(imgui_context.io_mut(), &window)
                .expect("Failed to prepare frame");
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            // Create frame for the all important `&imgui::Ui`
            let ui = imgui_context.frame();

            // Draw our example content
            ui.show_demo_window(&mut true);

            // Setup for drawing
            let mut target = display.draw();

            // Renderer doesn't automatically clear window
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);

            // Perform rendering
            winit_platform.prepare_render(ui, &window);
            let draw_data = imgui_context.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        event => {
            winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        }
    });
}

fn create_window() -> (EventLoop<()>, Display<WindowSurface>, Window) {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Tack")
            .build(&event_loop);

    (event_loop, display, window)
}

fn imgui_init(window: &Window) -> (imgui_winit_support::WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);

    let dpi_mode = imgui_winit_support::HiDpiMode::Default;

    winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    (winit_platform, imgui_context)
}
