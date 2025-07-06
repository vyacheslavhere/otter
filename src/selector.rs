// конфиг
use cliclack::Select;
use crate::{apps};
use crate::config::Config;

// показ категории
fn show_category(config: &Config, name: String) {
    if let Some(category) = config.categories.get(&name).cloned() {
        let mut menu: Select<String> = cliclack::select("🧭 Select app");
        for (app_id, app) in &category.apps {
            menu = menu.item(
                app_id.clone(),
                app.name.clone(),
                app.description.clone(),
            );
        }
        menu = menu.item(
            "back".to_string(),
            "back".to_string(),
            "back".to_string()
        );
        let selected = menu.interact();
        match selected {
            Ok(value) => {
                match value.as_str() {
                    "back" => {
                        run(config);
                    }
                    _ => {
                        let app = category.apps.get(&value).unwrap();
                        apps::install_app(app, config);
                    }
                }
            }
            Err(err) => {
                panic!("selecting error occurred: {}", err)
            }
        }
    }
    else {
        panic!("no category found with name {}", name);
    }
}
// запуск
pub fn run(config: &Config) {
    println!("Otter apps installer 🦦🌿.");
    let mut menu: Select<String> = cliclack::select("select app category: ");
    for (category_id, category) in config.categories.clone() {
        menu = menu.item(
            category_id,
            category.title,
            category.description
        );
    }
    let selected = menu.interact();
    match selected {
        Ok(tweak) => {
            match tweak {
                _ => {
                    show_category(config, tweak.to_string());
                }
            }
        }
        Err(err) => {
            panic!("selecting error occurred: {}", err)
        }
    }
}