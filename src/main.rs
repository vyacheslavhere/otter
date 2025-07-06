// модули
mod apps;
mod config;
mod selector;

use std::fs::File;
use std::path::PathBuf;
// импорты
use clap::Parser;

// cli
#[derive(Parser)]
struct CLI {
    #[arg(value_name = "file")]
    path: PathBuf,
}


// выбор твика
fn main() {
    // аргументы
    let args = CLI::parse();
    // файл и конфиг
    let file = File::open(args.path)
        .expect("Could not open config.json");
    let config = serde_json::from_reader(file).expect("config is not found");
    // запуск
    selector::run(&config);
}