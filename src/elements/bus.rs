use crate::elements::wire::Wire;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Bus {
    wires: Vec<Wire>,
}

impl Bus {
    pub fn new(wires: usize) -> Self {
        Self {
            wires: (0..wires).map(|_| Wire::new()).collect(),
        }
    }

    pub fn with_wires(wires: Vec<Wire>) -> Self {
        Self { wires }
    }

    pub fn get_wire(&self, index: usize) -> Wire {
        self.wires[index].clone()
    }

    pub fn set_wire(&mut self, index: usize, wire: Wire) {
        self.wires[index] = wire;
    }

    pub fn remove_wire(&mut self, index: usize) -> Wire {
        self.wires.remove(index)
    }

    pub fn add_wire(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    pub fn wires(&self) -> &[Wire] {
        &self.wires
    }

    pub fn size(&self) -> usize {
        self.wires.len()
    }
}

pub trait BusAccess<T> {
    fn set(&self, offset: usize, value: T);
    fn get(&self, offset: usize) -> T;
}

impl BusAccess<bool> for Bus {
    fn set(&self, offset: usize, value: bool) {
        self.wires[offset].set(value);
    }

    fn get(&self, offset: usize) -> bool {
        self.wires[offset].get()
    }
}

impl BusAccess<u8> for Bus {
    fn set(&self, offset: usize, value: u8) {
        for i in 0..8 {
            let offset = offset + i;
            if offset < self.wires.len() {
                self.wires[offset].set((value >> (7 - i) & 1) == 1);
            }
        }
    }

    fn get(&self, offset: usize) -> u8 {
        let mut value = 0;
        for i in 0..8 {
            let offset = offset + i;
            if offset < self.wires.len() {
                value <<= 1;
                if self.wires[offset].get() {
                    value |= 1;
                }
            }
        }
        value
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for wire in &self.wires {
            write!(f, "{}", wire)?;
        }
        Ok(())
    }
}
