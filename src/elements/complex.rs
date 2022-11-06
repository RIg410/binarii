use crate::elements::bus::Bus;
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

    pub fn add_input_bus(&mut self, bus: Bus) {
        for wire in bus.wires() {
            self.add_input(wire.clone());
        }
    }

    pub fn add_output(&mut self, wire: Wire) -> usize {
        let id = self.output.len();
        self.output.push(wire);
        id
    }

    pub fn add_output_bus(&mut self, bus: Bus) {
        for wire in bus.wires() {
            self.add_output(wire.clone());
        }
    }

    pub fn add_gate(&mut self, key: Gate) -> usize {
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

    pub fn get_in_bus(&self, offset: usize, len: usize) -> Bus {
        Bus::with_wires(self.input[offset..offset + len].to_vec())
    }

    pub fn get_out_bus(&self, offset: usize, len: usize) -> Bus {
        Bus::with_wires(self.output[offset..offset + len].to_vec())
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

        while let Some(key) = mapping.keys().next().cloned() {
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
        writeln!(f, "[")?;
        for gate in &self.gates {
            writeln!(f, "{:?}", gate)?;
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
            write!(f, "{},", i)?;
        }
        write!(f, ")->(")?;
        for i in &self.output {
            write!(f, "{},", i)?;
        }
        writeln!(f, ")")
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.tp)?;
        for i in &self.input {
            write!(f, "{},", i)?;
        }
        write!(f, ")->(")?;
        for i in &self.output {
            write!(f, "{},", i)?;
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
