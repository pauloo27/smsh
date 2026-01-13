use anyhow::Result as AnyResult;
use gtk4 as gtk;
use std::fs;
use std::path::PathBuf;

use super::UI;

impl UI {
    pub(super) fn load_css_from_file(&self, path: PathBuf) -> AnyResult<()> {
        let provider = gtk::CssProvider::new();
        let data = fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("Failed to load CSS file {:?}: {}", path, e))?;

        println!("{data}");

        provider.load_from_data(&data);

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default()
                .ok_or_else(|| anyhow::anyhow!("Could not connect to a display"))?,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        Ok(())
    }
}
