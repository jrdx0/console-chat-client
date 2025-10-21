use std::io::{self, Write, stdout};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{KeyCode, KeyModifiers, read},
    style, terminal,
};

const LOGO: &str = r#"
██████╗ ██╗   ██╗███████╗████████╗██╗███╗   ██╗ ██████╗      ██████╗██╗  ██╗ █████╗ ████████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██║████╗  ██║██╔════╝     ██╔════╝██║  ██║██╔══██╗╚══██╔══╝
██████╔╝██║   ██║███████╗   ██║   ██║██╔██╗ ██║██║  ███╗    ██║     ███████║███████║   ██║
██╔══██╗██║   ██║╚════██║   ██║   ██║██║╚██╗██║██║   ██║    ██║     ██╔══██║██╔══██║   ██║
██║  ██║╚██████╔╝███████║   ██║   ██║██║ ╚████║╚██████╔╝    ╚██████╗██║  ██║██║  ██║   ██║
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝
"#;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::SetForegroundColor(style::Color::Rgb {
            r: (36),
            g: (150),
            b: (107),
        }))?
        .queue(style::Print(LOGO))?
        .queue(cursor::MoveToNextLine(1))?;

    stdout.flush()?;

    let (cols, _) = terminal::size()?;

    stdout
        .queue(style::SetForegroundColor(style::Color::DarkGrey))?
        .queue(style::Print("—".repeat(cols.into())))?
        .queue(cursor::MoveToNextLine(1))?
        .queue(style::SetForegroundColor(style::Color::DarkBlue))?
        .queue(style::Print("> "))?
        .queue(cursor::MoveRight(1))?
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveToNextLine(1))?
        .queue(style::SetForegroundColor(style::Color::DarkGrey))?
        .queue(style::Print("—".repeat(cols.into())))?;

    stdout.flush()?;

    stdout.execute(cursor::RestorePosition)?;

    let mut user_input = String::new();

    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        match event.code {
            KeyCode::Backspace => {
                if event.modifiers == KeyModifiers::CONTROL {
                    if let Some(pos) = user_input.rfind(" ") {
                        user_input = user_input[..pos].into();
                    } else {
                        user_input = "".into();
                    }
                } else {
                    user_input.pop();
                }
            }
            KeyCode::Char(c) => {
                user_input.push(c);
            }
            _ => continue,
        }

        stdout
            .queue(cursor::RestorePosition)?
            .queue(terminal::Clear(terminal::ClearType::UntilNewLine))?
            .queue(style::SetForegroundColor(style::Color::White))?
            .queue(style::Print(&user_input))?;

        stdout.flush()?;
    }

    Ok(())
}
