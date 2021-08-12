#[cfg(test)]
mod test_rv32i {
    // use crate::cpu::instruction::Instruction;
    // use crate::cpu::register::XRegister;
    // use crate::cpu::core::Core;

    // #[test]
    // fn test_add() {
    //     let mut core = Core::new();
    //
    //     let instruction = Instruction::add{
    //         rd: XRegister::x3,
    //         rs1: XRegister::x1,
    //         rs2: XRegister::x2
    //     };
    //
    //     core.x_registers[XRegister::x1] = 2;
    //     core.x_registers[XRegister::x2] = 4;
    //     instruction.execute(&mut core).unwrap();
    //     assert_eq!(core.x_registers[XRegister::x3], 6);
    //
    //     core.x_registers[XRegister::x1] = u64::MAX - 2;
    //     core.x_registers[XRegister::x2] = 6;
    //     instruction.execute(&mut core).unwrap();
    //     assert_eq!(core.x_registers[XRegister::x3], 3);
    // }
}