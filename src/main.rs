mod app;
mod flag;
mod lib;

fn main() {
    let matches = app::build();
    app::run(matches);
}
