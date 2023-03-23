fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();

    let monitor = event_loop.primary_monitor().unwrap();
    let monitor_scale_factor = monitor.scale_factor();
    let monitor_size = monitor.size().to_logical::<f64>(monitor_scale_factor);
    let window_size = wry::application::dpi::LogicalSize::new(800.0, 600.0); // use f64 values
    let window_position = wry::application::dpi::LogicalPosition::new(
        monitor_size.width - window_size.width,
        0.0, // use f64 value
    );

    let window = WindowBuilder::new()
        .with_title("Hello World")
        .with_decorations(false)
        .with_inner_size(window_size)
        .with_position(window_position)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url("http://localhost:8080")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
