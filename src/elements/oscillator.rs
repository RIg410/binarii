use crate::elements::wire::Wire;
use crate::elements::Conduct;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Oscillator {
    out: Wire,
    half_period: u32,
    counter: Rc<Cell<u32>>,
}

impl Oscillator {
    pub fn new(out: Wire, mut half_period: usize) -> Self {
        if half_period == 0 {
            half_period = 1;
        }

        out.set(false);
        Self {
            out,
            half_period: half_period as u32,
            counter: Rc::new(Cell::new(0)),
        }
    }

    pub fn get_out(&self) -> Wire {
        self.out.clone()
    }

    pub fn wire_out(&mut self, wire: Wire) {
        self.out = wire;
    }
}

impl Conduct for Oscillator {
    fn conduct(&self) {
        let count = self.counter.get();
        self.counter.set(count + 1);
        if count == self.half_period - 1 {
            self.counter.set(0);
            self.out.set(!self.out.get());
        }
    }
}
