pub mod bus;
pub mod complex;
pub mod key;
pub mod wire;

pub trait Conduct {
    fn conduct(&self);
}
