//! Simple interactive application

extern crate elmesque;
extern crate piston;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;
extern crate benzene;
extern crate benzene_2d;

use piston::window::WindowSettings;
use benzene::Driver;
use benzene_2d::Driver2d;
use app::App;

mod app;
mod button;


fn settings() -> WindowSettings {
    WindowSettings::new("I'm a toggle button", (800, 600))
}

fn main() {
    let mut driver2d = Driver2d::new(settings());
    let output = benzene::start(App::new(), driver2d.output());
    driver2d.run(output);
}
