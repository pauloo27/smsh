use super::parser;
use crate::def::action::{Action, ActionType};
use crate::def::component::ComponentType;
use crate::def::container::{Container, ContainerOrientation};
use gtk::gio::{self};
use gtk::prelude::*;
use gtk4 as gtk;
use std::process;
use std::rc::Rc;

mod css;
mod keybinds;
mod options;
pub use options::*;

const APP_ID: &str = "cafe.ndo.SMSH";

pub fn run(options: AppOptions) {
    let app = gtk::Application::builder()
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .application_id(APP_ID)
        .build();
    let options_rc = Rc::new(options);

    app.connect_activate(move |app| {
        setup_ui(app, options_rc.clone());
    });

    let args: [String; 0] = [];
    app.run_with_args(&args);
}

fn setup_ui(app: &gtk::Application, options: Rc<AppOptions>) {
    let def_file = &options.definition_file;
    let css_file = &options.css_file;

    let win_def = match parser::load_window_from_file(def_file.into()) {
        Ok(win_def) => win_def,
        Err(err) => {
            eprintln!("Failed to parse file {err}");
            process::exit(1);
        }
    };
    if let Some(css_file) = css_file {
        css::load_css_from_file(css_file.into());
    }

    let container = build_container(win_def.container);

    let mut window_builder = gtk::ApplicationWindow::builder()
        .application(app)
        .title(win_def.title)
        .child(&container);

    if let Some(width) = win_def.width {
        window_builder = window_builder.default_width(width);
    }
    if let Some(height) = win_def.height {
        window_builder = window_builder.default_height(height);
    }

    let window = window_builder.build();

    if win_def.enable_vim_keys.unwrap_or(false) {
        keybinds::add_vim_keyboard_actions(app, &window);
    }
    if win_def.enable_esc_as_exit.unwrap_or(false) {
        keybinds::add_esc_keyboard_action(app, &window);
    }

    window.present();
}

fn build_container(container_def: Container) -> gtk::Box {
    let orientation = match container_def.orientation {
        ContainerOrientation::Vertical => gtk::Orientation::Vertical,
        ContainerOrientation::Horizontal => gtk::Orientation::Horizontal,
    };

    let container = gtk::Box::builder().orientation(orientation).build();

    for component in container_def.children {
        match component.r#type {
            ComponentType::Label => {
                let lbl = gtk::Label::builder()
                    .tooltip_text(component.tooltip)
                    .label(component.text)
                    .build();

                container.append(&lbl)
            }
            ComponentType::Entry => {
                let entry = gtk::Entry::builder()
                    .tooltip_text(component.tooltip)
                    .text(component.text)
                    .build();

                if let Some(actions) = component.actions {
                    entry.connect_activate(move |entry| {
                        call_actions(&actions, entry.text().to_string());
                    });
                }

                container.append(&entry)
            }
            ComponentType::Button => {
                let btn = gtk::Button::builder()
                    .tooltip_text(component.tooltip)
                    .label(component.text)
                    .build();

                if let Some(actions) = component.actions {
                    btn.connect_clicked(move |_| {
                        call_actions(&actions, "".to_string());
                    });
                }

                container.append(&btn)
            }
        }
    }

    container
}

fn call_actions(actions: &Vec<Action>, value: String) {
    for action in actions {
        match action.r#type {
            ActionType::PrintValueToStdOut => {
                println!("{value}");
            }
            ActionType::ExitWithCode => {
                let code: i32 = action.value.parse().expect("Invalid status code");
                process::exit(code);
            }
            ActionType::Shell => {
                let _ = process::Command::new("env")
                    .arg("sh")
                    .arg("-c")
                    .arg(action.value.as_str())
                    .spawn();
            }
        }
    }
}
