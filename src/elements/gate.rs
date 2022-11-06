use crate::elements::wire::Wire;
use crate::elements::Conduct;

#[derive(Clone)]
pub struct Gate {
    in_1: Wire,
    in_2: Wire,
    out: Wire,
    act: fn(bool, bool) -> bool,
}

impl Gate {
    pub fn and(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        let gate = Self {
            in_1,
            in_2,
            out,
            act: |in_1, in_2| in_1 && in_2,
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

#[cfg(test)]
mod test {
    use crate::elements::gate::Gate;
    use crate::elements::wire::Wire;
    use crate::elements::Conduct;

    #[test]
    pub fn test_and() {
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::and(wire_1.clone(), wire_2.clone(), wire_out.clone());
        assert_eq!(wire_out.get(), false);
        wire_1.set(true);
        wire_2.set(true);
        assert_eq!(wire_out.get(), false);
        key.conduct();
        assert_eq!(wire_out.get(), true);
        wire_1.set(false);
        wire_2.set(true);
        key.conduct();
        assert_eq!(wire_out.get(), false);
    }

    #[test]
    pub fn test_or() {
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::or(wire_1.clone(), wire_2.clone(), wire_out.clone());
        assert_eq!(wire_out.get(), false);
        wire_1.set(true);
        wire_2.set(true);
        key.conduct();
        assert_eq!(wire_out.get(), true);
        wire_1.set(false);
        key.conduct();
        assert_eq!(wire_out.get(), true);
        wire_2.set(false);
        key.conduct();
        assert_eq!(wire_out.get(), false);
    }

    #[test]
    pub fn test_not() {
        let in_1 = Wire::new();
        let out = Wire::new();
        let key = Gate::not(in_1.clone(), out.clone());
        assert_eq!(out.get(), true);
        in_1.set(true);
        key.conduct();
        assert_eq!(out.get(), false);
        in_1.set(false);
        key.conduct();
        assert_eq!(out.get(), true);
    }
}
