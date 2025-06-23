// конфиг
use cliclack::Select;
use crate::{apps, config};

// показ категории
fn show_category(name: String) {
    if let Some(category) = config::APPS_CONFIG.categories.get(&name).cloned() {
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
                        run();
                    }
                    _ => {
                        let app = category.apps.get(&value).unwrap();
                        apps::install_app(app);
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
pub fn run() {
    println!("Otter apps installer 🦦🌿.");
    let mut menu: Select<String> = cliclack::select("Select app category: ");
    for (category_id, category) in config::APPS_CONFIG.categories.clone() {
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
                    show_category(tweak.to_string());
                }
            }
        }
        Err(err) => {
            panic!("selecting error occurred: {}", err)
        }
    }
}