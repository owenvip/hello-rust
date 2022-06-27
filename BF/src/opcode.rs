/*
 * @Description:
 * @Author: OwenWong
 * @Email: owen.cq.cn@gmail.com
 * @Date: 2022-06-27 10:11:43
 */
// 字符                              含义
//   >                               指针加一
//   <                               指针减一
//   +                               指针指向的字节的值加一
//   -                                指针指向的字节的值减一
//   .                                 输出指针指向的单元内容(ASCⅡ码)
//   ,                                 输入内容到指针指向的单元(ASCⅡ码)
//   [                                 如果指针指向的单元值为零，向后跳转到对应的]指令的次一指令处
//   ]                                 如果指针指向的单元值不为零，向前跳转到对应的[指令的次一指令处
#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2C => Opcode::PUTCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];
        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();
    }
}
