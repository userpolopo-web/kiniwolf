mod assets;
mod browser;
mod settings;

use browser::{parse_browser_command, BrowserCommand};
use serde_json::{json, Value};
use settings::Settings;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tao::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder},
};
use wry::{Rect, WebContext, WebView, WebViewBuilder};
const HOME: &str = "https://duckduckgo.com";

#[derive(Debug)]
enum Message {
    Command(Value),
    Url(u64, String),
    Title(u64, String),
    Focus,
}
struct Tab {
    id: u64,
    url: String,
    title: String,
    view: Option<WebView>,
    popup_source: Arc<AtomicBool>,
}
struct Browser {
    tabs: Vec<Tab>,
    active: u64,
    next: u64,
    settings: Settings,
    context: WebContext,
    panel: bool,
}

impl Browser {
    fn height(&self) -> u32 {
        if self.panel {
            250
        } else {
            94
        }
    }
    fn activate(
        &mut self,
        id: u64,
        window: &Window,
        proxy: &EventLoopProxy<Message>,
    ) -> wry::Result<()> {
        if !self.tabs.iter().any(|t| t.id == id) {
            return Ok(());
        }
        self.active = id;
        let height = self.height();
        for tab in &mut self.tabs {
            if tab.id != id {
                if self.settings.memory_saver && !tab.popup_source.load(Ordering::Relaxed) {
                    tab.view = None;
                } else if let Some(view) = &tab.view {
                    view.set_visible(false)?;
                }
            }
        }
        let tab = self.tabs.iter_mut().find(|t| t.id == id).unwrap();
        if tab.view.is_none() {
            let nav = proxy.clone();
            let title = proxy.clone();
            let ipc = proxy.clone();
            let popup_source = tab.popup_source.clone();
            let view = WebViewBuilder::with_web_context(&mut self.context)
                .with_url(&tab.url)
                .with_bounds(bounds(window, height, false))
                .with_initialization_script(include_str!("../assets/content.js"))
                .with_ipc_handler(move |request| {
                    let event = match request.body().as_str() {
                        "focus-address" => Some(Message::Focus),
                        "new-tab" => Some(Message::Command(json!({"type":"new-tab"}))),
                        "close-tab" => Some(Message::Command(json!({"type":"close-tab","id":id}))),
                        _ => None,
                    };
                    if let Some(event) = event {
                        let _ = ipc.send_event(event);
                    }
                })
                .with_navigation_handler(move |url| {
                    let _ = nav.send_event(Message::Url(id, url));
                    true
                })
                .with_document_title_changed_handler(move |text| {
                    let _ = title.send_event(Message::Title(id, text));
                })
                .with_new_window_req_handler(move |_| {
                    // A native popup preserves window.opener, postMessage and delayed
                    // navigation from about:blank. Reopening its URL loses that context.
                    popup_source.store(true, Ordering::Relaxed);
                    true
                })
                .build_as_child(window)?;
            configure_passwords(&view, self.settings.save_passwords)?;
            tab.view = Some(view);
        }
        let view = tab.view.as_ref().unwrap();
        view.set_visible(true)?;
        view.set_bounds(bounds(window, height, false))?;
        view.focus()
    }
    fn add(
        &mut self,
        url: String,
        window: &Window,
        proxy: &EventLoopProxy<Message>,
    ) -> wry::Result<()> {
        let id = self.next;
        self.next += 1;
        self.tabs.push(Tab {
            id,
            url,
            title: "Nova aba".into(),
            view: None,
            popup_source: Arc::new(AtomicBool::new(false)),
        });
        self.activate(id, window, proxy)
    }
    fn render(&self, toolbar: &WebView) -> wry::Result<()> {
        let tabs: Vec<_> = self
            .tabs
            .iter()
            .map(|t| json!({"id":t.id,"url":t.url,"title":t.title,"suspended":t.view.is_none()}))
            .collect();
        toolbar.evaluate_script(&format!(
            "window.renderBrowser({});",
            json!({"tabs":tabs,"active":self.active,"settings":self.settings,"panel":self.panel})
        ))
    }
    fn command(
        &mut self,
        value: Value,
        window: &Window,
        proxy: &EventLoopProxy<Message>,
        toolbar: &WebView,
    ) -> wry::Result<()> {
        match value["type"].as_str().unwrap_or("") {
            "ready" => {}
            "new-tab" => self.add(
                value["url"]
                    .as_str()
                    .and_then(browser::normalize_navigation_input)
                    .unwrap_or_else(|| HOME.into()),
                window,
                proxy,
            )?,
            "select-tab" => {
                if let Some(id) = value["id"].as_u64() {
                    self.activate(id, window, proxy)?;
                }
            }
            "close-tab" => {
                if let Some(index) = self
                    .tabs
                    .iter()
                    .position(|t| Some(t.id) == value["id"].as_u64())
                {
                    let removed = self.tabs.remove(index);
                    if self.tabs.is_empty() {
                        self.add(HOME.into(), window, proxy)?;
                    } else if removed.id == self.active {
                        let id = self.tabs[index.min(self.tabs.len() - 1)].id;
                        self.activate(id, window, proxy)?;
                    }
                }
            }
            "panel" => {
                self.panel = !self.panel;
                toolbar.set_bounds(bounds(window, self.height(), true))?;
                self.activate(self.active, window, proxy)?;
            }
            "settings" => {
                let mut settings = self.settings.clone();
                if let Some(enabled) = value["memory_saver"].as_bool() {
                    settings.memory_saver = enabled;
                }
                if let Some(enabled) = value["save_passwords"].as_bool() {
                    settings.save_passwords = enabled;
                }
                for tab in &self.tabs {
                    if let Some(view) = &tab.view {
                        configure_passwords(view, settings.save_passwords)?;
                    }
                }
                if let Err(error) = settings.save() {
                    eprintln!("Settings: {error}");
                    toolbar.evaluate_script(
                        "window.showError('Nao foi possivel salvar as configuracoes.')",
                    )?;
                } else {
                    self.settings = settings;
                    self.activate(self.active, window, proxy)?;
                }
            }
            _ => {
                if let Some(command) = parse_browser_command(&value.to_string()) {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == self.active) {
                        if let Some(view) = &tab.view {
                            match command {
                                BrowserCommand::Navigate(url) => {
                                    view.load_url(&url)?;
                                    tab.url = url;
                                }
                                BrowserCommand::Home => {
                                    view.load_url(HOME)?;
                                    tab.url = HOME.into();
                                }
                                BrowserCommand::Back => view.evaluate_script("history.back()")?,
                                BrowserCommand::Forward => {
                                    view.evaluate_script("history.forward()")?
                                }
                                BrowserCommand::Reload => {
                                    view.evaluate_script("location.reload()")?
                                }
                            }
                        }
                    }
                }
            }
        }
        self.render(toolbar)
    }
}

#[cfg(target_os = "windows")]
fn configure_passwords(view: &WebView, enabled: bool) -> wry::Result<()> {
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings4;
    use windows_core::Interface;
    use wry::WebViewExtWindows;
    // Keep credential encryption, consent prompts and origin checks inside WebView2.
    unsafe {
        let settings: ICoreWebView2Settings4 =
            view.controller().CoreWebView2()?.Settings()?.cast()?;
        settings.SetIsPasswordAutosaveEnabled(enabled)?;
        settings.SetIsGeneralAutofillEnabled(enabled)?;
    }
    Ok(())
}
#[cfg(not(target_os = "windows"))]
fn configure_passwords(_: &WebView, _: bool) -> wry::Result<()> {
    Ok(())
}

fn bounds(window: &Window, height: u32, toolbar: bool) -> Rect {
    let size = window.inner_size();
    let height = (height as f64 * window.scale_factor()).round() as u32;
    Rect {
        position: PhysicalPosition::new(0, if toolbar { 0 } else { height }).into(),
        size: PhysicalSize::new(
            size.width,
            if toolbar {
                height
            } else {
                size.height.saturating_sub(height)
            },
        )
        .into(),
    }
}
fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<Message>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Kiniwolf Browser")
        .with_inner_size(LogicalSize::new(1180.0, 760.0))
        .with_min_inner_size(LogicalSize::new(440.0, 400.0))
        .build(&event_loop)
        .expect("create window");
    let toolbar_proxy = proxy.clone();
    let toolbar = WebViewBuilder::new()
        .with_html(assets::browser_shell_html(HOME))
        .with_bounds(bounds(&window, 94, true))
        .with_ipc_handler(move |request| {
            if let Ok(value) = serde_json::from_str(request.body()) {
                let _ = toolbar_proxy.send_event(Message::Command(value));
            }
        })
        .build_as_child(&window)?;
    std::fs::create_dir_all(settings::data_dir()).expect("create browser profile");
    let mut browser = Browser {
        tabs: vec![],
        active: 0,
        next: 1,
        settings: Settings::load(),
        context: WebContext::new(Some(settings::data_dir().join("WebView2"))),
        panel: false,
    };
    browser.add(HOME.into(), &window, &proxy)?;
    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        let result = match event {
            Event::WindowEvent {event:WindowEvent::CloseRequested,..} => { *flow = ControlFlow::Exit; Ok(()) }
            Event::WindowEvent {event:WindowEvent::Resized(_),..} | Event::WindowEvent {event:WindowEvent::ScaleFactorChanged {..},..} => {
                let _ = toolbar.set_bounds(bounds(&window,browser.height(),true));
                for tab in &browser.tabs { if let Some(view) = &tab.view { let _ = view.set_bounds(bounds(&window,browser.height(),false)); } } Ok(())
            }
            Event::UserEvent(Message::Command(value)) => browser.command(value,&window,&proxy,&toolbar),
            Event::UserEvent(Message::Url(id,url)) => { if let Some(t) = browser.tabs.iter_mut().find(|t| t.id==id && t.view.is_some()) {t.url=url;} browser.render(&toolbar) }
            Event::UserEvent(Message::Title(id,title)) => { if let Some(t) = browser.tabs.iter_mut().find(|t| t.id==id && t.view.is_some()) {t.title=title;} browser.render(&toolbar) }
            Event::UserEvent(Message::Focus) => { let _ = toolbar.focus(); toolbar.evaluate_script("document.getElementById('address').focus(); document.getElementById('address').select();") }
            _ => Ok(()),
        };
        if let Err(error) = result { eprintln!("Browser: {error}"); let _ = toolbar.evaluate_script("window.showError('Nao foi possivel concluir esta acao.')"); }
    });
}
