
/// This is based off of the algorithm presented in Annex D of
/// ITU-T Recommendation X.224 (1995).
/// `input` MUST include the checksum parameter with the value set to 0x0000.
pub fn calculate_checksum (input: &[u8], n: usize) -> u16 {
    let mut C0 = 0;
    let mut C1 = 0;
    let L = input.len();
    for i in 1..L {
        let octet = input[i - 1];
        C0 += octet as usize;
        C1 += C0;
    }
    let X = (L - n)*C0 - C1;
    let Y = C1 - (L - n + 1)*C0;
    u16::from_be_bytes([ X as u8, Y as u8 ])
}

pub fn verify_checksum (input: &[u8]) -> bool {
    let mut C0 = 0;
    let mut C1 = 0;
    let i = 1;
    let L = input.len();
    for i in 1..L {
        let octet = input[i - 1];
        C0 += octet as usize;
        C1 += C0;
    }
    (C0 == 0) && (C1 == 0)
}

pub struct Checksummer {
    pub C0: usize,
    pub C1: usize,
    pub L: usize,
}

impl Checksummer {

    pub fn new () -> Self {
        Checksummer {
            C0: 0,
            C1: 0,
            L: 0,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        let L = input.len();
        for i in 1..L {
            let octet = input[i - 1];
            self.C0 += octet as usize;
            self.C1 += self.C0;
        }
        self.L += L;
    }

    pub fn digest (&self, n: usize) -> u16 {
        let Checksummer { C0, C1, L } = self;
        let X = (L - n)*C0 - C1;
        let Y = C1 - (L - n + 1)*C0;
        u16::from_be_bytes([ X as u8, Y as u8 ])
    }

}
