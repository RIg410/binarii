pub mod bus;
pub mod complex;
pub mod gate;
pub mod oscillator;
pub mod wire;

pub trait Conduct {
    fn conduct(&self);
}
