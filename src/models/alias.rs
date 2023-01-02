use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Alias {
    pub name: String,
    pub date: String,
}

impl FromStr for Alias {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("=");
        let name = parts.nth(0).expect("Could not locate name from alias");
        let date = parts.nth(0).expect("Could not locate date from alias");

        Ok(Alias {
            name: name.to_string(),
            date: date.to_string(),
        })
    }
}

impl ToString for Alias {
    fn to_string(&self) -> String {
        format!("{:?}={:?}", self.name, self.date)
    }
}
