use anyhow::anyhow;

mod tui;
use tui::TuiApp;

fn main() {
    let res = match TuiApp::build() {
        Ok(app) => app.run(),
        Err(e) => Err(anyhow!(e)),
    };

    println!("result: {:?}", res);
}
