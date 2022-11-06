use binarii::elements::bus::{Bus, BusAccess};

#[test]
pub fn test_set_u8() {
    let bus = Bus::new(8);
    let b: u8 = bus.get(0);
    assert_eq!(b, 0);
    bus.set(0, 0b10101010);
    let b: u8 = bus.get(0);
    assert_eq!(b, 0b10101010);
    let val: bool = bus.get(0);
    assert_eq!(val, true);
    let val: bool = bus.get(1);
    assert_eq!(val, false);
    let val: bool = bus.get(2);
    assert_eq!(val, true);
    let val: bool = bus.get(3);
    assert_eq!(val, false);
    let val: bool = bus.get(4);
    assert_eq!(val, true);
    let val: bool = bus.get(5);
    assert_eq!(val, false);
    let val: bool = bus.get(6);
    assert_eq!(val, true);
    let val: bool = bus.get(7);
    assert_eq!(val, false);
}
