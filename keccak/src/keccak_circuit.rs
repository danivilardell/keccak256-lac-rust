use crate::omega_step::*;
use lac::utils::*;
use lac::xor::*;

pub fn get_keccak_lac_circuit(input: Vec<i64>) -> LAC<i64> {
    let mut lac: LAC<i64> = LAC::new();

    let basic_layer = get_keccak_basic_layer(input);

    lac.set_basic_layer(basic_layer);

    lac
}

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

///in ids from 2..size+2
pub fn get_keccak_absorbing_phase_layers(input_ids: Vec<u64>, size_out: u64) -> Vec<Layer<i64>> {
    let mut layers: Vec<Layer<i64>> = Vec::new();

    let size_in = input_ids.len();
    let mut layer0 = Layer::new();
    let mut degree = 1;
    layer0.add_gate_0_and_1(1);
    for i in 0..size_in {
        let mut gate = Gate::new_add_gate();
        gate.set_all(
            Some(degree),
            Some(input_ids[i]),
            Some([0, input_ids[i]]),
            None,
            None,
        );
        layer0.append_gate(gate);
    }

    //get S[x,y,z] where x, y in F5 and z id up to output size
    let out_ids: Vec<u64> = (input_ids.last().unwrap() + 1
        ..(size_out * 5 * 5 + input_ids.last().unwrap() + 1))
        .collect();
    for id in out_ids {
        let mut gate = Gate::new_add_gate();
        gate.set_all(Some(degree), Some(id), Some([0, 0]), None, None);
        layer0.append_gate(gate);
    }

    layers.push(layer0);
    degree += 1;
    /*for i in 0..(size_in / (size_out as usize)) {
        let mut layers_xor = get_xor_bitstring_as_layers(
            input_ids[(i * (size_out as usize))..(i * (size_out as usize))].to_vec(),
            ,
            vec![],
            degree,
        );
    }*/

    layers
}

/*pub fn get_keccak_f_function(input_ids: Vec<u64>) -> Vec<Layer<i64>> {

}*/
