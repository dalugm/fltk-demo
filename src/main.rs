use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    let mut wind = Window::new(100, 100, 400, 300, "Zechat");
    let mut frame = Frame::new(0, 0, 400, 200, "frame");
    let mut but = Button::new(160, 210, 80, 40, "click");

    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello World"));

    app.run().unwrap();
}
