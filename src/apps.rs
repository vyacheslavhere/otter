// импорты
use crate::config::{App, Config};
use std::process::{Command, Stdio};
use crate::selector;

// установка приложения
#[allow(unused_variables)]
pub fn install_app(app: &App, config: &Config) {
    // note
    cliclack::note(
        "the app installation command: ", 
        app.cmd.clone()).expect("not error occurred."
    );
    // установить ли
    let mut selected = cliclack::confirm("Install app? [y/n]");
    let selected = selected.interact();
    // матчим
    match selected {
        // если устанавливаем
        Ok(install) => {
            // если отказались
            if !install {
                println!("cancelled");
                selector::run(&config);
            }
            // создаём процесс
            let mut process = Command::new(app.cmd.clone())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .expect("failed to start process");
            // ждём
            let status = process.wait();
            // смотрим статус
            match status {
                Ok(status) => {
                    println!("installed app: {}", app.name);
                },
                Err(_) => eprintln!("failed to install app: {}", app.name),
            }
            // возвращаемся в меню
            selector::run(&config);
        }
        // если не устанавливаем
        Err(err) => {
            panic!("selecting error occurred: {}", err)
        }
    }
}