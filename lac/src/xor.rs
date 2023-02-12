use crate::utils::*;

//OR gate implemented using XOR(x0,x1) = x0+x1-2*x0*x1
//Using the following layered arithmetic circuit:
//  layer0:    g_0=0       g_1=1       g_2=x0        g_3=x1
//  layer1:       g_4=g_0+g_1   g_5=g_2+g_3    g_6=g_2*g_3
//  layer2:             g_7=(g_5-2*g_6)*(g_4)


pub fn get_xor_lac_circuit(x0: i64, x1: i64) -> LAC<i64> {
    let mut lac = LAC::new();

    lac.set_basic_layer(get_xor_basic_layer(x0, x1));

    let layer1 = get_xor_first_layer();
    let layer2 = get_xor_second_layer();
    let layers = vec![layer1, layer2];
    lac.append_layers(layers);

    lac.set_output_gate_id(7);

    lac
}

fn get_xor_basic_layer(x0: i64, x1: i64) -> BasicLayer<i64> {
    let mut basic_layer = BasicLayer::new();

    let mut value0: Value<i64> = Value::new();
    let mut value1: Value<i64> = Value::new();
    let mut value2: Value<i64> = Value::new();
    let mut value3: Value<i64> = Value::new();

    value0.set_all(0, 0);
    value1.set_all(1, 1);
    value2.set_all(2, x0);
    value3.set_all(3, x1);

    let values = vec![value0, value1, value2, value3];
    basic_layer.append_values(values);

    basic_layer
}

fn get_xor_first_layer() -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();

    let mut gate4: Gate<i64> = Gate::new_add_gate();
    let mut gate5: Gate<i64> = Gate::new_add_gate();
    let mut gate6: Gate<i64> = Gate::new_mult_gate();

    gate4.set_all(Some(1), Some(4), Some([0, 1]), None, None);
    gate5.set_all(Some(1), Some(5), Some([2, 3]), None, None);
    gate6.set_all(Some(1), Some(6), Some([2, 3]), None, None);

    layer.append_gates(vec![gate4, gate5, gate6]);
    layer.set_degree(1);

    layer
}

#[allow(non_snake_case)]
fn get_xor_second_layer() -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();

    let mut gate7: Gate<i64> = Gate::new_R1CS_gate();

    let input_id_R1CS = Some([vec![5, 6], vec![4]]);
    let weights_R1CS = Some([vec![1, -2], vec![1]]);
    gate7.set_all(Some(2), Some(7), None, input_id_R1CS, weights_R1CS);

    layer.append_gates(vec![gate7]);
    layer.set_degree(2);

    layer
}
