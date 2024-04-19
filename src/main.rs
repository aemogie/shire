use std::{
    io::stdout,
    process::{Command, Stdio},
};

use crossterm::{
    cursor::MoveLeft,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

fn main() {
    println!("welcome to the shire, a project that died before it started.");
    let mut out = stdout();
    let _ = enable_raw_mode();
    let _ = out.execute(Print("$ "));
    let mut cmd = String::new();
    loop {
        let Ok(Event::Key(key)) = event::read() else {
            continue;
        };
        match key {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('c'),
                ..
            } => break,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Backspace,
                ..
            } => {
                let _ = cmd.pop();
                let _ = execute!(out, MoveLeft(1), Print(' '), MoveLeft(1));
            }
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Enter,
                ..
            } => {
                if cmd == "exit" {
                    break;
                }
                let _ = disable_raw_mode();
                println!();
                Command::new("/bin/sh")
                    .arg("-c")
                    .stdout(Stdio::inherit())
                    .stdin(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .arg(cmd.clone())
                    .output()
                    .unwrap();
                cmd.clear();
                let _ = enable_raw_mode();
                let _ = execute!(out, Print("$ "));
            }
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Char(c),
                ..
            } => {
                cmd.push(c);
                let _ = out.execute(Print(c));
            }
            _ => {}
        };
    }
    let _ = disable_raw_mode();
}
