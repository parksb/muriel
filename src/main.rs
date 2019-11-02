mod app;
mod flag;
mod lib;
mod util;

fn main() {
    let matches = app::build();
    app::run(matches);
}
