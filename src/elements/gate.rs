use crate::elements::wire::Wire;
use crate::elements::Conduct;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Gate {
    in_1: Wire,
    in_2: Wire,
    out: Wire,
    act: fn(bool, bool) -> bool,
    tp: &'static str,
}

impl Gate {
    pub fn and(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| in_1 && in_2,
            tp: "and",
        };
        gate.conduct();
        gate
    }

    pub fn or(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| in_1 || in_2,
            tp: "or",
        };
        gate.conduct();
        gate
    }

    pub fn xor(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| in_1 ^ in_2,
            tp: "xor",
        };
        gate.conduct();
        gate
    }

    pub fn not(in_1: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2: Wire::new(),
            out,
            act: |in_1, _| !in_1,
            tp: "not",
        };
        gate.conduct();
        gate
    }

    pub fn nor(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| !(in_1 || in_2),
            tp: "nor",
        };
        gate.conduct();
        gate
    }

    pub fn nand(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| !(in_1 && in_2),
            tp: "nand",
        };
        gate.conduct();
        gate
    }

    pub fn wire_in_1(&mut self, wire: Wire) {
        self.in_1 = wire;
    }

    pub fn wire_in_2(&mut self, wire: Wire) {
        self.in_2 = wire;
    }

    pub fn wire_out(&mut self, wire: Wire) {
        self.out = wire;
    }

    pub fn get_in_1(&self) -> Wire {
        self.in_1.clone()
    }

    pub fn get_in_2(&self) -> Wire {
        self.in_2.clone()
    }

    pub fn get_out(&self) -> Wire {
        self.out.clone()
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "IN:{:#x}\nIN:{:#x}\nOUT:{:#x}",
            self.in_1.id(),
            self.in_2.id(),
            self.out.id()
        )?;
        writeln!(f, "{}({}, {})->{}", self.tp, self.in_1, self.in_2, self.out)
    }
}

impl Default for Gate {
    fn default() -> Self {
        Self::and(Wire::default(), Wire::default(), Wire::default())
    }
}

impl Conduct for Gate {
    #[inline]
    fn conduct(&self) {
        let in_1 = self.in_1.get();
        let in_2 = self.in_2.get();
        self.out.set((self.act)(in_1, in_2));
    }
}
