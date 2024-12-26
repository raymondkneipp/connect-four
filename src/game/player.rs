use once_cell::sync::Lazy;
use std::sync::Mutex;

static NEXT_TOKEN: Lazy<Mutex<char>> = Lazy::new(|| Mutex::new('a'));

#[derive(Clone, PartialEq, Debug)]
pub struct Player {
    pub name: String,
    pub token: char,
}

impl Player {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        if name.is_empty() {
            panic!("Player must have a name.")
        }

        let mut token_lock = NEXT_TOKEN.lock().unwrap();
        let token = *token_lock;

        // Increment token for the next player
        if *token_lock < 'z' {
            *token_lock = (token as u8 + 1) as char;
        }

        Self { name, token }
    }
}
