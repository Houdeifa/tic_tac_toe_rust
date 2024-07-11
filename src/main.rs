
use gtk::{prelude::*};
use gtk::{Application,ApplicationWindow};
mod application_ui;
fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();
    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
        .application(app)
        .title("Tic-tac-toe game")
        .default_width(application_ui::WINDOW_WIDTH)
        .default_height(application_ui::WINDOW_HEIGHT)
        .expand(false)
        .resizable(false)
        .build();
        unsafe {
            window.add(&application_ui::MY_UI_INSTANCE.container_box);
        }
        window.show_all();
    });

    application.run();
}
