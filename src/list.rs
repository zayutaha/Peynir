use eyre::Result;

pub fn display_tokens(tokens: Vec<(String, String)>) -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    println!("┌──────────┬──────────────┐");
    println!("    Name         Code ");
    println!("├──────────┼──────────────┤");
    for token in tokens.iter() {
        println!("|{:<9} | {:<13}|", token.0, token.1);
    }
    println!("└──────────┴──────────────┘");
    Ok(())
}
