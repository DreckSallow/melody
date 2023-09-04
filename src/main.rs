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
    if let Err(e) = res {
        eprintln!("{}", e);
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
