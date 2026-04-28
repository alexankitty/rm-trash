mod encoding;
mod templates;

use crate::prettyprint::printer::PrettyPrinter;

use super::prettyprint;
use super::prettyprint::constants::*;
use std::env;
use std::path::Path;
use std::time;
use string_template_plus::*;

pub struct FileHandler {
    trash_dir: String,
    pretty_printer: PrettyPrinter,
}

impl FileHandler {
    pub fn new() -> Self {
        let data_home = &env::var("XDG_DATA_HOME").unwrap_or_else(|_| "~/.local/share".to_string());
        let trash_dir = Path::new(data_home)
            .join("Trash/files")
            .to_string_lossy()
            .into_owned();
        let pretty_printer = PrettyPrinter::new();
        Self {
            trash_dir,
            pretty_printer,
        }
    }

    pub fn trash_file(&self, path: &str) -> std::io::Result<()> {
        let filepath = Path::new(path);

        let dest = Path::new(&self.trash_dir).join(filepath.file_name().unwrap());
        std::fs::rename(path, dest)?;
        self.pretty_printer
            .print_action(Some(VERB_TRASHED), Some(path), None, None);
        Ok(())
    }

    pub fn update_trash_meta(&self) {
        //todo: implement logic
    }

    pub fn get_trash_dir(&self) -> &str {
        // todo: implement logic
        &self.trash_dir
    }

    pub fn get_mtime(&self) {
        //todo: implement getting time
    }
}
