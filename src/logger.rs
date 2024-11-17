use std::{fs::{self, File, OpenOptions}, io::Write, path::PathBuf};

pub struct Logger {
    base_path: PathBuf,
}

impl Logger {
    pub fn new<P: Into<PathBuf>>(base_path: P) -> std::io::Result<Self> {
        let base_path = base_path.into();
        fs::create_dir_all(&base_path)?;
        Ok(Logger { base_path })
    }

    fn get_log_file(&self) -> std::io::Result<File> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let file_name = format!("{}.log", today);
        let file_path = self.base_path.join(file_name);

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
    }

    pub fn log(&self, message: &str) -> std::io::Result<()> {
        let mut file = self.get_log_file()?;
        file.write_all(message.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }
}