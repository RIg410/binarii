use crate::elements::wire::Wire;
use crate::elements::Conduct;

#[derive(Clone)]
pub struct Key {
    in_1: Wire,
    in_2: Wire,
    out: Wire,
}

impl Key {
    pub fn new(in_1: Wire, in_2: Wire, out: Wire) -> Self {
        Self { in_1, in_2, out }
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

    pub fn get_out(&self) -> Wire {
        self.out.clone()
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::new(Wire::default(), Wire::default(), Wire::default())
    }
}

impl Conduct for Key {
    #[inline]
    fn conduct(&self) {
        let in_1 = self.in_1.get();
        let in_2 = self.in_2.get();
        let out = in_1 && in_2;
        self.out.set(out);
    }
}

#[cfg(test)]
mod test {
    use crate::elements::complex::Complex;
    use crate::elements::key::Key;
    use crate::elements::wire::Wire;
    use crate::elements::Conduct;

    #[test]
    pub fn test_and() {
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_out = Wire::new();
        let mut key = Key::new(wire_1.clone(), wire_2.clone(), wire_out.clone());
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
    pub fn test_and_3() {
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_3 = Wire::new();
        let wire_out = Wire::new();

        let mut and_3 = Complex::new();
        and_3.add_input(wire_1.clone());
        and_3.add_input(wire_2.clone());
        and_3.add_input(wire_3.clone());

        and_3.add_output(wire_out.clone());

        let inner = Wire::new();
        and_3.add_key(Key::new(and_3.get_in(0), and_3.get_in(1), inner.clone()));
        and_3.add_key(Key::new(inner, and_3.get_in(2), and_3.get_out(0)));

        assert_eq!(and_3.get_out(0).get(), false);

        // and_3.get_in(0).set(true);
    }
}
