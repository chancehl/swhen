pub mod file {
    use std::{
        error::Error,
        fs::{self, File, OpenOptions},
        io::Write,
        path::PathBuf,
        str::FromStr,
    };

    use crate::models::alias::Alias;

    /// Gets a specific alias from tmp file
    pub fn get_alias(name: &str, path: &PathBuf) -> Result<Option<Alias>, Box<dyn Error>> {
        let aliases = get_aliases(path)?;

        for alias in aliases {
            if alias.name == name {
                return Ok(Some(alias));
            }
        }

        return Ok(None);
    }

    /// Parses aliases from a tmp file
    pub fn get_aliases(path: &PathBuf) -> Result<Vec<Alias>, Box<dyn Error>> {
        let aliases = match fs::read_to_string(&path) {
            Ok(contents) => contents
                .split("\n")
                .filter(|line| line.len() > 0)
                .map(|line| Alias::from_str(line).expect("Could not convert alias to struct"))
                .collect::<Vec<Alias>>(),
            Err(_) => {
                println!("tmpfile does not exist... creating one now at {:?}", &path);

                File::create(path).expect("Could not create tmpfile");

                Vec::new()
            }
        };

        Ok(aliases)
    }

    /// Adds a new alias to the aliases file
    pub fn save_alias(alias: Alias, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .expect("Cannot open alias file");

        let new_entry = format!("\n{}", alias.to_string());

        file.write_all(new_entry.as_bytes())
            .expect("Could not write alias to aliases file");

        Ok(())
    }
}
