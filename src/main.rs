use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, terminal_size};
use termion::AsyncReader;
use std::io::{Read, Write, Bytes, stdout, StdoutLock};
use std::thread;
use std::time::Duration;

static RILO_VERSION: &str = "0.0.1";

struct EditorConfig {
    screen: (u16, u16),
}

#[derive(PartialEq, Eq)]
enum Command {
    Quit,
    Move,
}

#[derive(PartialEq, Eq)]
struct Action {
    command: Command,
    character: Option<u8>,
}

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = async_stdin().bytes();
    let editorconfig = init_editor();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1))?;

    editor_rows(&mut stdout, editorconfig)?;

    loop {
        let actions = handle_input(&mut stdin);

        if actions.contains(&Action{command: Command::Quit, character: None}) {
            break
        }

        for action in actions {
            match action.command {
                Command::Move => move_cursor(&mut stdout, action.character)?,
                _ => (),
            }
        }

        // commands.into_iter()
        //     .filter(|x| x.command == Command::Print)
        //     .for_each(|x: Action | write!(stdout, "{:?}\r\n", x.character).unwrap());

        stdout.flush()?;

        thread::sleep(Duration::from_millis(50));
        stdout.flush()?;
    }

    Ok(())
}

fn handle_input(stdin: &mut Bytes<AsyncReader>) -> Vec<Action> {
    let mut out: Vec<Action> = vec![];

    for (_, byte) in stdin.enumerate() {
        let b = byte.unwrap();
        match b {
            b'q' => out.push(Action{command: Command::Quit, character: None}),
            b'w' | b'a' | b's' | b'd' => out.push(Action{command: Command::Move, character: Some(b)}),
            _ => (),
        };
    }

    out
}

fn editor_rows(stdout: &mut RawTerminal<StdoutLock>, econfig: EditorConfig) -> Result<(), std::io::Error> {
    for i in 0..econfig.screen.1-1 {
        if i == econfig.screen.1 / 3 {
            let padding: usize = (econfig.screen.0 as usize - RILO_VERSION.len() - 15) / 2 as usize;

            write!(stdout,
                "~{}Rilo editor -- {}{}\r\n",
                " ".repeat(padding),
                RILO_VERSION,
                " ".repeat(padding))?;
        } else {
            write!(stdout, "~\r\n")?;
        }
    }

    write!(stdout, "~")?;
    stdout.flush()?;
    Ok(())
}

fn init_editor() -> EditorConfig {
    EditorConfig{screen: terminal_size().unwrap()}
}

fn move_cursor(stdout: &mut RawTerminal<StdoutLock>, direction: Option<u8>) -> Result<(), std::io::Error> {
    // dbg!(direction);
    // dbg!(b'w');
    match direction.unwrap() {
        b'w' => write!(stdout, "{}", termion::cursor::Up(1))?,
        b'a' => write!(stdout, "{}", termion::cursor::Left(1))?,
        b's' => write!(stdout, "{}", termion::cursor::Down(1))?,
        b'd' => write!(stdout, "{}", termion::cursor::Right(1))?,
        _ => (),
    }
    Ok(())
}
