use termion::raw::{IntoRawMode, RawTerminal};
use termion::async_stdin;
use termion::AsyncReader;
use std::io::{Read, Write, Bytes, stdout, StdoutLock};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Eq)]
enum Command {
    Quit,
    Print,
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

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))?;

    loop {
        let commands = handle_input(&mut stdin);

        if commands.contains(&Action{command: Command::Quit, character: None}) {
            break
        }

        commands.into_iter()
            .filter(|x| x.command == Command::Print)
            .for_each(|x: Action | write!(stdout, "{:?}\r\n", x.character).unwrap());

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
            _    => out.push(Action{command: Command::Print, character: Some(b)}),
        };
    }

    out
}
