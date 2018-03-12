extern crate config;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    snake_test: String,
}

fn main() {
    let mut c = config::Config::default();
    c.merge(config::File::new("Settings", config::FileFormat::Toml))
        .unwrap();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_into::<Settings>().unwrap();

    println!("{:?}", s);
}
