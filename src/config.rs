// импорты
use serde::Deserialize;
use std::collections::HashMap;

// приложение
#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct App {
    pub name: String,
    pub description: String,
    pub cmd: String
}

// для статической переменной
unsafe impl Send for App {}
unsafe impl Sync for App {}

// категория
#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct Category {
    pub description: String,
    pub title: String,
    pub apps: HashMap<String, App>
}

// для статической переменной
unsafe impl Send for Category {}
unsafe impl Sync for Category {}

// конфиг
#[derive(Clone, Deserialize)]
pub struct Config {
    pub categories: HashMap<String, Category>
}
// для статической переменной
unsafe impl Send for Config {}
unsafe impl Sync for Config {}