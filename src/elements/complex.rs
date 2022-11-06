use crate::elements::gate::Gate;
use crate::elements::wire::Wire;
use crate::elements::Conduct;
use bevy::utils::HashMap;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::mem;

pub struct Complex {
    input: Vec<Wire>,
    output: Vec<Wire>,
    gates: Vec<Element>,
}

pub enum Element {
    Key(Gate),
    Complex(Complex),
}

impl Element {
    pub fn input(&self) -> Vec<Wire> {
        match self {
            Element::Key(gate) => vec![gate.get_in_1(), gate.get_in_2()],
            Element::Complex(complex) => complex.input.clone(),
        }
    }

    pub fn output(&self) -> Vec<Wire> {
        match self {
            Element::Key(gate) => vec![gate.get_out()],
            Element::Complex(complex) => complex.output.clone(),
        }
    }
}

impl Conduct for Element {
    fn conduct(&self) {
        match self {
            Element::Key(el) => el.conduct(),
            Element::Complex(el) => el.conduct(),
        }
    }
}

impl Complex {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
            gates: Vec::new(),
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

    pub fn add_key(&mut self, key: Gate) -> usize {
        let id = self.gates.len();
        self.gates.push(Element::Key(key));
        self.compile();
        id
    }

    pub fn add_complex(&mut self, complex: Complex) -> usize {
        let id = self.gates.len();
        self.gates.push(Element::Complex(complex));
        self.compile();
        id
    }

    pub fn get_element(&mut self, id: usize) -> &mut Element {
        &mut self.gates[id]
    }

    pub fn get_in(&self, wire_id: usize) -> Wire {
        self.input[wire_id].clone()
    }

    pub fn get_out(&self, wire_id: usize) -> Wire {
        self.output[wire_id].clone()
    }

    pub fn compile(&mut self) {
        let mut gates = Vec::new();
        let mut mapping = mem::take(&mut self.gates)
            .into_iter()
            .enumerate()
            .collect::<BTreeMap<usize, Element>>();

        loop {
            let gate = if let Some(key) = mapping.keys().next().cloned() {
                mapping.remove(&key).unwrap()
            } else {
                break;
            };

            gates.push(gate);
        }
        self.gates = gates;
    }

    fn map(&self, gates: &mut Vec<Element>, elements: &mut HashMap<usize, Element>, key: usize) {
        let element = if let Some(element) = elements.remove(&key) {
            element
        } else {
            return;
        };

        let input = element.input();

        for wire in input.iter() {
            if self.input.iter().any(|w| w.id() == wire.id()) {
                continue;
            }

            let element_to_handle = elements.iter().find_map(|(key, wal)| {
                wal.output().iter().find_map(|w| {
                    if w.id() == wire.id() {
                        Some(*key)
                    } else {
                        None
                    }
                })
            });

            if let Some(key) = element_to_handle {
                self.map(gates, elements, key);
            }
        }

        gates.push(element);
    }
}

impl Conduct for Complex {
    fn conduct(&self) {
        for key in &self.gates {
            key.conduct();
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, i) in self.input.iter().enumerate() {
            let val = if i.get() { "1" } else { "0" };
            writeln!(f, "IN_{}: {}", idx, val)?;
        }

        for (idx, o) in self.output.iter().enumerate() {
            let val = if o.get() { "1" } else { "0" };
            writeln!(f, "OUT_{}: {}", idx, val)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::complex::Complex;
    use crate::elements::gate::Gate;
    use crate::elements::wire::Wire;
    use crate::elements::Conduct;

    pub fn and() -> Complex {
        let mut complex = Complex::new();
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::and(wire_1.clone(), wire_2.clone(), wire_out.clone());
        complex.add_input(wire_1);
        complex.add_input(wire_2);
        complex.add_output(wire_out);
        complex.add_key(key);
        complex
    }

    pub fn or() -> Complex {
        let mut complex = Complex::new();
        let wire_1 = Wire::new();
        let wire_2 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::or(wire_1.clone(), wire_2.clone(), wire_out.clone());
        complex.add_input(wire_1);
        complex.add_input(wire_2);
        complex.add_output(wire_out);
        complex.add_key(key);
        complex
    }

    pub fn not() -> Complex {
        let mut complex = Complex::new();
        let wire_1 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::not(wire_1.clone(), wire_out.clone());
        complex.add_input(wire_1);
        complex.add_output(wire_out);
        complex.add_key(key);
        complex
    }

    pub fn rs_trigger() -> Complex {
        let mut complex = Complex::new();
        let wire_r = Wire::new();
        let wire_s = Wire::new();
        let wire_q = Wire::new();
        let wire_qn = Wire::new();

        complex.add_input(wire_r.clone());
        complex.add_input(wire_s.clone());
        complex.add_output(wire_q.clone());
        complex.add_output(wire_qn.clone());

        complex.add_key(Gate::nor(wire_r.clone(), wire_qn.clone(), wire_q.clone()));
        complex.add_key(Gate::nor(wire_s.clone(), wire_q.clone(), wire_qn.clone()));

        complex
    }

    #[test]
    pub fn test_not() {
        let complex = not();
        let in_1 = complex.get_in(0);
        let out = complex.get_out(0);
        assert_eq!(out.get(), true);
        in_1.set(false);
        complex.conduct();
        assert_eq!(out.get(), true);
    }

    #[test]
    pub fn test_or() {
        let complex = or();
        assert_eq!(complex.get_out(0).get(), false);
        complex.get_in(0).set(true);
        complex.conduct();
        assert_eq!(complex.get_out(0).get(), true);
        complex.get_in(1).set(true);
        complex.conduct();
        assert_eq!(complex.get_out(0).get(), true);

        complex.get_in(0).set(false);
        complex.conduct();
        assert_eq!(complex.get_out(0).get(), true);
        complex.get_in(1).set(false);
        complex.conduct();
        assert_eq!(complex.get_out(0).get(), false);
    }

    #[test]
    pub fn test_and_2() {
        let complex = and();
        let wire_1 = complex.get_in(0);
        let wire_2 = complex.get_in(1);
        let wire_out = complex.get_out(0);
        wire_1.set(true);
        wire_2.set(true);
        complex.conduct();
        assert_eq!(wire_out.get(), true);

        wire_1.set(false);
        wire_2.set(true);
        complex.conduct();
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
        and_3.add_key(Gate::and(and_3.get_in(0), and_3.get_in(1), inner.clone()));
        and_3.add_key(Gate::and(inner, and_3.get_in(2), and_3.get_out(0)));
        and_3.conduct();
        assert_eq!(and_3.get_out(0).get(), false);

        wire_1.set(true);
        and_3.conduct();
        assert_eq!(and_3.get_out(0).get(), false);
        wire_2.set(true);
        and_3.conduct();
        assert_eq!(and_3.get_out(0).get(), false);
        wire_3.set(true);
        and_3.conduct();
        assert_eq!(and_3.get_out(0).get(), true);
        wire_1.set(false);
        and_3.conduct();
        assert_eq!(and_3.get_out(0).get(), false);
    }
}
