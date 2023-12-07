use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))?;

    loop {
        let b = stdin.next();
        match b {
            None => (),
            Some(Ok(17)) => break, // Ctrl-q
            Some(Ok(_)) => write!(stdout, "{:?}\n\r", b)?,
            _ => break,
        }
        stdout.flush()?;

        thread::sleep(Duration::from_millis(50));
        stdout.flush()?;
    }

    Ok(())
}
