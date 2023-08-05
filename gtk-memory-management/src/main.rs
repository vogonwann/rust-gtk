use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_GObjectMemoryManagement0";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    // let mut number = 0; - DOES NOT WORK

    // Reference-counted object with inner mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease => 
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }));
    button_decrease.connect_clicked(clone!(@weak button_increase => 
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }));

    // Add button to `gtk box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    
    // Create a window
    let window = ApplicationWindow::builder()
       .application(app)
       .title("GObject MM")
       .child(&gtk_box)
       .build();

    // Present the window
    window.present();
}
