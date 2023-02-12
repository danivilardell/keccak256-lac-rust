use lac::utils::*;
use lac::or::*;
use lac::xor::*;

#[test]
fn test_multiplication_gate() {
    let mut lac: LAC<i64> = LAC::new();
    let mut basic_layer: BasicLayer<i64> = BasicLayer::new();
    let mut value1: Value<i64> = Value::new();
    let mut value2: Value<i64> = Value::new();

    value1.set_id(0);
    value1.set_value(10);
    value2.set_id(1);
    value2.set_value(14);

    basic_layer.append_value(value1);
    basic_layer.append_value(value2);
    lac.set_basic_layer(basic_layer);

    let mut layer: Layer<i64> = Layer::new();
    let mut gate: Gate<i64> = Gate::new_mult_gate();
    gate.set_degree(1);
    gate.set_id(2);
    gate.set_input_id([0, 1]);

    layer.append_gate(gate);
    lac.append_layer(layer);

    lac.set_output_gate_id(2);

    let result = lac.evaluate();
    assert_eq!(result, 140);
}

#[test]
fn test_addition_gate() {
    let mut lac: LAC<i64> = LAC::new();
    let mut basic_layer: BasicLayer<i64> = BasicLayer::new();
    let mut value1: Value<i64> = Value::new();
    let mut value2: Value<i64> = Value::new();

    value1.set_id(0);
    value1.set_value(10);
    value2.set_id(1);
    value2.set_value(14);

    basic_layer.append_value(value1);
    basic_layer.append_value(value2);
    lac.set_basic_layer(basic_layer);

    let mut layer: Layer<i64> = Layer::new();
    let mut gate: Gate<i64> = Gate::new_add_gate();
    gate.set_degree(1);
    gate.set_id(2);
    gate.set_input_id([0, 1]);

    layer.append_gate(gate);
    lac.append_layer(layer);

    lac.set_output_gate_id(2);

    let result = lac.evaluate();
    assert_eq!(result, 24);
}

#[test]
#[allow(non_snake_case)]
fn test_R1CS_gate() {
    let mut lac: LAC<i64> = LAC::new();
    let mut basic_layer: BasicLayer<i64> = BasicLayer::new();
    let mut value1: Value<i64> = Value::new();
    let mut value2: Value<i64> = Value::new();
    let mut value3: Value<i64> = Value::new();
    let mut value4: Value<i64> = Value::new();

    value1.set_id(0);
    value1.set_value(10);
    value2.set_id(1);
    value2.set_value(14);
    value3.set_id(2);
    value3.set_value(100);
    value4.set_id(3);
    value4.set_value(2);

    basic_layer.append_value(value1);
    basic_layer.append_value(value2);
    basic_layer.append_value(value3);
    basic_layer.append_value(value4);
    lac.set_basic_layer(basic_layer.clone());

    let mut layer: Layer<i64> = Layer::new();
    let mut gate: Gate<i64> = Gate::new_R1CS_gate();
    gate.set_degree(1);
    gate.set_id(4);
    gate.set_R1CS_weights([vec![1, 3], vec![2, 4]]);
    gate.set_input_id_R1CS([vec![0, 1], vec![2, 3]]);

    layer.append_gate(gate);
    lac.append_layer(layer);

    lac.set_output_gate_id(4);

    let result = lac.evaluate();
    assert_eq!(result, 10816);
}

#[test]
fn test_or_lac_circuit() {
    let mut lac = get_or_lac_circuit(0, 0);
    let result = lac.evaluate();
    assert_eq!(result, 0);

    let mut lac = get_or_lac_circuit(1, 0);
    let result = lac.evaluate();
    assert_eq!(result, 1);

    let mut lac = get_or_lac_circuit(0, 1);
    let result = lac.evaluate();
    assert_eq!(result, 1);

    let mut lac = get_or_lac_circuit(1, 1);
    let result = lac.evaluate();
    assert_eq!(result, 1);
}

#[test]
fn test_xor_lac_circuit() {
    let mut lac = get_xor_lac_circuit(0, 0);
    let result = lac.evaluate();
    assert_eq!(result, 0);

    let mut lac = get_xor_lac_circuit(1, 0);
    let result = lac.evaluate();
    assert_eq!(result, 1);

    let mut lac = get_xor_lac_circuit(0, 1);
    let result = lac.evaluate();
    assert_eq!(result, 1);

    let mut lac = get_xor_lac_circuit(1, 1);
    let result = lac.evaluate();
    assert_eq!(result, 0);
}