pub mod file {
    use std::{
        error::Error,
        fs::{self, File, OpenOptions},
        io::Write,
        path::PathBuf,
        str::FromStr,
    };

    use crate::models::alias::Alias;

    /// Parses aliases from a tmp file
    pub fn get_aliases(path: PathBuf) -> Result<Vec<Alias>, Box<dyn Error>> {
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
    pub fn make_alias(path: PathBuf, alias: Alias) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Cannot open alias file");

        file.write_all(alias.to_string().as_bytes())
            .expect("Could not write alias to aliases file");

        Ok(())
    }
}
