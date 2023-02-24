use crate::utils::*;

//OR gate implemented using XOR(x0,x1) = x0+x1-2*x0*x1
//Using the following layered arithmetic circuit:
//  layer0:    g_0=x0         g_1=x1
//  layer1:    g_0=g_0+g_1    g_1=g_0*g_1
//  layer2:       g_0=(g_0-2*g_1)*1

pub fn get_xor_lac_circuit(x0: i64, x1: i64) -> LAC<i64> {
    let mut lac = LAC::new();

    lac.set_basic_layer(get_xor_basic_layer(x0, x1));

    let layer1 = get_xor_first_layer(vec![2, 3], vec![2, 3], 1);
    let layer2 = get_xor_second_layer(vec![2, 3], vec![2], 2);
    let layers = vec![layer1, layer2];
    lac.append_layers(layers);

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

fn get_xor_zero_layer(in_ids: Vec<u64>, gate_ids: Vec<u64>, degree: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);
    layer.add_gate_0_and_1(degree);

    let mut gate0: Gate<i64> = Gate::new_add_gate();
    let mut gate1: Gate<i64> = Gate::new_add_gate();

    gate0.set_all(
        Some(degree),
        Some(gate_ids[0]),
        Some([0, in_ids[0]]),
        None,
        None,
    );
    gate1.set_all(
        Some(degree),
        Some(gate_ids[1]),
        Some([0, in_ids[1]]),
        None,
        None,
    );
    layer.append_gates(vec![gate0, gate1]);

    layer
}

fn get_xor_first_layer(in_ids: Vec<u64>, gate_ids: Vec<u64>, degree: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);
    layer.add_gate_0_and_1(degree);

    let mut gate0: Gate<i64> = Gate::new_add_gate();
    let mut gate1: Gate<i64> = Gate::new_mult_gate();

    gate0.set_all(
        Some(degree),
        Some(gate_ids[0]),
        Some([in_ids[0], in_ids[1]]),
        None,
        None,
    );
    gate1.set_all(
        Some(degree),
        Some(gate_ids[1]),
        Some([in_ids[0], in_ids[1]]),
        None,
        None,
    );
    layer.append_gates(vec![gate0, gate1]);

    layer
}

#[allow(non_snake_case)]
fn get_xor_second_layer(in_ids: Vec<u64>, out_ids: Vec<u64>, degree: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);
    //layer.add_gate_0_and_1(degree);
    let mut gate0: Gate<i64> = Gate::new_R1CS_gate();

    let input_id_R1CS = Some([vec![in_ids[0], in_ids[1]], vec![1]]);
    let weights_R1CS = Some([vec![1, -2], vec![1]]);
    gate0.set_all(
        Some(degree),
        Some(out_ids[0]),
        None,
        input_id_R1CS,
        weights_R1CS,
    );

    layer.append_gates(vec![gate0]);

    layer
}

pub fn get_xor_as_layers(in_ids: Vec<u64>, out_id: u64, degree: u64) -> Vec<Layer<i64>> {
    //let layer0 = get_xor_zero_layer(in_ids.clone(), in_ids.clone(), degree);
    let layer1 = get_xor_first_layer(in_ids.clone(), in_ids.clone(), degree);
    let layer2 = get_xor_second_layer(in_ids.clone(), vec![out_id], degree);
    let layers = vec![layer1, layer2];
    layers
}

///XOR for bit_string, uses 2 layers
pub fn get_xor_bitstring_as_layers(
    in_ids0: Vec<u64>,
    in_ids1: Vec<u64>,
    out_ids: Vec<u64>,
    degree: u64,
) -> Vec<Layer<i64>> {
    let size = in_ids0.len();
    let mut layers = vec![Layer::new(), Layer::new()];
    for i in 0..size {
        let layers_xor_bit_i = get_xor_as_layers(vec![in_ids0[i], in_ids1[i]], out_ids[i], degree);
        for j in 0..2 {
            layers[j].merge_layer(layers_xor_bit_i[j].clone());
        }
    }
    layers
}
