use anyhow::anyhow;

mod app;
mod component;
mod data;
mod dirs;
mod event;
mod handlers;
mod tabs;
mod tui;
mod utils;
mod view;

fn main() {
    let res = match tui::TuiApp::build() {
        Ok(app) => app.run(),
        Err(e) => Err(anyhow!(e)),
    };
    println!("res: {:?}", res);
}
