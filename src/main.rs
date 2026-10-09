mod assets;
mod browser;

use assets::browser_shell_html;
use browser::{parse_browser_command, BrowserCommand};
use tao::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{http::Request, Rect, WebView, WebViewBuilder};

const HOME_URL: &str = "https://duckduckgo.com";
const TOOLBAR_HEIGHT: u32 = 52;

#[derive(Debug)]
enum UserEvent {
    BrowserCommand(BrowserCommand),
    FocusAddress,
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

    let size = window.inner_size();
    let toolbar = WebViewBuilder::new()
        .with_html(browser_shell_html(HOME_URL))
        .with_ipc_handler(handler)
        .with_bounds(toolbar_bounds(size))
        .build_as_child(&window)?;

    let content_proxy = event_loop.create_proxy();
    let content_handler = move |request: Request<String>| {
        if request.body() == "focus-address" {
            let _ = content_proxy.send_event(UserEvent::FocusAddress);
        }
    };

    let content = WebViewBuilder::new()
        .with_url(HOME_URL)
        .with_initialization_script(CONTENT_SHORTCUTS)
        .with_ipc_handler(content_handler)
        .with_bounds(content_bounds(size))
        .build_as_child(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    let _ = toolbar.set_bounds(toolbar_bounds(size));
                    let _ = content.set_bounds(content_bounds(size));
                }
                _ => {}
            },
            Event::UserEvent(UserEvent::BrowserCommand(command)) => {
                if let Err(error) = handle_browser_command(command, &toolbar, &content) {
                    eprintln!("failed to run browser command: {error}");
                }
            }
            Event::UserEvent(UserEvent::FocusAddress) => {
                let _ = toolbar.evaluate_script("document.getElementById('address').focus(); document.getElementById('address').select();");
                let _ = toolbar.focus();
            }
            _ => {}
        }
    });
}

fn handle_browser_command(command: BrowserCommand, toolbar: &WebView, content: &WebView) -> wry::Result<()> {
    match command {
        BrowserCommand::Navigate(url) => {
            let url = serde_json::to_string(&url).expect("URL string should serialize");
            toolbar.evaluate_script(&format!("window.kiniwolfNavigate({url});"))?;
            content.load_url(serde_json::from_str::<String>(&url).expect("URL should deserialize").as_str())
        }
        BrowserCommand::Back => content.evaluate_script("history.back();"),
        BrowserCommand::Forward => content.evaluate_script("history.forward();"),
        BrowserCommand::Reload => content.evaluate_script("location.reload();"),
        BrowserCommand::Home => {
            toolbar.evaluate_script(&format!("window.kiniwolfNavigate({});", serde_json::to_string(HOME_URL).unwrap()))?;
            content.load_url(HOME_URL)
        }
    }
}

fn toolbar_bounds(size: PhysicalSize<u32>) -> Rect {
    Rect {
        position: LogicalPosition::new(0, 0).into(),
        size: LogicalSize::new(size.width, TOOLBAR_HEIGHT).into(),
    }
}

fn content_bounds(size: PhysicalSize<u32>) -> Rect {
    Rect {
        position: LogicalPosition::new(0, TOOLBAR_HEIGHT).into(),
        size: LogicalSize::new(size.width, size.height.saturating_sub(TOOLBAR_HEIGHT)).into(),
    }
}

const CONTENT_SHORTCUTS: &str = r#"
window.addEventListener("keydown", (event) => {
  if (event.altKey && event.key === "ArrowLeft") {
    event.preventDefault();
    history.back();
  }
  if (event.altKey && event.key === "ArrowRight") {
    event.preventDefault();
    history.forward();
  }
  if (event.ctrlKey && event.key.toLowerCase() === "r") {
    event.preventDefault();
    location.reload();
  }
  if (event.ctrlKey && event.key.toLowerCase() === "l") {
    event.preventDefault();
    window.ipc.postMessage("focus-address");
  }
});
"#;
