mod encoding;

use crate::mount::*;
use crate::prettyprint::printer::PrettyPrinter;

use super::prettyprint::constants::*;
use chrono::Local;
use std::env;
use std::path::Path;
use std::time;

pub struct FileHandler<'a> {
    trash_dir: String,
    pretty_printer: &'a PrettyPrinter,
    uid: u32,
    disable_root_trash: bool,
}

impl<'a> FileHandler<'a> {
    pub fn new(pretty_printer: &'a PrettyPrinter) -> Self {
        let data_home = &env::var("XDG_DATA_HOME").unwrap_or_else(|_| "~/.local/share".to_string());
        let uid = env::var("UID").unwrap_or_default().parse().unwrap_or(0);
        let disable_root_trash = env::var("DISABLE_ROOT_TRASH").is_ok();
        let trash_dir = Path::new(data_home)
            .join("Trash")
            .to_string_lossy()
            .into_owned();
        Self {
            trash_dir,
            pretty_printer,
            uid: uid,
            disable_root_trash: disable_root_trash,
        }
    }

    pub fn trash_file(&self, path: &str) -> std::io::Result<()> {
        let filepath = Path::new(path);
        let trash_dir = self.get_trash_dir(path);

        let dest = Path::new(&trash_dir).join(filepath.file_name().unwrap());

        std::fs::rename(path, dest)?;
        self.update_trash_metadata(path, &trash_dir)?;
        self.pretty_printer
            .print_action(Some(VERB_TRASHED), Some(path), None, None);
        Ok(())
    }

    pub fn delete_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::remove_file(path)?;
        self.pretty_printer
            .print_action(Some(VERB_REMOVED), Some(path), None, None);
        Ok(())
    }

    pub fn update_trash_metadata(&self, pathstr: &str, trash_dir: &String) -> std::io::Result<()> {
        let path = Path::new(pathstr);
        let encoded_path = encoding::encode_path(path);
        let mut metapath = Path::new(trash_dir).join("info").join(format!(
            "{}.trashinfo",
            path.file_name().unwrap().to_string_lossy()
        ));
        let directorysizespath = Path::new(trash_dir).join("directorysizes");
        let mut counter = 1;
        while std::fs::exists(&metapath).ok().unwrap_or(false) {
            metapath = metapath.with_file_name(format!(
                "{} ({}).trashinfo",
                metapath.file_name().unwrap().to_string_lossy(),
                counter
            ));
            counter += 1;
        }
        let trashinfo = format!(
            "[Trash Info]\nPath={}\nDeletionDate={}",
            path.to_string_lossy(),
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
        );
        std::fs::write(&metapath, trashinfo)?;
        if std::fs::exists(&directorysizespath).ok().unwrap_or(false) {
            let directory_size = self.get_size(pathstr)?;
            let mut directory_sizes = std::fs::read_to_string(&directorysizespath)?;
            let mtime = self.get_mtime(pathstr)?;
            directory_sizes.push_str(&format!(
                "{} {:?} {}\n",
                directory_size, mtime, encoded_path
            ));
            std::fs::write(&directorysizespath, directory_sizes)?;
        }
        Ok(())
    }

    pub fn get_trash_dir(&self, path: &str) -> String {
        let mount_root = find_mount_root(path).unwrap_or("/".to_string());
        let mut trash_dir = self.trash_dir.clone();
        if mount_root != "/" {
            trash_dir = format!("{}/.trash-{}", mount_root, self.uid);
        }
        trash_dir
    }

    pub fn get_mtime(&self, path: &str) -> std::io::Result<time::SystemTime> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.modified()?)
    }

    pub fn get_size(&self, path: &str) -> std::io::Result<u64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len())
    }

    pub fn ensure_trash_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.trash_dir)?;
        Ok(())
    }
}
