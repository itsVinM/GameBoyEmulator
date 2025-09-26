//===== REGISTERS =====
pub enum GbSpeed{
    Single = 1,
    Double = 2,
}

#[derive(Copy, Clone)]
pub struct Registers{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    f: u8,
    pub pc: u16,
    pub sp: u16,
}

#[derive(Copy, Clone)]
pub enum CpuFlag {
    Z = 0x80, // zero flag - set if the result of an operation is zero
    N = 0x40, // subtract flag 
    H = 0x20, // half-carry flag - set if. carry or borrow from lower nibble (bit3-> 4)
    C = 0x10, // carry flag from the most significant bit (bit 7)
}

impl Registers {
    pub fn new() -> Registers{
        use CpuFlag::*;
        Registers{
                a: 0x01,
                f: C as u8 | H as u8 | Z as u8,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                pc: 0x0100, // program counter starts after bios 
                sp: 0xFFFE,
        }
    }
    // get 
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | ((self.c as u16))
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | ((self.e as u16))
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn hld(&mut self) -> u16 {
        let res = self.hl();
        self.sethl(res-1);
        res
    }
    pub fn hli(&mut self) -> u16 {
        let res = self.hl();
        self.sethl(res+1);
        res
    }

    // set 
    pub fn setaf(&mut self, value: u16){
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }
    pub fn setbc(&mut self, value: u16){
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    pub fn setde(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    pub fn sethl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    // Flag functions
    pub fn flag(&mut self, flags: &CpuFlag, set: bool){
        let mask = *flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;

    }

    pub fn getflag(&self, flags: &CpuFlag) -> bool {
        (self.f & (*flags as u8)) != 0
    }

    #[cfg(test)]
    fn setf(&mut self, flags: u8){
        self.f= flags & 0xF0;
    }
}

#[cfg(test)]
mod test {
    use super::CpuFlag::*;
    use super::Registers;
    
    #[test]
    fn wide_registers(){
        let mut reg = Registers::new();
        reg.a = 0x12;
        reg.setf(0x23);
        reg.b = 0x33;
        reg.c = 0x46;
        reg.d = 0x56;
        reg.e = 0x67;
        reg.h = 0x78;
        reg.l = 0x89;
        assert_eq!(reg.af(), 0x1220);
        assert_eq!(reg.bc(), 0x3346);
        assert_eq!(reg.de(), 0x5667);
        assert_eq!(reg.hl(), 0x7889);

        reg.setaf(0x1111);
        reg.setbc(0x1111);
        reg.setde(0x1111);
        reg.sethl(0x1111);
        assert_eq!(reg.af(), 0x1110);
        assert_eq!(reg.bc(), 0x1111);
        assert_eq!(reg.de(), 0x1111);
        assert_eq!(reg.hl(), 0x1111);
    }

    #[test]
    fn flags(){
        let mut reg = Registers::new();
        let flags = [C, H, N, Z];
        
        // check goodness of flags
        assert_eq!(reg.f & 0x0f, 0);

        reg.setf(0x00);
        for i in 0..4{
            let mask = flags[i];
            assert_eq!(reg.getflag(&mask), false);
            reg.flag(&mask, true);
            assert_eq!(reg.getflag(&mask), true);
            reg.flag(&mask, false);
            assert_eq!(reg.getflag(&mask), false); 
        }
    }

    #[test]
    fn hl_special(){
        let mut reg = Registers::new();
        reg.sethl(0x1234);
        assert_eq!(reg.hl(), 0x1234);
        assert_eq!(reg.hld(), 0x1234);
        assert_eq!(reg.hld(), 0x1233);
        assert_eq!(reg.hld(), 0x1232);
        assert_eq!(reg.hli(), 0x1231);
        assert_eq!(reg.hli(), 0x1232);
        assert_eq!(reg.hli(), 0x1233);
        assert_eq!(reg.hli(), 0x1234);
    }
}