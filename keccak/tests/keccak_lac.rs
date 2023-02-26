use lac::utils::LAC;
use keccak::keccak_circuit::*;

#[test]
fn test_keccak_circuit() {
    let input: Vec<i64> = vec![1,0,0,1,0,1,1,0,1,1,0,0,0,1,0,1];
    let keccak: LAC<i64> = get_keccak_lac_circuit(input, 1152, 448, 256);
    println!("layers: {:?}, gates: {:?}", keccak.get_layers_amount(), keccak.get_gates_amount())
}

#[test]
fn test_keccak_circuit_OK() {
    //"OK" = 0100111101001011
    let input: Vec<i64> = vec![0,1,0,0,1,1,1,1,0,1,0,0,1,0,1,1];
    let mut keccak: LAC<i64> = get_keccak_lac_circuit(input, 1344, 256, 256);
    println!("layers: {:?}, gates: {:?}", keccak.get_layers_amount(), keccak.get_gates_amount());
    let result = keccak.evaluate();
    println!("result: {:?}", result);
}
