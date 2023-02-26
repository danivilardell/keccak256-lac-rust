use lac::and::*;
use lac::not::*;
use lac::utils::*;
use lac::xor::*;

/// RC for the first 24 rounds
/// All constants can be precomputed since they form a multiplicative cyclic group of order 255
const RC: [u64; 24] = [
    1,
    32898,
    9223372036854808714,
    9223372039002292224,
    32907,
    2147483649,
    9223372039002292353,
    9223372036854808585,
    138,
    136,
    2147516425,
    2147483658,
    2147516555,
    9223372036854775947,
    9223372036854808713,
    9223372036854808579,
    9223372036854808578,
    9223372036854775936,
    32778,
    9223372039002259466,
    9223372039002292353,
    9223372036854808704,
    2147483649,
    9223372039002292232,
];

const ROT: [u64; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];

pub fn get_keccak_f_layers(input_ids: Vec<u64>, degree: u64, w: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let mut n = 12;
    let mut pow = 1;
    while pow != w {
        pow *= 2;
        n += 2;
    }

    for i in 0..n {
        let mut layer_f_fun = get_keccak_f_round_layers(input_ids.clone(), degree, w, i);
        layers.append(&mut layer_f_fun);
    }

    /*let mut gate_amount = 0;
    for i in 0..layers.len() {
        gate_amount += layers[i].gates_amount();
    }
    println!("keccak_f ===> layers: {:?}, gates: {:?}", layers.len(), gate_amount);*/

    layers
}

/// layers: 27      gates: 43940
pub fn get_keccak_f_round_layers(
    input_ids: Vec<u64>,
    mut degree: u64,
    w: u64,
    round: usize,
) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let mut omega_step_layers = get_keccak_f_omega_step_layers(input_ids.clone(), degree, w);
    for i in 0..omega_step_layers.len() {
        let degree = omega_step_layers[i].get_degree();
        omega_step_layers[i].add_gate_0_and_1(degree);
    }
    layers.append(&mut omega_step_layers);
    degree += omega_step_layers.len() as u64;

    let mut pi_step_layer = get_keccak_f_pi_rho_steps_layer(input_ids.clone(), degree, w);
    pi_step_layer.add_gate_0_and_1(degree);
    layers.push(pi_step_layer);
    degree += 1;

    let mut chi_step_layer = get_keccak_f_chi_step_layer(input_ids.clone(), degree, w);
    for i in 0..chi_step_layer.len() {
        chi_step_layer[i].add_gate_0_and_1(degree);
        layers.push(chi_step_layer[i].clone());
        degree += 1;
    }

    let mut iota_step_layer = get_keccak_f_iota_step_layer(input_ids.clone(), degree, w, RC[round]);
    iota_step_layer.add_gate_0_and_1(degree);
    layers.push(iota_step_layer);

    /*let mut gate_amount = 0;
    for i in 0..layers.len() {
        gate_amount += layers[i].gates_amount();
    }
    println!("keccak_f round ===> layers: {:?}, gates: {:?}", layers.len(), gate_amount);*/

    layers
}

/// layers: 12      gates: 23052
#[allow(non_snake_case)]
pub fn get_keccak_f_omega_step_layers(
    input_ids: Vec<u64>,
    mut degree: u64,
    w: u64,
) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();
    let mut curr_degree = degree;
    for i in 0..6 {
        let mut layer0: Layer<i64> = Layer::new();
        let mut layer1: Layer<i64> = Layer::new();
        layer0.set_degree(degree + 2 * i);
        layer1.set_degree(degree + 2 * i + 1);
        layer0.copy_gates_by_ids(input_ids.clone());
        if i != 5 {
            layer1.copy_gates_by_ids(input_ids.clone());
        }
        layers.append(&mut vec![layer0, layer1]);
    }

    for i in 0..5 {
        let out_ids_C: Vec<u64> = (((1e9 as u64) + i * w)..((1e9 as u64) + (i + 1) * w)).collect();
        for j in 1..5 {
            let in_ids0: Vec<u64> =
                input_ids[(((i * 5 + j) * w) as usize)..(((i * 5 + j + 1) * w) as usize)].to_vec();
            let in_ids1: Vec<u64>;
            if j == 0 {
                in_ids1 =
                    input_ids[(((i * 5) * w) as usize)..(((i * 5 + 1) * w) as usize)].to_vec();
            } else {
                in_ids1 = out_ids_C.clone();
            }
            let layers_xor = get_xor_bitstring_as_layers(
                in_ids0.clone(),
                in_ids1.clone(),
                out_ids_C.clone(),
                curr_degree + 2 * (j - 1),
            );
            layers[(2 * (j - 1)) as usize].merge_layer(layers_xor[0].clone());
            layers[(2 * (j - 1) + 1) as usize].merge_layer(layers_xor[1].clone());
        }
    }
    curr_degree += 4 * 2;
    for i in 0..5 {
        let out_ids_D: Vec<u64> =
            (((2 * 1e9 as u64) + i * w)..((2 * 1e9 as u64) + (i + 1) * w)).collect();
        let in_ids0: Vec<u64> =
            (((1e9 as u64) + (i + 4) % 5 * w)..((1e9 as u64) + ((i + 4) % 5 + 1) * w)).collect();
        let in_ids1: Vec<u64> = rot_ids(
            (((1e9 as u64) + (i + 1) % 5 * w)..((1e9 as u64) + ((i + 1) % 5 + 1) * w)).collect(),
            1,
        );
        let mut layers_xor = get_xor_bitstring_as_layers(in_ids0, in_ids1, out_ids_D, curr_degree);

        let C_ids: Vec<u64> = ((1e9 as u64)..((1e9 as u64) + 5 * w)).collect();
        for i in 0..2 {
            layers_xor[i].copy_gates_by_ids(C_ids.clone());
            layers[(curr_degree-degree) as usize + i].merge_layer(layers_xor[i].clone());
        }
    }
    curr_degree += 2;

    for i in 0..5 {
        for j in 0..5 {
            let in_ids0: Vec<u64> =
                input_ids[(((i * 5 + j) * w) as usize)..(((i * 5 + j + 1) * w) as usize)].to_vec();
            let in_ids1: Vec<u64> =
                (((2 * 1e9 as u64) + j * w)..((2 * 1e9 as u64) + (j + 1) * w)).collect();

            let out_ids = in_ids0.clone();
            let layers_xor = get_xor_bitstring_as_layers(in_ids0, in_ids1, out_ids, curr_degree);
            for i in 0..2 {
                layers[(curr_degree-degree) as usize + i].merge_layer(layers_xor[i].clone());
            }
        }
    }

    /*let mut gate_amount = 0;
    for i in 0..layers.len() {
        gate_amount += layers[i].gates_amount();
    }
    println!("omega step ===> layers: {:?}, gates: {:?}", layers.len(), gate_amount);*/

    layers
}

/// layers: 1       gates: ?
pub fn get_keccak_f_pi_rho_steps_layer(input_ids: Vec<u64>, degree: u64, w: u64) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);

    for i in 0..5 {
        for j in 0..5 {
            let in_ids: Vec<u64> = input_ids
                [((((j + 5 * i) * w) as usize)..(((j + 5 * i + 1) * w) as usize))]
                .to_vec();
            let out_ids: Vec<u64> = (1e9 as u64 + (i + 5 * ((2 * j + 2 * i) % 5)) * w
                ..1e9 as u64 + (i + 5 * ((2 * j + 2 * i) % 5) + 1) * w)
                .collect();

            layer.copy_gates_by_ids_set_out(rot_ids(in_ids, ROT[(j + 5 * i) as usize]), out_ids);
        }
    }
    layer.copy_gates_by_ids(input_ids.clone());

    //println!("pi and rho steps ===> layers: {:?}, gates: {:?}", 1, layer.gates_amount());

    layer
}

/// layers: 4       gates: 17602
pub fn get_keccak_f_chi_step_layer(input_ids: Vec<u64>, degree: u64, w: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = vec![];

    for i in 0..4 {
        let mut layer: Layer<i64> = Layer::new();
        layer.set_degree(degree + i);
        if i != 3 {
            layer.copy_gates_by_ids(input_ids.clone());
        }
        layers.push(layer);
    }

    for i in 0..5 {
        for j in 0..5 {
            let mut curr_degree = degree;
            let in_ids_not: Vec<u64> = (1e9 as u64 + ((j + 1) % 5 + 5 * i) * w
                ..1e9 as u64 + ((j + 1) % 5 + 5 * i + 1) * w)
                .collect();
            let out_ids_not: Vec<u64> = (2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i) * w
                ..2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i + 1) * w)
                .collect();
            let mut not_layer =
                get_not_bitstring_as_layer(in_ids_not.clone(), out_ids_not, curr_degree);
            not_layer.copy_gates_by_ids(in_ids_not.clone());
            layers[0].merge_layer(not_layer);
            curr_degree += 1;

            let in_ids0_and: Vec<u64> = (1e9 as u64 + ((j + 2) % 5 + 5 * i) * w
                ..1e9 as u64 + ((j + 2) % 5 + 5 * i + 1) * w)
                .collect();
            let in_ids1_and: Vec<u64> = (2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i) * w
                ..2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i + 1) * w)
                .collect();
            let out_ids_and: Vec<u64> = (2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i) * w
                ..2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i + 1) * w)
                .collect();
            let mut and_layer =
                get_and_bitstring_as_layers(in_ids0_and, in_ids1_and, out_ids_and, curr_degree);
            and_layer.copy_gates_by_ids(in_ids_not.clone());
            layers[1].merge_layer(and_layer);
            curr_degree += 1;

            let in_ids1_xor: Vec<u64> =
                (1e9 as u64 + (j + 5 * i) * w..1e9 as u64 + (j + 5 * i + 1) * w).collect();
            let in_ids2_xor: Vec<u64> = (2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i) * w
                ..2 * 1e9 as u64 + ((j + 1) % 5 + 5 * i + 1) * w)
                .collect();
            let out_ids_xor: Vec<u64> = input_ids
                [((((j + 5 * i) * w) as usize)..(((j + 5 * i + 1) * w) as usize))]
                .to_vec();
            let mut xor_layers =
                get_xor_bitstring_as_layers(in_ids1_xor, in_ids2_xor, out_ids_xor, curr_degree);
            xor_layers[0].copy_gates_by_ids(in_ids_not.clone());
            xor_layers[1].copy_gates_by_ids(in_ids_not.clone());
            layers[2].merge_layer(xor_layers[0].clone());
            layers[3].merge_layer(xor_layers[1].clone());
        }
    }

    /*let mut gate_amount = 0;
    for i in 0..layers.len() {
        gate_amount += layers[i].gates_amount();
    }
    println!("chi step ===> layers: {:?}, gates: {:?}", layers.len(), gate_amount);*/

    layers
}

/// layers: 1       gates: 64
#[allow(non_snake_case)]
pub fn get_keccak_f_iota_step_layer(
    in_ids: Vec<u64>,
    degree: u64,
    w: u64,
    mut RC_round: u64,
) -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();
    layer.set_degree(degree);
    for i in 0..w {
        let curr = RC_round % 2;
        RC_round /= 2;
        if curr == 1 {
            layer.merge_layer(get_not_as_layer(
                in_ids[(w - i - 1) as usize],
                in_ids[(w - i - 1) as usize],
                degree,
            ))
        } else {
            layer.copy_gates_by_ids(vec![in_ids[(w - i - 1) as usize]]);
        }
    }

    //println!("iota step ===> layers: {:?}, gates: {:?}", 1, layer.gates_amount());

    layer
}

pub fn rot_ids(vec: Vec<u64>, n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];
    for i in 0..vec.len() {
        res.push(vec[((i + n as usize) % vec.len()) as usize])
    }
    res
}
