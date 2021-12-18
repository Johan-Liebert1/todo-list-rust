#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);

    canvas.draw(5, 5, "red");
    canvas.draw(15, 15, "orange");
    canvas.draw(20, 20, "blue");

    stdweb::event_loop();
}
