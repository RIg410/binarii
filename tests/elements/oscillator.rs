use binarii::elements::oscillator::Oscillator;
use binarii::elements::wire::Wire;
use binarii::elements::Conduct;

#[test]
pub fn test_oscillator() {
    let wire_out = Wire::new();
    let osc = Oscillator::new(wire_out.clone(), 2);
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
}

#[test]
pub fn test_oscillator_1() {
    let wire_out = Wire::new();
    let osc = Oscillator::new(wire_out.clone(), 1);
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
    osc.conduct();
    assert_eq!(wire_out.get(), true);
    osc.conduct();
    assert_eq!(wire_out.get(), false);
}
