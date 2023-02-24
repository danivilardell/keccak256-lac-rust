use crate::utils::*;

pub fn get_not_as_layer(in_id: u64, out_id: u64, degree: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);

    let mut gate: Gate<i64> = Gate::new_R1CS_gate();

    gate.set_all(
        Some(degree),
        Some(out_id),
        None,
        Some([vec![1], vec![in_id]]),
        Some([vec![1], vec![-1]]),
    );

    layer.append_gate(gate);

    layer
}

///NOT for bit_string, uses 1 layer
pub fn get_not_bitstring_as_layer(in_ids: Vec<u64>, out_ids: Vec<u64>, degree: u64) -> Layer<i64> {
    let size = in_ids.len();
    let mut layer: Layer<i64> = Layer::new();
    for i in 0..size {
        let layer_not_bit_i = get_not_as_layer(in_ids[i], out_ids[i], degree);
        layer.merge_layer(layer_not_bit_i);
    }
    layer
}
