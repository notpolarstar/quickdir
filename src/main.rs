use std::io;

pub mod app;

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let appl = app::App::default().run(&mut term);
    ratatui::restore();
    appl
}
