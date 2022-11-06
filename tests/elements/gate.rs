use binarii::elements::gate::Gate;
use binarii::elements::wire::Wire;
use binarii::elements::Conduct;

#[test]
pub fn test_and() {
    let wire_1 = Wire::new();
    let wire_2 = Wire::new();
    let wire_out = Wire::new();
    let key = Gate::and(wire_1.clone(), wire_2.clone(), wire_out.clone());
    assert_eq!(wire_out.get(), false);
    wire_1.set(true);
    wire_2.set(true);
    assert_eq!(wire_out.get(), false);
    key.conduct();
    assert_eq!(wire_out.get(), true);
    wire_1.set(false);
    wire_2.set(true);
    key.conduct();
    assert_eq!(wire_out.get(), false);
}

#[test]
pub fn test_or() {
    let wire_1 = Wire::new();
    let wire_2 = Wire::new();
    let wire_out = Wire::new();
    let key = Gate::or(wire_1.clone(), wire_2.clone(), wire_out.clone());
    assert_eq!(wire_out.get(), false);
    wire_1.set(true);
    wire_2.set(true);
    key.conduct();
    assert_eq!(wire_out.get(), true);
    wire_1.set(false);
    key.conduct();
    assert_eq!(wire_out.get(), true);
    wire_2.set(false);
    key.conduct();
    assert_eq!(wire_out.get(), false);
}

#[test]
pub fn test_not() {
    let in_1 = Wire::new();
    let out = Wire::new();
    let key = Gate::not(in_1.clone(), out.clone());
    assert_eq!(out.get(), true);
    in_1.set(true);
    key.conduct();
    assert_eq!(out.get(), false);
    in_1.set(false);
    key.conduct();
    assert_eq!(out.get(), true);
}
