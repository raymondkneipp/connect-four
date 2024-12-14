#[derive(Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub token: char,
}

impl Player {
    pub fn new<S: Into<String>>(name: S, token: char) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("Player must have a name.")
        }

        Self { name, token }
    }
}
