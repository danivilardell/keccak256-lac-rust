use crate::utils::*;

//OR gate implemented using OR(x0,x1) = 1-(1-x0)*(1-x1)
//Using the following layered arithmetic circuit:
//  layer0: g_0=0           g_1=1           g_2=x0          g_3=x1
//  layer1: g_4=g_0+g_0     g_5=g_0+g_1     g_6=(1-g_2)*1   g_7=(1-g_3)*1
//  layer2:         g_8=g_4+g_5         g_9=g_6*g_7
//  layer3:                 g_10=(g_8-g_9)*(g_8)

pub fn get_or_lac_circuit(x0: i64, x1: i64) -> LAC<i64> {
    let mut lac = LAC::new();

    lac.set_basic_layer(get_or_basic_layer(x0, x1));

    let layer1 = get_or_first_layer();
    let layer2 = get_or_second_layer();
    let layer3 = get_or_third_layer();
    let layers = vec![layer1, layer2, layer3];
    lac.append_layers(layers);

    lac.set_output_gate_id(10);

    lac
}

fn get_or_basic_layer(x0: i64, x1: i64) -> BasicLayer<i64> {
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

#[allow(non_snake_case)]
fn get_or_first_layer() -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();

    let mut gate4: Gate<i64> = Gate::new_add_gate();
    let mut gate5: Gate<i64> = Gate::new_add_gate();
    let mut gate6: Gate<i64> = Gate::new_R1CS_gate();
    let mut gate7: Gate<i64> = Gate::new_R1CS_gate();

    gate4.set_all(Some(1), Some(4), Some([0, 0]), None, None);
    gate5.set_all(Some(1), Some(5), Some([0, 1]), None, None);

    let input_id_R1CS = Some([vec![1, 2], vec![1]]);
    let weights_R1CS = Some([vec![1, -1], vec![1]]);
    gate6.set_all(Some(1), Some(6), None, input_id_R1CS, weights_R1CS);

    let input_id_R1CS = Some([vec![1, 3], vec![1]]);
    let weights_R1CS = Some([vec![1, -1], vec![1]]);
    gate7.set_all(Some(1), Some(7), None, input_id_R1CS, weights_R1CS);

    layer.append_gates(vec![gate4, gate5, gate6, gate7]);
    layer.set_degree(1);

    layer
}

fn get_or_second_layer() -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();

    let mut gate8: Gate<i64> = Gate::new_add_gate();
    let mut gate9: Gate<i64> = Gate::new_mult_gate();

    gate8.set_all(Some(2), Some(8), Some([4, 5]), None, None);
    gate9.set_all(Some(2), Some(9), Some([6, 7]), None, None);

    layer.append_gates(vec![gate8, gate9]);
    layer.set_degree(2);

    layer
}

#[allow(non_snake_case)]
fn get_or_third_layer() -> Layer<i64> {
    let mut layer: Layer<i64> = Layer::new();

    let mut gate10: Gate<i64> = Gate::new_R1CS_gate();

    let input_id_R1CS = Some([vec![8, 9], vec![8]]);
    let weights_R1CS = Some([vec![1, -1], vec![1]]);
    gate10.set_all(Some(3), Some(10), None, input_id_R1CS, weights_R1CS);
    layer.append_gate(gate10);
    layer.set_degree(3);

    layer
}
