use crate::elements::gate::Gate;
use crate::elements::wire::Wire;
use crate::elements::Conduct;
use bevy::utils::HashMap;
use std::fmt::{Debug, Display};
use std::mem;

pub struct Complex {
    input: Vec<Wire>,
    output: Vec<Wire>,
    gates: Vec<Element>,
    tp: &'static str,
    iters_per_tick: usize,
}

pub enum Element {
    Gate(Gate),
    Complex(Complex),
}

impl Element {
    pub fn input(&self) -> Vec<Wire> {
        match self {
            Element::Gate(gate) => vec![gate.get_in_1(), gate.get_in_2()],
            Element::Complex(complex) => complex.input.clone(),
        }
    }

    pub fn output(&self) -> Vec<Wire> {
        match self {
            Element::Gate(gate) => vec![gate.get_out()],
            Element::Complex(complex) => complex.output.clone(),
        }
    }
}

impl Conduct for Element {
    fn conduct(&self) {
        match self {
            Element::Gate(el) => el.conduct(),
            Element::Complex(el) => el.conduct(),
        }
    }
}

impl Complex {
    pub fn new(tp: &'static str) -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
            gates: Vec::new(),
            tp,
            iters_per_tick: 1,
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
        self.gates.push(Element::Gate(key));
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

    pub fn set_in(&mut self, wire_id: usize, wire: Wire) {
        for gate in self.gates.iter_mut() {
            match gate {
                Element::Gate(gate) => {
                    if gate.get_in_1().id() == self.input[wire_id].id() {
                        gate.wire_in_1(wire.clone());
                    }
                    if gate.get_in_2().id() == self.input[wire_id].id() {
                        gate.wire_in_2(wire.clone());
                    }
                }
                Element::Complex(complex) => {
                    let idx = complex
                        .input
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, w)| w.id() == self.input[wire_id].id())
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    for id in idx {
                        complex.set_in(id, wire.clone());
                    }
                }
            }
        }

        self.input[wire_id] = wire;
        self.compile();
    }

    pub fn get_out(&self, wire_id: usize) -> Wire {
        self.output[wire_id].clone()
    }

    pub fn set_out(&mut self, wire_id: usize, wire: Wire) {
        for gate in self.gates.iter_mut() {
            match gate {
                Element::Gate(gate) => {
                    if gate.get_out().id() == self.output[wire_id].id() {
                        gate.wire_out(wire.clone());
                    }
                }
                Element::Complex(complex) => {
                    let idx = complex
                        .output
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, w)| w.id() == self.output[wire_id].id())
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();

                    for id in idx {
                        complex.set_out(id, wire.clone());
                    }
                }
            }
        }

        self.output[wire_id] = wire;
        self.compile();
    }

    pub fn compile(&mut self) {
        self.iters_per_tick = 1;
        let mut gates = Vec::new();
        let mut mapping = mem::take(&mut self.gates)
            .into_iter()
            .enumerate()
            .collect::<HashMap<usize, Element>>();

        loop {
            let key = if let Some(key) = mapping.keys().next().cloned() {
                key
            } else {
                break;
            };
            self.map(&mut gates, &mut mapping, key);
        }
        self.gates = gates;
    }

    fn map(
        &mut self,
        gates: &mut Vec<Element>,
        elements: &mut HashMap<usize, Element>,
        key: usize,
    ) {
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
            } else {
                self.iters_per_tick += 1;
            }
        }

        gates.push(element);
    }
}

impl Conduct for Complex {
    fn conduct(&self) {
        for _ in 0..self.iters_per_tick {
            for gate in self.gates.iter() {
                gate.conduct();
            }
        }
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = |w: &Wire| {
            if w.get() {
                "1"
            } else {
                "0"
            }
        };

        writeln!(f, "[")?;
        for gate in &self.gates {
            writeln!(f, "{}", gate)?;
        }
        writeln!(f, "]")?;

        for i in &self.input {
            writeln!(f, "IN:{:#x}", i.id())?;
        }
        for i in &self.output {
            writeln!(f, "OUT:{:#x}", i.id())?;
        }
        writeln!(f, "ITERS:{}", self.iters_per_tick)?;
        write!(f, "{}(", self.tp)?;
        for i in &self.input {
            write!(f, "{},", map(i))?;
        }
        write!(f, ")->(")?;
        for i in &self.output {
            write!(f, "{},", map(i))?;
        }
        writeln!(f, ")")
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = |w: &Wire| {
            if w.get() {
                "1"
            } else {
                "0"
            }
        };

        write!(f, "{}(", self.tp)?;
        for i in &self.input {
            write!(f, "{},", map(i))?;
        }
        write!(f, ")->(")?;
        for i in &self.output {
            write!(f, "{},", map(i))?;
        }
        writeln!(f, ")")
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Gate(gate) => write!(f, "{}", gate),
            Element::Complex(complex) => write!(f, "{}", complex),
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::complex::Complex;
    use crate::elements::gate::Gate;
    use crate::elements::wire::Wire;
    use crate::elements::Conduct;

    pub fn and() -> Complex {
        let mut complex = Complex::new("and");
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
        let mut complex = Complex::new("or");
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
        let mut complex = Complex::new("not");
        let wire_1 = Wire::new();
        let wire_out = Wire::new();
        let key = Gate::not(wire_1.clone(), wire_out.clone());
        complex.add_input(wire_1);
        complex.add_output(wire_out);
        complex.add_key(key);
        complex
    }

    pub fn rs_trigger() -> Complex {
        let mut complex = Complex::new("rs_trigger");
        let wire_r = Wire::new();
        let wire_s = Wire::new();
        let wire_q = Wire::new();
        let wire_qn = Wire::new();

        complex.add_input(wire_s.clone());
        complex.add_input(wire_r.clone());
        complex.add_output(wire_q.clone());
        complex.add_output(wire_qn.clone());

        complex.add_key(Gate::nor(wire_r.clone(), wire_qn.clone(), wire_q.clone()));
        complex.add_key(Gate::nor(wire_s.clone(), wire_q.clone(), wire_qn.clone()));

        complex
    }

    #[test]
    pub fn test_rs_trigger() {
        let rs = rs_trigger();

        let set = rs.get_in(0);
        let reset = rs.get_in(1);
        let q = rs.get_out(0);
        let qn = rs.get_out(1);
        reset.set(true);
        rs.conduct();
        assert_eq!(q.get(), false);
        assert_eq!(qn.get(), true);

        reset.set(false);
        set.set(true);
        rs.conduct();
        assert_eq!(q.get(), true);
        assert_eq!(qn.get(), false);

        set.set(false);
        rs.conduct();
        assert_eq!(q.get(), true);
        assert_eq!(qn.get(), false);

        reset.set(true);
        rs.conduct();
        assert_eq!(q.get(), false);
        assert_eq!(qn.get(), true);

        reset.set(false);
        rs.conduct();
        assert_eq!(q.get(), false);
        assert_eq!(qn.get(), true);
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

        let mut and_3 = Complex::new("and_3");
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

    pub fn half_sum() -> Complex {
        let mut complex = Complex::new("half_sum");
        let wire_a = Wire::new();
        let wire_b = Wire::new();
        let wire_s = Wire::new();
        let wire_c = Wire::new();

        complex.add_input(wire_a.clone());
        complex.add_input(wire_b.clone());
        complex.add_output(wire_s.clone());
        complex.add_output(wire_c.clone());

        complex.add_key(Gate::xor(wire_a.clone(), wire_b.clone(), wire_s.clone()));
        complex.add_key(Gate::and(wire_a.clone(), wire_b.clone(), wire_c.clone()));

        complex
    }

    pub fn sum() -> Complex {
        let mut complex = Complex::new("sum");
        let wire_a = Wire::new();
        let wire_b = Wire::new();
        let wire_p = Wire::new();

        complex.add_input(wire_a.clone());
        complex.add_input(wire_b.clone());
        complex.add_input(wire_p.clone());

        let wire_s = Wire::new();
        let wire_c = Wire::new();
        complex.add_output(wire_s.clone());
        complex.add_output(wire_c.clone());

        let mut half_sum_1 = half_sum();
        half_sum_1.set_in(0, wire_a.clone());
        half_sum_1.set_in(1, wire_b.clone());
        let wire_s_1 = half_sum_1.get_out(0);
        let wire_c_1 = half_sum_1.get_out(1);

        let mut half_sum_2 = half_sum();
        half_sum_2.set_in(0, wire_s_1.clone());
        half_sum_2.set_in(1, wire_p.clone());

        half_sum_2.set_out(0, wire_s.clone());

        let wire_c_2 = half_sum_2.get_out(1);

        complex.add_key(Gate::or(wire_c_1, wire_c_2, wire_c.clone()));
        complex.add_complex(half_sum_1);
        complex.add_complex(half_sum_2);
        complex
    }

    #[test]
    pub fn test_half_sum() {
        let complex = half_sum();
        complex.conduct();
        let wire_a = complex.get_in(0);
        let wire_b = complex.get_in(1);

        let wire_s = complex.get_out(0);
        let wire_c = complex.get_out(1);
        assert_eq!(wire_s.get(), false);
        assert_eq!(wire_c.get(), false);
        complex.conduct();

        wire_a.set(true);
        wire_b.set(false);
        complex.conduct();
        assert_eq!(wire_s.get(), true);
        assert_eq!(wire_c.get(), false);

        wire_b.set(true);
        complex.conduct();
        assert_eq!(wire_s.get(), false);
        assert_eq!(wire_c.get(), true);
    }

    #[test]
    pub fn test_sum() {
        let sum = sum();
        let wire_a = sum.get_in(0);
        let wire_b = sum.get_in(1);
        let wire_p = sum.get_in(2);

        let wire_s = sum.get_out(0);
        let wire_c = sum.get_out(1);
        sum.conduct();
        assert_eq!(wire_s.get(), false);
        assert_eq!(wire_c.get(), false);

        wire_a.set(true);
        sum.conduct();
        assert_eq!(wire_s.get(), true);
        assert_eq!(wire_c.get(), false);

        wire_b.set(true);
        sum.conduct();
        assert_eq!(wire_s.get(), false);
        assert_eq!(wire_c.get(), true);

        wire_p.set(true);
        sum.conduct();
        assert_eq!(wire_s.get(), true);
        assert_eq!(wire_c.get(), true);
    }
}
