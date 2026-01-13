use gtk::prelude::*;
use gtk4::{self as gtk, gio, glib};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;

use crate::schema::{UICommands, Window};

mod component;
mod css;
mod keybinds;

const APP_ID: &str = "cafe.ndo.SMSH";

pub struct UI {
    app: gtk::Application,
    sender: Sender<UICommands>,
}

impl UI {
    pub fn new(css_path: Option<PathBuf>) -> Self {
        let app = gtk::Application::builder()
            .flags(gio::ApplicationFlags::NON_UNIQUE)
            .application_id(APP_ID)
            .build();

        let (sender, receiver) = channel::<UICommands>();
        let rx = Rc::new(receiver);

        let app_clone = app.clone();
        app.connect_activate(move |_| {
            if let Some(ref path) = css_path {
                let ui = UI {
                    app: app_clone.clone(),
                    sender: channel().0,
                };
                if let Err(e) = ui.load_css_from_file(path.clone()) {
                    eprintln!("Failed to load CSS: {}", e);
                }
            }
            setup_window_handler(rx.clone(), app_clone.clone());
        });

        Self { app, sender }
    }

    pub fn get_sender(&self) -> Sender<UICommands> {
        self.sender.clone()
    }

    fn new_window(&self, win_def: Window) {
        let root_widget = component::build_component(win_def.root);

        let mut window_builder = gtk::ApplicationWindow::builder()
            .application(&self.app)
            .title(win_def.title)
            .child(&root_widget);

        if let Some(width) = win_def.width {
            window_builder = window_builder.default_width(width);
        }
        if let Some(height) = win_def.height {
            window_builder = window_builder.default_height(height);
        }

        let window = window_builder.build();

        if win_def.enable_vim_keys.unwrap_or(false) {
            self.add_vim_keyboard_actions(&window);
        }
        if win_def.enable_esc_as_exit.unwrap_or(false) {
            self.add_esc_keyboard_action(&window);
        }

        if win_def.exit_on_close.unwrap_or(false) {
            let exit_code = win_def.exit_code.unwrap_or(1);
            window.connect_close_request(move |_| {
                std::process::exit(exit_code);
            });
        }

        if win_def.present.unwrap_or(false) {
            window.present();
        }
    }

    pub fn run(&self) {
        let args: [String; 0] = [];
        self.app.run_with_args(&args);
    }
}

fn setup_window_handler(rx: Rc<std::sync::mpsc::Receiver<UICommands>>, app: gtk::Application) {
    let hold = app.hold();
    glib::idle_add_local(move || {
        let _hold = &hold;
        match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(command) => {
                let ui = UI {
                    app: app.clone(),
                    sender: channel().0, // Dummy sender, not used here
                };
                match command {
                    UICommands::NewWindow(window) => {
                        ui.new_window(window);
                    }
                    UICommands::LoadCSS(path) => {
                        if let Err(e) = ui.load_css_from_file(path) {
                            eprintln!("Failed to load CSS: {}", e);
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
            Err(err) => {
                eprintln!("Failed to recv command from channel: {:?}", err);
            }
        }
        glib::ControlFlow::Continue
    });
}
