use std::time::Duration;

use ks_core::{
    compiler_new::compiler::CompilerNew,
    lexer::lexer::Lexer,
    parser::{data_type::DataType, parser::Parser},
};
use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

fn register_parser_std(parser: &mut Parser) {
    parser.register_variable(
        "println",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );
}

fn register_compiler_std(compiler: &mut CompilerNew) {
    compiler.register_native("println", 0);
}

fn compile(path: &str) -> KsResult<Vec<u8>> {
    let mut lexer = Lexer::load(path)?;
    lexer.lexer()?;

    let mut parser = Parser::new();
    register_parser_std(&mut parser);
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    let block = parser.start()?;

    let mut compiler = CompilerNew::new();
    register_compiler_std(&mut compiler);
    compiler.compile(block)?;

    let program = compiler.program();
    println!("{:?}", program);

    let mut bytes = program.serialize();
    let bytes_len = bytes.len() as u32;
    let mut bytes_len = bytes_len.to_le_bytes().to_vec();

    let mut final_program = Vec::<u8>::with_capacity(bytes.len() + bytes_len.len());
    final_program.append(&mut bytes_len);
    final_program.append(&mut bytes);

    Ok(final_program)
}

fn send(port: &str, program: Vec<u8>) -> KsResult<()> {
    let mut port = serialport::new(port, 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| KsError::runtime(&e.to_string()))?;

    port.write_all(&program)
        .map_err(|e| KsError::runtime(&e.to_string()))?;

    port.flush().map_err(|e| KsError::runtime(&e.to_string()))?;

    println!("Sent {} bytes", program.len());

    Ok(())
}

fn main() -> KsResult<()> {
    let program = compile("examples/test.ks")?;

    println!("{:?}", program);

    let ports = serialport::available_ports().map_err(|e| KsError::runtime(&e.to_string()))?;
    for p in ports {
        println!("{}", p.port_name);
    }

    send("/dev/ttyACM0", program)?;

    Ok(())
}
