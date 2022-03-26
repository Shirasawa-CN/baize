pub mod api;
pub mod keyboard;
use crate::api::read_baize_configuration;

use anyhow::Result;
use gtk4::{prelude::*,Application, ApplicationWindow};

fn main() -> Result<()> {
    let _baize_config = read_baize_configuration();

    let app = Application::builder()
        .application_id("org.Cpointerz.baize")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(700)
            .default_height(500)
            .title("baize")
            .build();

        window.show();
    });

    app.run();

    Ok(())
}
