use crate::registers::*

// MMU trait placeholder ->> learn what is a train
pub trait MMU {}

// --- Instruction definitions ---
// Core CPU comtext
pub struct CpuContext<'a, T: Mmu>{
    pub regs: Registers,
    pub mmu: &'a mut T, 
    pub fetched_data: u16,
    pub cur_inst: &'a Instruction,
}

// Instruction type
#[derive(Debug, Copy, Clone)]
pub enum InstructionType{
    NONE, NOP, LD, INC, DEC, RLCA, ADD, RRCA, STOP, RLA, JR, RRA, DAA, CPL, SCF, CCF, 
    HALT, ADC, SUB, SBC, AND, XOR, OR, CP, POP, JP, PUSH, RET, CB, CALL, RETI, LDH, 
    JPHL, DI, EI, RST, ERR, RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL, BIT, RES, SET
}

impl Default for InstructionType{
    fn default() -> Self { InstructionType::NONE}
}

//  Register type 
#[derive(Debug, Copy, Clone)]
pub struct RegType{
    NONE, A, F, B, C, D, E, H, L, AF, BC, DE, HL, SP, PC
}

impl Default for RegType{
    fn default() -> Self {RegType::NONE}
}

// Addressing type
#[derive(Debug, Copy, Clone)]
pub struct AddressingMode{
    IMP, R_D16, MR_R, R, R_D8, RLCA, A16_R, R_MR, JR_D8, R_A16, 
    HLI_R, R_HLI, HLD_R, R_HLD, MR, MR_D8, A8_R, R_A8, HL_SPR, D8, D16, CB
}

impl Default for AddressingMode{
    fn default() -> Self { AddressingMode::IMP}
}

// Condition type
#[derive(Debug, Copy, Clone)]
pub enum ConditionType{
    NONE, NZ, Z, NC, C
}

impl Default for ConditionType{
    fn default() -> Self {ConditionType::NONE}
}

// Instruction structure (metadata)
#[derive(Debug, Copy, Clone, Default)] // Default derived for cleaner table entries
pub struct Instruction {
    pub instr_type: Instruction, 
    pub mode: AddressingMode,
    pub reg_1: RegType,
    pub reg_2: RegType,
    pub cond: ConditionType,
    pub param: Option<u8>,
}

impl Instruction{
    pub const fn new(
        instr_type: InstructionType,
        mode: AddressingMode,
        reg_1: RegType,
        reg_2: RegType,
        cond: ConditionType,
        param: Option<u8>,
    ) -> Self {
        Self { instr_type, mode, reg_1, reg_2, cond, param}
    }
}

// --- Instruction table
use InstructionType as IN;
use RegType as RT;
use AddressingMode as AM;
use ConditionType as CT;

pub const INSTRUCTIONS: [Instruction; 0x100] = [
    // Opcode 0x00 to 0x0F
    Instruction { instr_type: IN::NOP, mode: AM::IMP, ..Default::default()}, //0x00 NOP
    Instruction { instr_type: IN::LD, mode: AM::R_D16, reg_1: RT::BC, ..Default::default() }, // 0x01: LD BC, d16
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::BC, reg_2: RT::A, ..Default::default() }, // 0x02: LD (BC), A
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::BC, ..Default::default() },    // 0x03: INC BC
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::B, ..Default::default() },     // 0x04: INC B
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::B, ..Default::default() },     // 0x05: DEC B
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::B, ..Default::default() },   // 0x06: LD B, d8
    Instruction { instr_type: IN::RLCA, mode: AM::IMP, ..Default::default() },       // 0x07: RLCA
    Instruction { instr_type: IN::LD, mode: AM::A16_R, reg_2: RT::SP, ..Default::default() }, // 0x08: LD (a16), SP
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::HL, reg_2: RT::BC, ..Default::default() },// 0x09: ADD HL, BC
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::BC, ..Default::default() }, // 0x0A: LD A, (BC)
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::BC, ..Default::default() },    // 0x0B: DEC BC
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::C, ..Default::default() },     // 0x0C: INC C
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::C, ..Default::default() },     // 0x0D: DEC C
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::C, ..Default::default() },   // 0x0E: LD C, d8
    Instruction { instr_type: IN::RRCA, mode: AM::IMP, ..Default::default() },       // 0x0F: RRCA

    // Opcode 0x10 to 0x1F
    Instruction { instr_type: IN::STOP, mode: AM::IMP, ..Default::default() },       // 0x10: STOP
    Instruction { instr_type: IN::LD, mode: AM::R_D16, reg_1: RT::DE, ..Default::default() }, // 0x11: LD DE, d16
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::DE, reg_2: RT::A, ..Default::default() }, // 0x12: LD (DE), A
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::DE, ..Default::default() },    // 0x13: INC DE
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::D, ..Default::default() },     // 0x14: INC D
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::D, ..Default::default() },     // 0x15: DEC D
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::D, ..Default::default() },   // 0x16: LD D, d8
    Instruction { instr_type: IN::RLA, mode: AM::IMP, ..Default::default() },       // 0x17: RLA
    Instruction { instr_type: IN::JR, mode: AM::D8, ..Default::default() },          // 0x18: JR r8
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::HL, reg_2: RT::DE, ..Default::default() },// 0x19: ADD HL, DE
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::DE, ..Default::default() }, // 0x1A: LD A, (DE)
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::DE, ..Default::default() },    // 0x1B: DEC DE
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::E, ..Default::default() },     // 0x1C: INC E
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::E, ..Default::default() },     // 0x1D: DEC E
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::E, ..Default::default() },   // 0x1E: LD E, d8
    Instruction { instr_type: IN::RRA, mode: AM::IMP, ..Default::default() },       // 0x1F: RRA

    // Opcode 0x20 to 0x2F
    Instruction { instr_type: IN::JR, mode: AM::D8, cond: CT::NZ, ..Default::default() }, // 0x20: JR NZ, r8
    Instruction { instr_type: IN::LD, mode: AM::R_D16, reg_1: RT::HL, ..Default::default() }, // 0x21: LD HL, d16
    Instruction { instr_type: IN::LD, mode: AM::HLI_R, reg_1: RT::HL, reg_2: RT::A, ..Default::default() }, // 0x22: LD (HL+), A
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::HL, ..Default::default() },    // 0x23: INC HL
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::H, ..Default::default() },     // 0x24: INC H
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::H, ..Default::default() },     // 0x25: DEC H
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::H, ..Default::default() },   // 0x26: LD H, d8
    Instruction { instr_type: IN::DAA, mode: AM::IMP, ..Default::default() },       // 0x27: DAA
    Instruction { instr_type: IN::JR, mode: AM::D8, cond: CT::Z, ..Default::default() }, // 0x28: JR Z, r8
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::HL, reg_2: RT::HL, ..Default::default() },// 0x29: ADD HL, HL
    Instruction { instr_type: IN::LD, mode: AM::R_HLI, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x2A: LD A, (HL+)
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::HL, ..Default::default() },    // 0x2B: DEC HL
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::L, ..Default::default() },     // 0x2C: INC L
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::L, ..Default::default() },     // 0x2D: DEC L
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::L, ..Default::default() },   // 0x2E: LD L, d8
    Instruction { instr_type: IN::CPL, mode: AM::IMP, ..Default::default() },       // 0x2F: CPL

    // Opcode 0x30 to 0x3F
    Instruction { instr_type: IN::JR, mode: AM::D8, cond: CT::NC, ..Default::default() }, // 0x30: JR NC, r8
    Instruction { instr_type: IN::LD, mode: AM::R_D16, reg_1: RT::SP, ..Default::default() }, // 0x31: LD SP, d16
    Instruction { instr_type: IN::LD, mode: AM::HLD_R, reg_1: RT::HL, reg_2: RT::A, ..Default::default() }, // 0x32: LD (HL-), A
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::SP, ..Default::default() },    // 0x33: INC SP
    Instruction { instr_type: IN::INC, mode: AM::MR, reg_1: RT::HL, ..Default::default() },    // 0x34: INC (HL)
    Instruction { instr_type: IN::DEC, mode: AM::MR, reg_1: RT::HL, ..Default::default() },    // 0x35: DEC (HL)
    Instruction { instr_type: IN::LD, mode: AM::MR_D8, reg_1: RT::HL, ..Default::default() },  // 0x36: LD (HL), d8
    Instruction { instr_type: IN::SCF, mode: AM::IMP, ..Default::default() },       // 0x37: SCF
    Instruction { instr_type: IN::JR, mode: AM::D8, cond: CT::C, ..Default::default() }, // 0x38: JR C, r8
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::HL, reg_2: RT::SP, ..Default::default() },// 0x39: ADD HL, SP
    Instruction { instr_type: IN::LD, mode: AM::R_HLD, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x3A: LD A, (HL-)
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::SP, ..Default::default() },    // 0x3B: DEC SP
    Instruction { instr_type: IN::INC, mode: AM::R, reg_1: RT::A, ..Default::default() },     // 0x3C: INC A
    Instruction { instr_type: IN::DEC, mode: AM::R, reg_1: RT::A, ..Default::default() },     // 0x3D: DEC A
    Instruction { instr_type: IN::LD, mode: AM::R_D8, reg_1: RT::A, ..Default::default() },   // 0x3E: LD A, d8
    Instruction { instr_type: IN::CCF, mode: AM::IMP, ..Default::default() },       // 0x3F: CCF

    // Opcode 0x40 to 0x4F (LD B/C, r)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::B, ..Default::default() }, // 0x40: LD B, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::C, ..Default::default() }, // 0x41: LD B, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::D, ..Default::default() }, // 0x42: LD B, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::E, ..Default::default() }, // 0x43: LD B, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::H, ..Default::default() }, // 0x44: LD B, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::L, ..Default::default() }, // 0x45: LD B, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::B, reg_2: RT::HL, ..Default::default() }, // 0x46: LD B, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::B, reg_2: RT::A, ..Default::default() }, // 0x47: LD B, A
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::B, ..Default::default() }, // 0x48: LD C, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::C, ..Default::default() }, // 0x49: LD C, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::D, ..Default::default() }, // 0x4A: LD C, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::E, ..Default::default() }, // 0x4B: LD C, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::H, ..Default::default() }, // 0x4C: LD C, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::L, ..Default::default() }, // 0x4D: LD C, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::C, reg_2: RT::HL, ..Default::default() }, // 0x4E: LD C, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::C, reg_2: RT::A, ..Default::default() }, // 0x4F: LD C, A

    // Opcode 0x50 to 0x5F (LD D/E, r)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::B, ..Default::default() }, // 0x50: LD D, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::C, ..Default::default() }, // 0x51: LD D, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::D, ..Default::default() }, // 0x52: LD D, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::E, ..Default::default() }, // 0x53: LD D, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::H, ..Default::default() }, // 0x54: LD D, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::L, ..Default::default() }, // 0x55: LD D, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::D, reg_2: RT::HL, ..Default::default() }, // 0x56: LD D, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::D, reg_2: RT::A, ..Default::default() }, // 0x57: LD D, A
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::B, ..Default::default() }, // 0x58: LD E, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::C, ..Default::default() }, // 0x59: LD E, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::D, ..Default::default() }, // 0x5A: LD E, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::E, ..Default::default() }, // 0x5B: LD E, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::H, ..Default::default() }, // 0x5C: LD E, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::L, ..Default::default() }, // 0x5D: LD E, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::E, reg_2: RT::HL, ..Default::default() }, // 0x5E: LD E, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::E, reg_2: RT::A, ..Default::default() }, // 0x5F: LD E, A

    // Opcode 0x60 to 0x6F (LD H/L, r)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::B, ..Default::default() }, // 0x60: LD H, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::C, ..Default::default() }, // 0x61: LD H, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::D, ..Default::default() }, // 0x62: LD H, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::E, ..Default::default() }, // 0x63: LD H, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::H, ..Default::default() }, // 0x64: LD H, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::L, ..Default::default() }, // 0x65: LD H, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::H, reg_2: RT::HL, ..Default::default() }, // 0x66: LD H, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::H, reg_2: RT::A, ..Default::default() }, // 0x67: LD H, A
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::B, ..Default::default() }, // 0x68: LD L, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::C, ..Default::default() }, // 0x69: LD L, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::D, ..Default::default() }, // 0x6A: LD L, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::E, ..Default::default() }, // 0x6B: LD L, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::H, ..Default::default() }, // 0x6C: LD L, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::L, ..Default::default() }, // 0x6D: LD L, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::L, reg_2: RT::HL, ..Default::default() }, // 0x6E: LD L, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::L, reg_2: RT::A, ..Default::default() }, // 0x6F: LD L, A

    // Opcode 0x70 to 0x7F (LD (HL), r / HALT / LD A, r/m)
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::B, ..Default::default() }, // 0x70: LD (HL), B
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::C, ..Default::default() }, // 0x71: LD (HL), C
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::D, ..Default::default() }, // 0x72: LD (HL), D
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::E, ..Default::default() }, // 0x73: LD (HL), E
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::H, ..Default::default() }, // 0x74: LD (HL), H
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::L, ..Default::default() }, // 0x75: LD (HL), L
    Instruction { instr_type: IN::HALT, mode: AM::IMP, ..Default::default() },       // 0x76: HALT
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::HL, reg_2: RT::A, ..Default::default() }, // 0x77: LD (HL), A
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0x78: LD A, B
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0x79: LD A, C
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0x7A: LD A, D
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0x7B: LD A, E
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0x7C: LD A, H
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0x7D: LD A, L
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x7E: LD A, (HL)
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0x7F: LD A, A

    // Opcode 0x80 to 0x8F (ADD / ADC)
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0x80: ADD A, B
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0x81: ADD A, C
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0x82: ADD A, D
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0x83: ADD A, E
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0x84: ADD A, H
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0x85: ADD A, L
    Instruction { instr_type: IN::ADD, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x86: ADD A, (HL)
    Instruction { instr_type: IN::ADD, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0x87: ADD A, A
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0x88: ADC A, B
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0x89: ADC A, C
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0x8A: ADC A, D
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0x8B: ADC A, E
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0x8C: ADC A, H
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0x8D: ADC A, L
    Instruction { instr_type: IN::ADC, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x8E: ADC A, (HL)
    Instruction { instr_type: IN::ADC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0x8F: ADC A, A

    // Opcode 0x90 to 0x9F (SUB / SBC)
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0x90: SUB B
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0x91: SUB C
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0x92: SUB D
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0x93: SUB E
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0x94: SUB H
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0x95: SUB L
    Instruction { instr_type: IN::SUB, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x96: SUB (HL)
    Instruction { instr_type: IN::SUB, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0x97: SUB A
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0x98: SBC A, B
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0x99: SBC A, C
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0x9A: SBC A, D
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0x9B: SBC A, E
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0x9C: SBC A, H
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0x9D: SBC A, L
    Instruction { instr_type: IN::SBC, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0x9E: SBC A, (HL)
    Instruction { instr_type: IN::SBC, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0x9F: SBC A, A

    // Opcode 0xA0 to 0xAF (AND / XOR)
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0xA0: AND B
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0xA1: AND C
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0xA2: AND D
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0xA3: AND E
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0xA4: AND H
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0xA5: AND L
    Instruction { instr_type: IN::AND, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0xA6: AND (HL)
    Instruction { instr_type: IN::AND, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0xA7: AND A
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0xA8: XOR B
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0xA9: XOR C
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0xAA: XOR D
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0xAB: XOR E
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0xAC: XOR H
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0xAD: XOR L
    Instruction { instr_type: IN::XOR, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0xAE: XOR (HL)
    Instruction { instr_type: IN::XOR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0xAF: XOR A

    // Opcode 0xB0 to 0xBF (OR / CP)
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0xB0: OR B
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0xB1: OR C
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0xB2: OR D
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0xB3: OR E
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0xB4: OR H
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0xB5: OR L
    Instruction { instr_type: IN::OR, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0xB6: OR (HL)
    Instruction { instr_type: IN::OR, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0xB7: OR A
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::B, ..Default::default() }, // 0xB8: CP B
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0xB9: CP C
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::D, ..Default::default() }, // 0xBA: CP D
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::E, ..Default::default() }, // 0xBB: CP E
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::H, ..Default::default() }, // 0xBC: CP H
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::L, ..Default::default() }, // 0xBD: CP L
    Instruction { instr_type: IN::CP, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::HL, ..Default::default() }, // 0xBE: CP (HL)
    Instruction { instr_type: IN::CP, mode: AM::R_R, reg_1: RT::A, reg_2: RT::A, ..Default::default() }, // 0xBF: CP A

    // Opcode 0xC0 to 0xCF
    Instruction { instr_type: IN::RET, mode: AM::IMP, cond: CT::NZ, ..Default::default() }, // 0xC0: RET NZ
    Instruction { instr_type: IN::POP, mode: AM::R, reg_1: RT::BC, ..Default::default() }, // 0xC1: POP BC
    Instruction { instr_type: IN::JP, mode: AM::D16, cond: CT::NZ, ..Default::default() }, // 0xC2: JP NZ, a16
    Instruction { instr_type: IN::JP, mode: AM::D16, ..Default::default() }, // 0xC3: JP a16
    Instruction { instr_type: IN::CALL, mode: AM::D16, cond: CT::NZ, ..Default::default() }, // 0xC4: CALL NZ, a16
    Instruction { instr_type: IN::PUSH, mode: AM::R, reg_1: RT::BC, ..Default::default() }, // 0xC5: PUSH BC
    Instruction { instr_type: IN::ADD, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xC6: ADD A, d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x00), ..Default::default() }, // 0xC7: RST 00H
    Instruction { instr_type: IN::RET, mode: AM::IMP, cond: CT::Z, ..Default::default() }, // 0xC8: RET Z
    Instruction { instr_type: IN::RET, mode: AM::IMP, ..Default::default() }, // 0xC9: RET
    Instruction { instr_type: IN::JP, mode: AM::D16, cond: CT::Z, ..Default::default() }, // 0xCA: JP Z, a16
    Instruction { instr_type: IN::CB, mode: AM::CB, ..Default::default() }, // 0xCB: PREFIX CB
    Instruction { instr_type: IN::CALL, mode: AM::D16, cond: CT::Z, ..Default::default() }, // 0xCC: CALL Z, a16
    Instruction { instr_type: IN::CALL, mode: AM::D16, ..Default::default() }, // 0xCD: CALL a16
    Instruction { instr_type: IN::ADC, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xCE: ADC A, d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x08), ..Default::default() }, // 0xCF: RST 08H

    // Opcode 0xD0 to 0xDF
    Instruction { instr_type: IN::RET, mode: AM::IMP, cond: CT::NC, ..Default::default() }, // 0xD0: RET NC
    Instruction { instr_type: IN::POP, mode: AM::R, reg_1: RT::DE, ..Default::default() }, // 0xD1: POP DE
    Instruction { instr_type: IN::JP, mode: AM::D16, cond: CT::NC, ..Default::default() }, // 0xD2: JP NC, a16
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xD3: UNUSED
    Instruction { instr_type: IN::CALL, mode: AM::D16, cond: CT::NC, ..Default::default() }, // 0xD4: CALL NC, a16
    Instruction { instr_type: IN::PUSH, mode: AM::R, reg_1: RT::DE, ..Default::default() }, // 0xD5: PUSH DE
    Instruction { instr_type: IN::SUB, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xD6: SUB d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x10), ..Default::default() }, // 0xD7: RST 10H
    Instruction { instr_type: IN::RET, mode: AM::IMP, cond: CT::C, ..Default::default() }, // 0xD8: RET C
    Instruction { instr_type: IN::RETI, mode: AM::IMP, ..Default::default() }, // 0xD9: RETI
    Instruction { instr_type: IN::JP, mode: AM::D16, cond: CT::C, ..Default::default() }, // 0xDA: JP C, a16
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xDB: UNUSED
    Instruction { instr_type: IN::CALL, mode: AM::D16, cond: CT::C, ..Default::default() }, // 0xDC: CALL C, a16
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xDD: UNUSED
    Instruction { instr_type: IN::SBC, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xDE: SBC A, d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x18), ..Default::default() }, // 0xDF: RST 18H

    // Opcode 0xE0 to 0xEF
    Instruction { instr_type: IN::LDH, mode: AM::A8_R, reg_2: RT::A, ..Default::default() }, // 0xE0: LDH (a8), A
    Instruction { instr_type: IN::POP, mode: AM::R, reg_1: RT::HL, ..Default::default() }, // 0xE1: POP HL
    Instruction { instr_type: IN::LD, mode: AM::MR_R, reg_1: RT::C, reg_2: RT::A, ..Default::default() }, // 0xE2: LD (C), A
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xE3: UNUSED
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xE4: UNUSED
    Instruction { instr_type: IN::PUSH, mode: AM::R, reg_1: RT::HL, ..Default::default() }, // 0xE5: PUSH HL
    Instruction { instr_type: IN::AND, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xE6: AND d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x20), ..Default::default() }, // 0xE7: RST 20H
    Instruction { instr_type: IN::ADD, mode: AM::HL_SPR, reg_1: RT::SP, ..Default::default() }, // 0xE8: ADD SP, r8
    Instruction { instr_type: IN::JP, mode: AM::JPHL, reg_1: RT::HL, ..Default::default() }, // 0xE9: JP (HL)
    Instruction { instr_type: IN::LD, mode: AM::A16_R, reg_2: RT::A, ..Default::default() }, // 0xEA: LD (a16), A
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xEB: UNUSED
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xEC: UNUSED
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xED: UNUSED
    Instruction { instr_type: IN::XOR, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xEE: XOR d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x28), ..Default::default() }, // 0xEF: RST 28H

    // Opcode 0xF0 to 0xFF
    Instruction { instr_type: IN::LDH, mode: AM::R_A8, reg_1: RT::A, ..Default::default() }, // 0xF0: LDH A, (a8)
    Instruction { instr_type: IN::POP, mode: AM::R, reg_1: RT::AF, ..Default::default() }, // 0xF1: POP AF
    Instruction { instr_type: IN::LD, mode: AM::R_MR, reg_1: RT::A, reg_2: RT::C, ..Default::default() }, // 0xF2: LD A, (C)
    Instruction { instr_type: IN::DI, mode: AM::IMP, ..Default::default() }, // 0xF3: DI
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xF4: UNUSED
    Instruction { instr_type: IN::PUSH, mode: AM::R, reg_1: RT::AF, ..Default::default() }, // 0xF5: PUSH AF
    Instruction { instr_type: IN::OR, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xF6: OR d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x30), ..Default::default() }, // 0xF7: RST 30H
    Instruction { instr_type: IN::LD, mode: AM::HL_SPR, reg_1: RT::HL, reg_2: RT::SP, ..Default::default() }, // 0xF8: LD HL, SP+r8
    Instruction { instr_type: IN::LD, mode: AM::R_R, reg_1: RT::SP, reg_2: RT::HL, ..Default::default() }, // 0xF9: LD SP, HL
    Instruction { instr_type: IN::LD, mode: AM::R_A16, reg_1: RT::A, ..Default::default() }, // 0xFA: LD A, (a16)
    Instruction { instr_type: IN::EI, mode: AM::IMP, ..Default::default() }, // 0xFB: EI
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xFC: UNUSED
    Instruction { instr_type: IN::ERR, mode: AM::IMP, ..Default::default() }, // 0xFD: UNUSED
    Instruction { instr_type: IN::CP, mode: AM::R_D8, reg_1: RT::A, ..Default::default() }, // 0xFE: CP d8
    Instruction { instr_type: IN::RST, mode: AM::IMP, param: Some(0x38), ..Default::default() }  // 0xFF: RST 38H
];


// --- TESTS ---
#[cfg(test)]
mod test {
    use super::*;

    // Verify total no. of instructions
    #[test]
    fn instruction_table_size(){
        assert_eq!(INSTRUCTIONS.len(), 0x100, "Instruction table must contain 256 opcodes (0x00 to 0xFF)");
    }

    // Check simple NOP instruction (0x00)
    #[test]
    fn text_0x00_nop(){
        let inst = &INSTRUCTIONS[0x00];
        assert_eq!(instr.instr_type, InstructionType::NOP, "0x00 should be NOP");
        assert_eq!(instr.mode, AddressingMode::IMP, "0x00 should be IMP");
        assert_eq!(instr.mode, ConditionType::NONE);
        assert_eq!(instr.reg_1, RegType::NONE);
    }
    // Check simple Load instruction (0x01: LD BC, d16)
    #[test]
    fn text_0x00_nop(){
        let inst = &INSTRUCTIONS[0x10];
        assert_eq!(instr.instr_type, InstructionType::LD, "0x01 should be LD");
        assert_eq!(instr.mode, AddressingMode::R_D16, "0x00 should be R_D16");
        assert_eq!(instr.reg_1, RegType::BC, "0x01 target should be BC");
        assert_eq!(instr.reg_2, RegType::NONE);
    }

    // Check a conditional jump instruction (0x20: JR NZ, r8)
    #[test]
    fn test_0x20_jr_nz_r8() {
        let instr = &INSTRUCTIONS[0x20];
        assert_eq!(instr.instr_type, InstructionType::JR, "0x20 should be JR");
        assert_eq!(instr.mode, AddressingMode::D8, "0x20 should be D8 mode (relative jump)");
        assert_eq!(instr.cond, ConditionType::NZ, "0x20 should be conditional NZ");
    }

    // Check the prefix instruction (0xCB)
    #[test]
    fn test_0xcb_prefix() {
        let instr = &INSTRUCTIONS[0xCB];
        assert_eq!(instr.instr_type, InstructionType::CB, "0xCB should be PREFIX CB");
        assert_eq!(instr.mode, AddressingMode::CB, "0xCB should have CB addressing mode");
    }

    // Check an RST instruction (0xFF: RST 38H) which uses the 'param' field
    #[test]
    fn test_0xff_rst_38h() {
        let instr = &INSTRUCTIONS[0xFF];
        assert_eq!(instr.instr_type, InstructionType::RST, "0xFF should be RST");
        assert_eq!(instr.mode, AddressingMode::IMP, "0xFF should be IMP");
        assert_eq!(instr.param, Some(0x38), "0xFF RST target should be 0x38");
    }
}

