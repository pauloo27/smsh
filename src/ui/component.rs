use gtk::prelude::*;
use gtk4 as gtk;

use crate::schema::{Align, Component, ContainerOrientation, ToggleButtonData};

fn build_toggle_button(data: &ToggleButtonData) -> gtk::ToggleButton {
    let toggle_btn = gtk::ToggleButton::builder().active(data.active).build();

    match (&data.icon, &data.text) {
        (Some(icon_name), Some(label)) => {
            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            let image = gtk::Image::from_icon_name(icon_name);
            let lbl = gtk::Label::new(Some(label));
            hbox.append(&image);
            hbox.append(&lbl);
            toggle_btn.set_child(Some(&hbox));
        }
        (Some(icon_name), None) => {
            let image = gtk::Image::from_icon_name(icon_name);
            toggle_btn.set_child(Some(&image));
        }
        (None, Some(label)) => {
            toggle_btn.set_label(label);
        }
        (None, None) => {}
    }

    if let Some(ref tip) = data.tooltip {
        toggle_btn.set_tooltip_text(Some(tip.as_str()));
    }

    if let Some(ref class_list) = data.classes {
        for class in class_list {
            toggle_btn.add_css_class(class.as_str());
        }
    }

    toggle_btn
}

pub(super) fn build_component(component: Component) -> gtk::Widget {
    match component {
        Component::Label {
            text,
            tooltip,
            classes,
        } => {
            let mut builder = gtk::Label::builder().label(text);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let lbl = builder.build();

            if let Some(class_list) = classes {
                for class in class_list {
                    lbl.add_css_class(&class);
                }
            }

            lbl.upcast()
        }
        Component::Button {
            text,
            icon,
            tooltip,
            classes,
            action,
        } => {
            let btn = gtk::Button::new();

            match (&icon, &text) {
                (Some(icon_name), Some(label)) => {
                    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let image = gtk::Image::from_icon_name(icon_name);
                    let lbl = gtk::Label::new(Some(label));
                    hbox.append(&image);
                    hbox.append(&lbl);
                    btn.set_child(Some(&hbox));
                }
                (Some(icon_name), None) => {
                    btn.set_icon_name(icon_name);
                }
                (None, Some(label)) => {
                    btn.set_label(label);
                }
                (None, None) => {}
            }

            if let Some(tip) = tooltip {
                btn.set_tooltip_text(Some(&tip));
            }

            if let Some(class_list) = classes {
                for class in class_list {
                    btn.add_css_class(&class);
                }
            }

            if let Some(action) = action {
                btn.connect_clicked(move |_| {
                    let _ = action.callback.call::<()>("".to_string());
                });
            }

            btn.upcast()
        }
        Component::Entry {
            text,
            tooltip,
            classes,
            action,
        } => {
            let mut builder = gtk::Entry::builder().text(text);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let entry = builder.build();

            if let Some(class_list) = classes {
                for class in class_list {
                    entry.add_css_class(&class);
                }
            }

            if let Some(action) = action {
                entry.connect_activate(move |entry| {
                    let _ = action.callback.call::<()>(entry.text().to_string());
                });
            }

            entry.upcast()
        }
        Component::ToggleButton(data) => {
            let toggle_btn = build_toggle_button(&data);

            if let Some(action) = data.action {
                toggle_btn.connect_toggled(move |btn| {
                    let state = if btn.is_active() { "true" } else { "false" };
                    let _ = action.callback.call::<()>(state.to_string());
                });
            }

            toggle_btn.upcast()
        }
        Component::ToggleButtonGroup { container: container_data, buttons } => {
            let gtk_orientation = match container_data.orientation {
                ContainerOrientation::Vertical => gtk::Orientation::Vertical,
                ContainerOrientation::Horizontal => gtk::Orientation::Horizontal,
            };

            let mut builder = gtk::Box::builder().orientation(gtk_orientation);

            if let Some(align) = container_data.halign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.halign(gtk_align);
            }

            if let Some(align) = container_data.valign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.valign(gtk_align);
            }

            let container = builder.build();

            if let Some(class_list) = container_data.classes {
                for class in class_list {
                    container.add_css_class(&class);
                }
            }

            let mut group_leader: Option<gtk::ToggleButton> = None;

            for data in buttons {
                let toggle_btn = build_toggle_button(&data);

                // Set up grouping - all buttons join the first button's group
                if let Some(ref leader) = group_leader {
                    toggle_btn.set_group(Some(leader));
                } else {
                    group_leader = Some(toggle_btn.clone());
                }

                if let Some(action) = data.action {
                    toggle_btn.connect_toggled(move |btn| {
                        let state = if btn.is_active() { "true" } else { "false" };
                        let _ = action.callback.call::<()>(state.to_string());
                    });
                }

                container.append(&toggle_btn);
            }

            container.upcast()
        }
        Component::Container {
            container: container_data,
            children,
        } => {
            let gtk_orientation = match container_data.orientation {
                ContainerOrientation::Vertical => gtk::Orientation::Vertical,
                ContainerOrientation::Horizontal => gtk::Orientation::Horizontal,
            };

            let mut builder = gtk::Box::builder().orientation(gtk_orientation);

            if let Some(align) = container_data.halign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.halign(gtk_align);
            }

            if let Some(align) = container_data.valign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.valign(gtk_align);
            }

            let container = builder.build();

            if let Some(class_list) = container_data.classes {
                for class in class_list {
                    container.add_css_class(&class);
                }
            }

            for child in children {
                let widget = build_component(child);
                container.append(&widget);
            }

            container.upcast()
        }
    }
}
