use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut line = String::new();
    let c = std::io::stdin().read_to_string(&mut line)?;
    dbg!(line);
    println!("No. bytes read: {:?}", c);
    Ok(())
}
