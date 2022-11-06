use std::cell::Cell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Wire {
    value: Rc<Cell<bool>>,
}

impl Wire {
    pub fn new() -> Self {
        Self {
            value: Rc::new(Cell::new(false)),
        }
    }

    pub fn set(&self, value: bool) {
        self.value.set(value);
    }

    pub fn get(&self) -> bool {
        self.value.get()
    }

    pub fn id(&self) -> usize {
        Rc::as_ptr(&self.value) as *const () as usize
    }
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
    }
}

impl Default for Wire {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.get() {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}
