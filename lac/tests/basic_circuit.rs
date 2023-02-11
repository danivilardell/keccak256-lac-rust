use lac::utils::*;

#[test]
fn basicCircuit() {
    let mut lac: LAC<i64> = LAC::new();
    let mut basic_layer: BasicLayer<i64> = BasicLayer::new();
    let mut value1: Value<i64> = Value::new();
    let mut value2: Value<i64> = Value::new();

    value1.set_id(0); value1.set_value(10);
    value2.set_id(1); value2.set_value(14);

    basic_layer.append_value(value1);
    basic_layer.append_value(value2);
    lac.set_basic_layer(basic_layer.clone());

    let mut layer: Layer<i64> = Layer::new();
    let mut gate: Gate<i64> = Gate::new_mult_gate();
    gate.set_degree(1);
    gate.set_basic_layer(basic_layer);
    gate.set_id(2);
    gate.set_input_id([0, 1]);
    gate.set_input();

    layer.append_gate(gate);
    lac.append_layer(layer);

    lac.set_output_id(2);

    let result = lac.evaluate();
    println!("{}", result);
}