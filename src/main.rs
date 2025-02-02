use anyhow::Result;
use text_analyzer::read_file;

fn main() -> Result<()> {
    read_file("./examples/tale.txt");
    Ok(()) 
}
