/*
 * @Description:
 * @Author: OwenWong
 * @Email: owen.cq.cn@gmail.com
 * @Date: 2022-06-27 09:50:32
 */
mod opcode;
use opcode::{Code, Opcode};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
    let code = Code::from(data);
    println!("{:?}", code.instrs);
    Ok(())
}
