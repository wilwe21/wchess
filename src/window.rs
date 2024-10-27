use gtk::prelude::*;
use crate::board::board;

fn on_activate(app: &gtk::Application) {
    let bbbb = board::new();
    let window = gtk::ApplicationWindow::builder()
        .title("wchess")
        .resizable(false)
        .application(app)
        .build();
    window.set_child(Some(&bbbb.board));
    window.show();
}

pub fn load() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wchess")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
