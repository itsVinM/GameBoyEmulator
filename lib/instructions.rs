
// ===== INSTRUCTIONS =====
// core CPU context -
pub struct CpuContext<'a> {
    // add also memory interface
    pub regs: Registers,
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

// Register type
#[derive(Debug, Copy, Clone)]
pub enum RegType {
    NONE, A, F, B, C, D, E, H, L, AF, BC, DE, HL, SP, PC
}

// Addressing mode 
#[debug(Debug, Copy, Clone)]
pub enum AddressingMode{
    IMP, R_D16, MR_R, R, R_D8, RLCA, A16_R, R_MR, JR_D8, R_A16, 
    HLI_R, R_HLI, HLD_R, R_HLD, MR, MR_D8, A8_R, R_A8, HL_SPR, D8, D16, CB
}

// Condition type 
#[debug(Debug, Copy, Clone)]
pub enum ConditionType{
    NONE, NZ, Z, NC, C
}

// Instruction structure (metadata)
#[derive(Derive, Copy, Clone)]
pub struct Instruction{
    pub r#type:InstructionType,
    pub mode: AddressingMode,
    pub reg_1: RegType,
    pub reg_2: RegType,
    pub cond: ConditionType,
    pub param: Option<u8>,
}

impl Instruction{
    const fn new(r#type)
}
