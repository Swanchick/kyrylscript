use std::time::Duration;

use ks_core::{compiler_new::compiler::CompilerNew, lexer::lexer::Lexer, parser::parser::Parser};
use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

fn main() -> KsResult<()> {
    let mut lexer = Lexer::load("examples/test.ks")?;
    lexer.lexer()?;

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    let block = parser.start()?;

    let mut compiler = CompilerNew::new();
    compiler.compile(block)?;

    let program = compiler.program();
    let mut bytes = program.serialize();
    let bytes_len = bytes.len() as u32;
    let mut bytes_len = bytes_len.to_le_bytes().to_vec();

    let mut final_program = Vec::<u8>::with_capacity(bytes.len() + bytes_len.len());
    final_program.append(&mut bytes_len);
    final_program.append(&mut bytes);

    println!("{:X?}", final_program);

    let ports = serialport::available_ports().expect("Hello World");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyACM0", 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| KsError::runtime(&e.to_string()))?;

    port.write_all(&final_program)
        .map_err(|e| KsError::runtime(&e.to_string()))?;

    port.flush().map_err(|e| KsError::runtime(&e.to_string()))?;

    println!("Sent {} bytes", final_program.len());

    Ok(())
}
