// импорты
use crate::config::App;
use std::process::{Command, Stdio};

// установка приложения
#[allow(unused_variables)]
pub fn install_app(app: &App) {
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
            // создаём процесс
            let mut process = Command::new("cmd")
                .arg("/C")
                .arg(app.cmd.clone())
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
        }
        // если не устанавливаем
        Err(err) => {
            panic!("selecting error occurred: {}", err)
        }
    }
}