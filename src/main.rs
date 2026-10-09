mod assets;
mod browser;

use assets::browser_shell_html;
use browser::{parse_browser_command, BrowserCommand};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{http::Request, WebViewBuilder};

const HOME_URL: &str = "https://duckduckgo.com";

#[derive(Debug)]
enum UserEvent {
    BrowserCommand(BrowserCommand),
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Kiniwolf Browser")
        .with_inner_size(LogicalSize::new(1180.0, 760.0))
        .build(&event_loop)
        .expect("failed to create window");

    let handler = move |request: Request<String>| {
        if let Some(command) = parse_browser_command(request.body()) {
            let _ = proxy.send_event(UserEvent::BrowserCommand(command));
        }
    };

    let webview = WebViewBuilder::new()
        .with_html(browser_shell_html(HOME_URL))
        .with_ipc_handler(handler)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(UserEvent::BrowserCommand(command)) => {
                let script = script_for_command(command);
                if let Err(error) = webview.evaluate_script(&script) {
                    eprintln!("failed to run browser command: {error}");
                }
            }
            _ => {}
        }
    });
}

fn script_for_command(command: BrowserCommand) -> String {
    match command {
        BrowserCommand::Navigate(url) => {
            let url = serde_json::to_string(&url).expect("URL string should serialize");
            format!("window.kiniwolfNavigate({url});")
        }
        BrowserCommand::Back => "window.kiniwolfBack();".to_string(),
        BrowserCommand::Forward => "window.kiniwolfForward();".to_string(),
        BrowserCommand::Reload => "window.kiniwolfReload();".to_string(),
        BrowserCommand::Home => "window.kiniwolfHome();".to_string(),
    }
}
