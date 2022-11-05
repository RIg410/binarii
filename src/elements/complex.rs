use crate::elements::key::Key;
use crate::elements::wire::Wire;
use crate::elements::Conduct;

pub struct Complex {
    input: Vec<Wire>,
    output: Vec<Wire>,
    keys: Vec<Key>,
}

impl Complex {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
            keys: Vec::new(),
        }
    }

    pub fn add_input(&mut self, wire: Wire) -> usize {
        let id = self.input.len();
        self.input.push(wire);
        id
    }

    pub fn add_output(&mut self, wire: Wire) -> usize {
        let id = self.output.len();
        self.output.push(wire);
        id
    }

    pub fn add_key(&mut self, key: Key) -> usize {
        let id = self.keys.len();
        self.keys.push(key);
        id
    }

    pub fn get_key(&mut self, id: usize) -> &mut Key {
        &mut self.keys[id]
    }

    pub fn get_in(&self, wire_id: usize) -> Wire {
        self.input[wire_id].clone()
    }

    pub fn get_out(&self, wire_id: usize) -> Wire {
        self.output[wire_id].clone()
    }
}

impl Conduct for Complex {
    fn conduct(&self) {
        for key in &self.keys {
            key.conduct();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::complex::Complex;
    use crate::elements::key::Key;
    use crate::elements::wire::Wire;
    use crate::elements::Conduct;

    #[test]
    pub fn test_and_2() {
        let mut and_2 = Complex::new();
        let in_1 = Wire::new();
        let in_2 = Wire::new();
        let out = Wire::new();
        let in_1_idx = and_2.add_input(in_1.clone());
        let in_2_idx = and_2.add_input(in_2.clone());
        let out_idx = and_2.add_output(out.clone());
        let key = Key::new(
            and_2.get_in(in_1_idx),
            and_2.get_in(in_2_idx),
            and_2.get_out(out_idx),
        );
        and_2.add_key(key);

        assert_eq!(and_2.get_out(out_idx).get(), false);

        in_1.set(true);
        and_2.conduct();
        assert_eq!(and_2.get_out(out_idx).get(), false);
        in_2.set(true);
        and_2.conduct();
        assert_eq!(and_2.get_out(out_idx).get(), true);

        in_1.set(false);
        and_2.conduct();
        assert_eq!(and_2.get_out(out_idx).get(), false);
    }
}
