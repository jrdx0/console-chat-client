use std::io::{Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::MoveToNextLine,
    event::{KeyCode, read},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    let mut message = String::new();

    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        match event.code {
            _ => {
                let user_char = event.code.to_string();
                message.push_str(&user_char);
            }
        }

        let (cols, _) = terminal::size()?;

        stdout
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("—".repeat(cols.into())))?
            .queue(ResetColor)?
            .queue(MoveToNextLine(1))?
            .queue(Print(">"))?
            .queue(Print(&message))?
            .queue(MoveToNextLine(1))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("—".repeat(cols.into())))?
            .queue(ResetColor)?;

        stdout.flush()?;
    }

    Ok(())
}
