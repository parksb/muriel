mod app;
mod lib;
mod util;
mod flags {
    pub mod add;
    pub mod list;
    pub mod remove;
    pub mod update;
}

fn main() {
    let matches = app::build();
    app::run(matches);
}
