use crate::keccak_f_circuit::*;
use lac::utils::*;
use lac::xor::*;

const rot: [i64; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];
/// RC for the first 12 rounds
//const RC: [i64; 12] = [1, 32898, 9223372036854808714, 9223372039002292224, 32907, 2147483649,
//                       9223372039002292353, 9223372036854808585, 138, 136, 2147516425, 2147483658];
const RC: [i64; 12] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

const rot_rc_ids: u64 = 1e8 as u64;

///c - capacity, r - bitrate, l - output length
pub fn get_keccak_lac_circuit(input: Vec<i64>, r: u64, c: u64, l: u64) -> LAC<i64> {
    let mut lac: LAC<i64> = LAC::new();

    let basic_layer = get_keccak_basic_layer(input.clone());
    lac.set_basic_layer(basic_layer);

    let blocks_amount = (input.len() as u64 - 1) / r + 1;
    let w = (r + c) / 25;

    let first_layer = get_keccak_first_layer(input.len() as u64, r, w, blocks_amount);
    lac.append_layer(first_layer);

    let absorbing_phase_layers = get_keccak_absorbing_phase_layers(blocks_amount, r, w);
    lac.append_layers(absorbing_phase_layers);

    lac
}

///layer with 0, 1, input
pub fn get_keccak_basic_layer(input: Vec<i64>) -> BasicLayer<i64> {
    let size = input.len();
    let mut basic_layer = BasicLayer::new();
    let mut value0: Value<i64> = Value::new();
    let mut value1: Value<i64> = Value::new();

    value0.set_all(0, 0);
    value1.set_all(1, 1);
    basic_layer.append_values(vec![value0, value1]);

    for i in 0..size {
        let mut value0: Value<i64> = Value::new();
        let id = i as u64 + 2;
        value0.set_all(id, input[i]);
        basic_layer.append_value(value0);
    }

    for i in rot_rc_ids..(rot_rc_ids + 25) {
        let mut valuerot: Value<i64> = Value::new();
        let pos: usize = (i - rot_rc_ids) as usize;
        valuerot.set_all(i, rot[pos]);
    }

    for i in (2 * rot_rc_ids as u64)..(2 * rot_rc_ids + 12) {
        let mut valuerot: Value<i64> = Value::new();
        let pos: usize = (i - 2 * rot_rc_ids) as usize;
        valuerot.set_all(i, RC[pos]);
    }

    basic_layer
}

/// First layer is 0, 1, input, padding, S[x][y][w]
/// I'm using a 10*1 padding
pub fn get_keccak_first_layer(input_size: u64, r: u64, w: u64, blocks_amount: u64) -> Layer<i64> {
    //0, 1, input
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(1);
    layer.copy_gates_by_ids((0..(input_size + 1)).collect());
    ///Copy input gates
    layer.copy_gates_by_ids((rot_rc_ids..(rot_rc_ids + 25)).collect());
    ///Copy rot gates
    layer.copy_gates_by_ids(((2 * rot_rc_ids)..(2 * rot_rc_ids + (RC.len() as u64))).collect());
    ///Copy RC gates
    //padding 10*1
    let mut gate: Gate<i64> = Gate::new_add_gate();
    gate.set_all(Some(1), Some(input_size + 1), Some([0, 1]), None, None);
    layer.append_gate(gate);
    for i in (input_size + 2)..(blocks_amount * r - 1) {
        let mut gate: Gate<i64> = Gate::new_add_gate();
        gate.set_all(Some(1), Some(i), Some([0, 0]), None, None);
        layer.append_gate(gate);
    }
    let mut gate: Gate<i64> = Gate::new_add_gate();
    gate.set_all(
        Some(1),
        Some(blocks_amount * r - 1),
        Some([0, 1]),
        None,
        None,
    );
    layer.append_gate(gate);

    /// S[x][y][w] = 0 for x, y int 0..4 and w in 0..(c+r)/25
    for i in (blocks_amount * r)..(blocks_amount * r + 5 * 5 * w) {
        let mut gate: Gate<i64> = Gate::new_add_gate();
        gate.set_all(Some(1), Some(i), Some([0, 0]), None, None);
        layer.append_gate(gate);
    }

    layer
}

/// Absorbing phase is made by concatenating two operations,
/// 1: S_i_subst = P_i | S_i_subst    where S_i_subst as size r/w
/// 2: S_(i+1)1 = f(S_i)
/// Where f is the Keccak-f
pub fn get_keccak_absorbing_phase_layers(blocks_amount: u64, r: u64, w: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let mut degree = 2;
    for i in 0..blocks_amount {
        let p_i: Vec<u64> = ((2 + i * r)..(2 + (i + 1) * r)).collect();
        let s_i_subst: Vec<u64> = ((2 + blocks_amount * r)..(2 + blocks_amount * r + r)).collect();
        let mut layers_xor = get_xor_bitstring_as_layers(s_i_subst, p_i, vec![], 2);
        layers.append(&mut layers_xor);
        degree += 3;

        let s_i: Vec<u64> =
            ((2 + blocks_amount * r)..(2 + blocks_amount * r + 5 * 5 * w)).collect();
        let mut layers_keccak_f_function: Vec<Layer<i64>> = get_keccak_f_function(s_i, degree);
        layers.append(&mut layers_keccak_f_function);
        //degree += m
        //TODO: degree += m where m is keccak_f function amount of layers
    }

    layers
}

/// Squeezing phase is made by concatenating two operations,
/// 1: Z = Z || S_i_substr      where S_i_substr as size r/w
/// 2: S_(i+1) = f(S_i)
/// Where f is the Keccak-f
/// We will iterate this loop until |Z| >= l, the output_size
pub fn get_keccak_squeezing_phase_layers(r: u64, w: u64, l: u64) {}

///keccak_f function with m layers(Still don't know how many layers it will take)
pub fn get_keccak_f_function(input_ids: Vec<u64>, degree: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let mut layers_res: Vec<Layer<i64>> = vec![];
    for mut layer in layers {
        layer.copy_gates_by_ids((rot_rc_ids..(rot_rc_ids + 25)).collect());
        layer.copy_gates_by_ids(((2 * rot_rc_ids)..(2 * rot_rc_ids + (RC.len() as u64))).collect());
        layers_res.push(layer);
    }
    layers_res
}
