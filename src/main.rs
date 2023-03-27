use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::MenuBar,
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Sets the URL to view
    #[clap(short, long)]
    url: String,

    /// Sets the window to always be on top
    #[clap(long)]
    always_on_top: bool,
}

fn main() -> wry::Result<()> {
    let cli: Cli = Cli::parse();

    let event_loop = EventLoop::new();

    let monitor = event_loop.primary_monitor().unwrap();
    let monitor_scale_factor = monitor.scale_factor();
    let monitor_size = monitor.size().to_logical::<f64>(monitor_scale_factor);
    let window_size = wry::application::dpi::LogicalSize::new(800.0, 600.0); // use f64 values
    let window_position = wry::application::dpi::LogicalPosition::new(
        monitor_size.width - window_size.width,
        0.0, // use f64 value
    );

    let mut menu = MenuBar::new();
    let mut file_menu = MenuBar::new();
    file_menu.add_native_item(tao::menu::MenuItem::Quit);

    menu.add_submenu("File", true, file_menu);

    let mut window_builder = WindowBuilder::new()
        .with_title("Hello World")
        .with_decorations(false)
        .with_menu(menu)
        .with_inner_size(window_size)
        .with_position(window_position);

    if cli.always_on_top {
        window_builder = window_builder.with_always_on_top(true);
    }

    let window = window_builder.build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?.with_url(&cli.url)?.build()?;

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
