use crate::keccak_f_circuit::*;
use lac::utils::*;
use lac::xor::*;

///c - capacity, r - bitrate, l - output length
pub fn get_keccak_lac_circuit(input: Vec<i64>, r: u64, c: u64, l: u64) -> LAC<i64> {
    let mut lac: LAC<i64> = LAC::new();

    let mut degree: u64 = 0;
    let basic_layer = get_keccak_basic_layer(input.clone());
    lac.set_basic_layer(basic_layer);
    degree += 1;

    let blocks_amount = (input.len() as u64 - 1) / r + 1;
    let w = (r + c) / 25;

    let first_layer = get_keccak_first_layer(input.len() as u64, r, w, blocks_amount);
    lac.append_layer(first_layer);
    degree += 1;

    let absorbing_phase_layers = get_keccak_absorbing_phase_layers(blocks_amount, r, w);
    lac.append_layers(absorbing_phase_layers.clone());
    degree += 2 + absorbing_phase_layers.len() as u64;

    let squeezing_phase_layers = get_keccak_squeezing_phase_layers(r, degree, w, l);
    lac.append_layers(squeezing_phase_layers.clone());

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

    basic_layer
}

/// First layer is 0, 1, input, padding, S[x][y][w]
/// I'm using a 10*1 padding
pub fn get_keccak_first_layer(input_size: u64, r: u64, w: u64, blocks_amount: u64) -> Layer<i64> {
    //0, 1, input
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(1);
    layer.copy_gates_by_ids((0..(input_size + 1)).collect());
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

    // S[x][y][w] = 0 for x, y int 0..4 and w in 0..(c+r)/25
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
        let mut layers_xor = get_xor_bitstring_as_layers(s_i_subst.clone(), p_i, s_i_subst, 2);
        layers_xor[0].copy_gates_by_ids(((2 + blocks_amount * r + r)..(2 + blocks_amount * r + 5 * 5 * w)).collect());
        layers_xor[1].copy_gates_by_ids(((2 + blocks_amount * r + r)..(2 + blocks_amount * r + 5 * 5 * w)).collect());
        layers.append(&mut layers_xor);
        degree += 2;

        let s_i: Vec<u64> =
            ((2 + blocks_amount * r)..(2 + blocks_amount * r + 5 * 5 * w)).collect();

        let mut keccak_f_layers: Vec<Layer<i64>> = get_keccak_f_layers(s_i, degree, w);
        for i in 0..keccak_f_layers.len() {
            keccak_f_layers[i].copy_gates_by_ids(((2 + (i as u64 + 1) * r)..(2 + (blocks_amount * r))).collect());
        }
        layers.append(&mut keccak_f_layers);
        degree += keccak_f_layers.len() as u64;
    }

    layers
}

/// Squeezing phase is made by concatenating two operations,
/// 1: Z = Z || S_i_substr      where S_i_substr as size r/w
/// 2: S_(i+1) = f(S_i)
/// Where f is the Keccak-f
/// We will iterate this loop until |Z| >= l, the output_size
pub fn get_keccak_squeezing_phase_layers(r: u64, mut degree: u64, w: u64, l: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let out_ids_start = 1e9 as u64;
    for i in 0..((l-1)/r+1) {
        let out_ids = ((out_ids_start + i*r)..(out_ids_start + (i+1)*r)).collect();
        let mut layer: Layer<i64> = Layer::new();
        layer.set_degree(degree);
        layer.copy_gates_by_ids(((out_ids_start)..(out_ids_start + i*r)).collect());
        let curr_s_substr: Vec<u64> = ((2 + i * (25*w))..(2 + i * (25*w) + r)).collect();
        layer.copy_gates_by_ids_set_out(curr_s_substr, out_ids);
        degree += 1;

        let curr_s: Vec<u64> = ((2 + i * (25*w))..(2 + (i + 1) * (25*w))).collect();
        let mut layers_keccak_f = get_keccak_f_layers(curr_s, degree, w);
        degree += layers_keccak_f.len() as u64;
        for j in 0..layers_keccak_f.len() {
            layers_keccak_f[i as usize].copy_gates_by_ids(((out_ids_start)..(out_ids_start + (i as u64 +1)*r)).collect());
            layers.push(layers_keccak_f[i as usize].clone());
        }
    }

    let mut last_layer: Layer<i64> = Layer::new();
    last_layer.copy_gates_by_ids_set_out(((out_ids_start)..(out_ids_start + l)).collect(), (0..l).collect());
    layers.push(last_layer);

    layers
}
