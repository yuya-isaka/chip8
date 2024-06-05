struct Chip8 {
    cop: u16,
    re: [u8; 2],
}

impl Chip8 {
    fn readop(&self) -> u16 {
        self.cop
    }

    fn run(&mut self) {
        let op = self.readop();

        let a = ((op >> 12) & 0xf) as u8;
        let b = ((op >> 8) & 0xf) as u8;
        let c = ((op >> 4) & 0xf) as u8;
        let d = (op & 0xf) as u8;

        match (a, b, c, d) {
            (0x8, _, _, 0x4) => self.add_bc(b, c),
            _ => todo!(),
        }
    }

    fn add_bc(&mut self, b: u8, c: u8) {
        self.re[b as usize] += self.re[c as usize];
    }
}

fn main() {
    let mut chip8 = Chip8 { cop: 0, re: [0; 2] };

    chip8.cop = 0x8014;
    chip8.re[0] = 40;
    chip8.re[1] = 2;

    chip8.run();

    assert_eq!(chip8.re[0], 42);
}
