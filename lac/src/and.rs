use crate::utils::*;

pub fn get_and_as_layer(in_ids: Vec<u64>, out_id: u64, degree: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);

    let mut gate: Gate<i64> = Gate::new_mult_gate();

    gate.set_all(
        Some(degree),
        Some(out_id),
        Some([in_ids[0], in_ids[1]]),
        None,
        None,
    );

    layer.append_gate(gate);

    layer
}

///AND for bit_string, uses 1 layer
pub fn get_and_bitstring_as_layers(
    in_ids0: Vec<u64>,
    in_ids1: Vec<u64>,
    out_ids: Vec<u64>,
    degree: u64,
) -> Layer<i64> {
    let size = in_ids0.len();
    let mut layer: Layer<i64> = Layer::new();
    for i in 0..size {
        let layer_and_bit_i = get_and_as_layer(vec![in_ids0[i], in_ids1[i]], out_ids[i], degree);
        layer.merge_layer(layer_and_bit_i);
    }
    layer
}