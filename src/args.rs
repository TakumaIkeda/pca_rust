use std::env;

pub struct Args {
    pub file_path: String,
}

impl Args {
    pub fn build(mut args: env::Args) -> Result<Self, String> {
        args.next();

        let mut file_path: Option<String> = None;

        for arg in args {
            if file_path.is_none() {
                file_path = Some(arg.clone());
            }
        }

        if file_path.is_none() {
            return Err("No file path provided".to_string());
        }

        Ok(Args {
            file_path: file_path.ok_or("File path not found")?,
        })
    }
}
