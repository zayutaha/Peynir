use eyre::Result;
use std::io::Write;

pub fn display_tokens(tokens: Vec<(String, String)>) -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    for token in tokens.iter() {
        println!("{:?} => {:?}", token.0, token.1);
    }
    Ok(())
}

fn update_display(tokens: Vec<(String, String)>) {
    let _ = std::io::stdout().flush();
    let _ = display_tokens(tokens);
}
