use std::fs::File;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }
}

impl Database {
    pub fn init(&mut self, app_data_dir: PathBuf) -> IoResult<()> {
        self.path.push(app_data_dir);
        self.path.push("scorey.db");

        self.ensure_db_exists()?;

        dbg!(self);

        Ok(())
    }

    fn ensure_db_dir_exists(&self) -> IoResult<()> {
        let dir = &self.path.parent().unwrap();
        if !dir.exists() {
            File::create(dir)?;
        }

        Ok(())
    }

    fn ensure_db_exists(&self) -> IoResult<()> {
        self.ensure_db_dir_exists()?;
        let path = &self.path;

        if !path.exists() {
            File::create(path)?;
        }

        Ok(())
    }
}
