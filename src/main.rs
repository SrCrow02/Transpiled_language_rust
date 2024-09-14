mod transpiler;
mod file_io;

use file_io::{read_file, write_file};
use transpiler::transpile_code;
use std::io;

fn main() -> io::Result<()> {
    let input_file = "input.gan";
    let output_file = "output.rs";
    
    let code = read_file(input_file)?;

    let transpiled_code = transpile_code(&code);

    write_file(output_file, &transpiled_code)?;

    println!("Transpilation complete. Output written to {}", output_file);

    Ok(())
}
