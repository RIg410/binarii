use binarii::elements::bus::{Bus, BusAccess};
use binarii::elements::complex::Complex;
use binarii::elements::gate::Gate;
use binarii::elements::wire::Wire;
use binarii::elements::Conduct;

pub fn and() -> Complex {
    let mut complex = Complex::new("and");
    let wire_1 = Wire::new();
    let wire_2 = Wire::new();
    let wire_out = Wire::new();
    let key = Gate::and(wire_1.clone(), wire_2.clone(), wire_out.clone());
    complex.add_input(wire_1);
    complex.add_input(wire_2);
    complex.add_output(wire_out);
    complex.add_gate(key);
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
    complex.add_gate(key);
    complex
}

pub fn not() -> Complex {
    let mut complex = Complex::new("not");
    let wire_1 = Wire::new();
    let wire_out = Wire::new();
    let key = Gate::not(wire_1.clone(), wire_out.clone());
    complex.add_input(wire_1);
    complex.add_output(wire_out);
    complex.add_gate(key);
    complex
}

pub fn rs_flip_flop() -> Complex {
    let mut complex = Complex::new("rs_flip_flop");
    let wire_r = Wire::new();
    let wire_s = Wire::new();
    let wire_q = Wire::new();
    let wire_qn = Wire::new();

    complex.add_input(wire_s.clone());
    complex.add_input(wire_r.clone());
    complex.add_output(wire_q.clone());
    complex.add_output(wire_qn.clone());

    complex.add_gate(Gate::nor(wire_r.clone(), wire_qn.clone(), wire_q.clone()));
    complex.add_gate(Gate::nor(wire_s.clone(), wire_q.clone(), wire_qn.clone()));

    complex
}

#[test]
pub fn test_rs_flip_flop() {
    let rs = rs_flip_flop();

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
    and_3.add_gate(Gate::and(and_3.get_in(0), and_3.get_in(1), inner.clone()));
    and_3.add_gate(Gate::and(inner, and_3.get_in(2), and_3.get_out(0)));
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

    complex.add_gate(Gate::xor(wire_a.clone(), wire_b.clone(), wire_s.clone()));
    complex.add_gate(Gate::and(wire_a.clone(), wire_b.clone(), wire_c.clone()));

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

    complex.add_gate(Gate::or(wire_c_1, wire_c_2, wire_c.clone()));
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

pub fn byte_sum() -> Complex {
    let mut sum_block = Complex::new("byte_sum");
    let a = Bus::new(8);
    let b = Bus::new(8);
    let res = Bus::new(9);

    let mut carry = Wire::new();
    sum_block.add_input_bus(a.clone());
    sum_block.add_input_bus(b.clone());
    sum_block.add_input(carry.clone());
    sum_block.add_output_bus(res.clone());

    for i in (0..8).rev() {
        let mut bit = sum();
        bit.set_in(0, a.get_wire(i));
        bit.set_in(1, b.get_wire(i));
        bit.set_in(2, carry);
        bit.set_out(0, res.get_wire(i));
        carry = bit.get_out(1);
        sum_block.add_complex(bit);
    }

    sum_block.set_out(8, carry.clone());
    sum_block.conduct();
    sum_block
}

#[test]
pub fn test_byte_sum() {
    let sum = byte_sum();
    let a = sum.get_in_bus(0, 8);
    let b = sum.get_in_bus(8, 8);
    let carry_in = sum.get_in(16);
    let res = sum.get_out_bus(0, 8);
    let carry_out = sum.get_out(8);

    for i in 0..255u8 {
        for j in 0..255u8 {
            a.set(0, i);
            b.set(0, j);
            carry_in.set(false);
            sum.conduct();
            let res: u8 = res.get(0);
            let carry = carry_out.get();
            let (expected_sum, expected_carry) = i.overflowing_add(j);
            assert_eq!(res, expected_sum);
            assert_eq!(carry, expected_carry);
        }
    }
}

pub fn d_flip_flop() -> Complex {
    let mut flip_flop = Complex::new("d_flip_flop");
    let d = Wire::new();
    let clk = Wire::new();

    flip_flop.add_input(d.clone());
    flip_flop.add_input(clk.clone());

    let reset = Wire::new();
    flip_flop.add_gate(Gate::and(clk.clone(), d.clone(), reset.clone()));

    let not_data = Wire::new();
    flip_flop.add_gate(Gate::not(d.clone(), not_data.clone()));

    let set = Wire::new();
    flip_flop.add_gate(Gate::and(clk.clone(), not_data.clone(), set.clone()));

    let mut rs = rs_flip_flop();
    rs.set_in(1, set.clone());
    rs.set_in(0, reset.clone());

    let q = rs.get_out(0);
    let nq = rs.get_out(1);
    flip_flop.add_output(q.clone());
    flip_flop.add_output(nq.clone());
    flip_flop.add_complex(rs);
    flip_flop
}

#[test]
pub fn test_d_flip_flop() {
    let flip_flop = d_flip_flop();
    let d = flip_flop.get_in(0);
    let clk = flip_flop.get_in(1);
    let q = flip_flop.get_out(0);
    let nq = flip_flop.get_out(1);
    clk.set(true);
    flip_flop.conduct();
    clk.set(false);
    flip_flop.conduct();
    println!("{}", flip_flop);
    assert_eq!(q.get(), false);
    assert_eq!(nq.get(), true);

    d.set(true);
    flip_flop.conduct();
    assert_eq!(q.get(), false);
    assert_eq!(nq.get(), true);

    d.set(false);
    flip_flop.conduct();
    assert_eq!(q.get(), false);
    assert_eq!(nq.get(), true);
    d.set(true);
    clk.set(true);
    flip_flop.conduct();
    assert_eq!(q.get(), true);
    assert_eq!(nq.get(), false);
    clk.set(false);
    d.set(false);
    flip_flop.conduct();
    assert_eq!(q.get(), true);
    assert_eq!(nq.get(), false);
    clk.set(true);
    flip_flop.conduct();
    assert_eq!(q.get(), false);
    assert_eq!(nq.get(), true);
}

pub fn byte_flip_flop() -> Complex {
    let mut flip_flop = Complex::new("byte_flip_flop");

    let d_in = Bus::new(8);
    let clk = Wire::new();

    let mut q_out = Bus::new(8);
    let clk_out = clk.clone();

    for i in 0..8 {
        let mut dff = d_flip_flop();
        dff.set_in(0, d_in.get_wire(i));
        dff.set_in(1, clk.clone());
        q_out.set_wire(i, dff.get_out(0));
        flip_flop.add_complex(dff);
    }
    flip_flop.add_input_bus(d_in);
    flip_flop.add_input(clk);
    flip_flop.add_output_bus(q_out);
    flip_flop.add_output(clk_out);

    flip_flop
}

#[test]
pub fn test_byte_flip_flop() {
    let flip_flop = byte_flip_flop();
    let d_in = flip_flop.get_in_bus(0, 8);
    let clk = flip_flop.get_in(8);

    let q_out = flip_flop.get_out_bus(0, 8);
    let clk_out = flip_flop.get_out(8);
    clk.set(true);
    flip_flop.conduct();
    clk.set(false);
    flip_flop.conduct();

    let data: u8 = q_out.get(0);
    assert_eq!(data, 0);
    d_in.set(0, 0b11111111);
    flip_flop.conduct();
    let data: u8 = q_out.get(0);
    assert_eq!(data, 0);

    clk.set(true);
    flip_flop.conduct();
    let data: u8 = q_out.get(0);
    assert_eq!(data, 0b11111111);

    clk.set(false);
    flip_flop.conduct();
    let data: u8 = q_out.get(0);
    assert_eq!(data, 0b11111111);

    d_in.set(0, 0b10101010);
    flip_flop.conduct();
    let data: u8 = q_out.get(0);
    assert_eq!(data, 0b11111111);

    clk.set(true);
    flip_flop.conduct();
    clk.set(false);
    flip_flop.conduct();
    let data: u8 = q_out.get(0);
    assert_eq!(data, 0b10101010);
}
