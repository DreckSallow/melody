use anyhow::anyhow;

mod app;
mod component;
mod dirs;
mod event;
mod loaders;
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
