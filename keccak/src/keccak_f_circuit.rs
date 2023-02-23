use lac::utils::*;
use lac::xor::*;

///keccak_f function with m layers(Still don't know how many layers it will take)
pub fn get_keccak_f_function(
    input_ids: Vec<u64>,
    degree: u64,
    r_ids: Vec<u64>,
    RC_ids: Vec<u64>,
    w: u64,
) -> Vec<Layer<i64>> {
    let layers: Vec<Layer<i64>> = Vec::new();

    layers
}

/// layers: 21      gates: ?
pub fn get_keccak_f_omega_step_layers(
    input_ids: Vec<u64>,
    mut degree: u64,
    w: u64,
) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();

    for i in 0..6 {
        let mut layer0: Layer<i64> = Layer::new();
        let mut layer1: Layer<i64> = Layer::new();
        let mut layer2: Layer<i64> = Layer::new();
        layer0.set_degree(degree + 3 * i);
        layer1.set_degree(degree + 3 * i + 1);
        layer2.set_degree(degree + 3 * i + 2);
        layer0.copy_gates_by_ids(input_ids.clone());
        layer1.copy_gates_by_ids(input_ids.clone());
        layer2.copy_gates_by_ids(input_ids.clone());
        layers.append(&mut vec![layer0, layer1, layer2]);
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
                degree + 3*(j-1),
            );
            layers[(3*(j-1)) as usize].merge_layer(layers_xor[0].clone());
            layers[(3*(j-1) + 1) as usize].merge_layer(layers_xor[1].clone());
            layers[(3*(j-1) + 2) as usize].merge_layer(layers_xor[2].clone());
        }
    }
    degree += 4*3+1;
    for i in 0..5 {
        let out_ids_D: Vec<u64> =
            (((2 * 1e9 as u64) + i * w)..((2 * 1e9 as u64) + (i + 1) * w)).collect();
        let in_ids0: Vec<u64> =
            (((1e9 as u64) + (i + 4) % 5 * w)..((1e9 as u64) + ((i + 4) % 5 + 1) * w)).collect();
        let in_ids1: Vec<u64> = rot_ids(
            (((1e9 as u64) + (i + 1) % 5 * w)..((1e9 as u64) + ((i + 1) % 5 + 1) * w)).collect(),
            1,
        );
        let mut layers_xor = get_xor_bitstring_as_layers(in_ids0, in_ids1, out_ids_D, degree);

        let C_ids: Vec<u64> = ((1e9 as u64)..((1e9 as u64) + 5 * w)).collect();
        for i in 0..3 {
            layers_xor[i].copy_gates_by_ids(C_ids.clone());
            layers[degree as usize + i].merge_layer(layers_xor[i].clone());
        }
    }
    degree += 3;

    for i in 0..5 {
        for j in 0..5 {
            let in_ids0: Vec<u64> =
                input_ids[(((i * 5 + j) * w) as usize)..(((i * 5 + j + 1) * w) as usize)].to_vec();
            let in_ids1: Vec<u64> =
                (((2 * 1e9 as u64) + j * w)..((2 * 1e9 as u64) + (j + 1) * w)).collect();

            let out_ids = in_ids0.clone();
            let layers_xor = get_xor_bitstring_as_layers(in_ids0, in_ids1, out_ids, degree);
            for i in 0..3 {
                layers[degree as usize + i].merge_layer(layers_xor[i].clone());
            }
        }
    }

    layers
}

pub fn rot_ids(vec: Vec<u64>, n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];
    for i in 0..vec.len() {
        res.push(vec[((i + n as usize) % vec.len()) as usize])
    }
    res
}
