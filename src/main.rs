struct Feed {
    id: i128,
    name: String,
    url: String,
}
struct Article {
    title: String,
    version: i32,
    path: String,
    feed: Feed,
    url: String,
}

struct Settings {
    feeds: Vec<Feed>,
    check_every: i32,
    save_root: String,
    config_path: String,
}

fn main() {
    println!("Hello, world!");
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            feeds: Vec::new(),
            check_every: 5,
            save_root: "$HOME/.adrastea".to_string(),
            config_path: "$HOME/.config/adrastea.json".to_string(),
        }
    }
}
