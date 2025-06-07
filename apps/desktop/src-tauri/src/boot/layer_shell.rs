use std::sync::Mutex;

use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::LayerShell;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

pub struct ShellState {
    pub gtk_window: gtk::ApplicationWindow,
}

unsafe impl Send for ShellState {}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("lux|boot:layer_shell")
        .on_webview_ready(|window| {
            const DEBUG: bool = false;
            let app = window.app_handle();

            let main_window = app
                .get_webview_window("main")
                .expect("failed to get main window");
            // main_window.hide().unwrap();

            let gtk_window = gtk::ApplicationWindow::new(
                &main_window.gtk_window().unwrap().application().unwrap(),
            );

            gtk_window.set_app_paintable(true);

            if !DEBUG {
                let vbox = main_window.default_vbox().unwrap();
                main_window.gtk_window().unwrap().remove(&vbox);
                gtk_window.add(&vbox);
            }

            gtk_window.init_layer_shell();
            gtk_window.set_namespace("lux");
            if !DEBUG {
                gtk_window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive);
            }
            gtk_window.set_layer(gtk_layer_shell::Layer::Top);

            // i'm fucking tired of this shit
            gtk_window.set_size_request(1920, 1080);
            gtk_window.fullscreen();

            let gdk_screen = GtkWindowExt::screen(&gtk_window).unwrap();
            let rgba_visual = gdk_screen.rgba_visual();
            gtk_window.set_visual(rgba_visual.as_ref());

            if !DEBUG {
                gtk_window.show_all();
            }

            app.manage(Mutex::new(ShellState {
                gtk_window: gtk_window.clone(),
            }));
        })
        .build()
}
