use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct LAC<T> {
    basic_layer: BasicLayer<T>,
    layers: Vec<Layer<T>>,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + std::iter::Sum + std::fmt::Debug> LAC<T> {
    pub fn new() -> Self {
        LAC {
            basic_layer: BasicLayer::new(),
            layers: Vec::new(),
        }
    }

    pub fn get_gates_amount(&self) -> usize {
        let mut res = 0;
        for layer in &self.layers {
            for (_, gate) in &layer.gates {
                match gate.borrow().gate_type {
                    GateType::R1CS => {
                        res += 3;
                    }
                    _ => {
                        res += 1;
                    }
                }
            }
        }
        res
    }

    pub fn get_layers_amount(&self) -> usize {
        self.layers.len()
    }

    pub fn get_input_size(&self) -> usize {
        self.basic_layer.values.len() - 2
    }

    pub fn set_basic_layer(&mut self, basic_layer: BasicLayer<T>) {
        self.basic_layer = basic_layer;
    }

    pub fn get_basic_layer(&mut self) -> &BasicLayer<T> {
        &self.basic_layer
    }

    pub fn get_layer_by_degree(&mut self, degree: u64) -> &Layer<T> {
        let pos = (degree - 1) as usize;
        &self.layers[pos]
    }

    pub fn append_layer(&mut self, layer: Layer<T>) {
        self.layers.push(layer);
    }

    pub fn append_layers(&mut self, layers: Vec<Layer<T>>) {
        for layer in layers {
            self.layers.push(layer);
        }
    }

    pub fn merge_lac(&mut self, lac: LAC<T>) {
        for (id, value) in lac.basic_layer.values {
            self.basic_layer.values.insert(id, value);
        }
        //TODO: merge_lac should use merge_layer method
        for layer in lac.layers {
            let degree = layer.degree.unwrap() as usize;
            for (id, gate) in layer.gates {
                self.layers[degree].gates.insert(id, gate);
            }
        }
    }

    pub fn add_layers(&mut self, layers: Vec<Layer<T>>) {
        for layer in layers {
            let degree = layer.degree.unwrap() as usize;
            for (id, gate) in layer.gates {
                self.layers[degree].gates.insert(id, gate);
            }
        }
    }

    pub fn evaluate(&mut self) -> Vec<T> {
        for i in 0..self.layers.len() {
            self.layers[i] = self.clone().layers[i].evaluate(self.clone());
        }
        let mut res: Vec<T> = Vec::new();
        for id in self.layers.last().unwrap().gates.keys().sorted() {
            res.push(
                self.layers.last().unwrap().gates[id]
                    .borrow()
                    .output
                    .unwrap()
                    .clone(),
            );
        }
        res
    }
}

#[derive(Clone)]
pub struct Layer<T> {
    degree: Option<u64>,
    gates: HashMap<u64, RefCell<Gate<T>>>, //id -> gate
    output: HashMap<u64, T>,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + std::iter::Sum + std::fmt::Debug> Layer<T> {
    pub fn new() -> Self {
        Layer {
            degree: None,
            gates: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn append_gate(&mut self, gate: Gate<T>) {
        self.gates.insert(gate.id.unwrap(), RefCell::new(gate));
    }

    pub fn append_gates(&mut self, gates: Vec<Gate<T>>) {
        for gate in gates {
            self.gates.insert(gate.id.unwrap(), RefCell::new(gate));
        }
    }

    pub fn gates_amount(&self) -> usize {
        self.gates.len()
    }

    pub fn set_degree(&mut self, degree: u64) {
        self.degree = Some(degree);
    }

    pub fn get_degree(&mut self) -> u64 {
        self.degree.unwrap()
    }

    pub fn add_gate_0_and_1(&mut self, degree: u64) {
        let mut gate0: Gate<T> = Gate::new_add_gate();
        let mut gate1: Gate<T> = Gate::new_add_gate();
        gate0.set_all(Some(degree), Some(0), Some([0, 0]), None, None);
        gate1.set_all(Some(degree), Some(1), Some([0, 1]), None, None);
        self.gates.insert(0, RefCell::new(gate0));
        self.gates.insert(1, RefCell::new(gate1));
    }

    //TODO: merge_layer should check if gate exists with such id
    pub fn merge_layer(&mut self, layer: Layer<T>) {
        for (id, gate) in layer.gates {
            self.gates.insert(id, gate);
        }
    }

    pub fn copy_gates_by_ids(&mut self, ids: Vec<u64>) {
        for id in ids {
            let mut gate: Gate<T> = Gate::new_add_gate();
            gate.set_all(self.degree, Some(id), Some([0, id]), None, None);
            self.append_gate(gate);
        }
    }

    pub fn copy_gates_by_ids_set_out(&mut self, ids: Vec<u64>, out_ids: Vec<u64>) {
        for i in 0..ids.len() {
            let mut gate: Gate<T> = Gate::new_add_gate();
            gate.set_all(self.degree, Some(out_ids[i]), Some([0, ids[i]]), None, None);
            self.append_gate(gate);
        }
    }

    fn evaluate(&mut self, lac: LAC<T>) -> Layer<T> {
        for (id, gate) in self.gates.iter() {
            let mut g = gate.borrow_mut();
            g.set_input(lac.clone());
            g.evaluate();
            self.output.insert(*id, g.output.unwrap());
        }
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub struct BasicLayer<T> {
    values: HashMap<u64, Value<T>>, //id -> Value
}

impl<T> BasicLayer<T> {
    pub fn new() -> Self {
        BasicLayer {
            values: HashMap::new(),
        }
    }

    pub fn append_value(&mut self, value: Value<T>) {
        self.values.insert(value.id.unwrap(), value);
    }

    pub fn append_values(&mut self, values: Vec<Value<T>>) {
        for value in values {
            self.values.insert(value.id.unwrap(), value);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Value<T> {
    id: Option<u64>,
    value: Option<T>,
}

impl<T> Value<T> {
    pub fn new() -> Self {
        Value {
            id: None,
            value: None,
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn set_all(&mut self, id: u64, value: T) {
        self.id = Some(id);
        self.value = Some(value);
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
enum GateType {
    Add,
    Mult,
    R1CS,
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Gate<T> {
    degree: Option<u64>,
    gate_type: GateType,
    id: Option<u64>,
    input_id: Option<[u64; 2]>,           //input id
    input_id_R1CS: Option<[Vec<u64>; 2]>, //input ids
    input: Option<[T; 2]>,
    input_R1CS: Option<[Vec<T>; 2]>,
    R1CS_weights: Option<[Vec<T>; 2]>,
    output: Option<T>, //output value
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + std::iter::Sum + std::fmt::Debug> Gate<T> {
    pub fn new_add_gate() -> Self {
        Gate {
            degree: None,
            gate_type: GateType::Add,
            id: None,
            input_id: None,
            input_id_R1CS: None,
            input: None,
            input_R1CS: None,
            R1CS_weights: None,
            output: None,
        }
    }

    pub fn new_mult_gate() -> Self {
        Gate {
            degree: None,
            gate_type: GateType::Mult,
            id: None,
            input_id: None,
            input_id_R1CS: None,
            input: None,
            input_R1CS: None,
            R1CS_weights: None,
            output: None,
        }
    }
    #[allow(non_snake_case)]
    pub fn new_R1CS_gate() -> Self {
        Gate {
            degree: None,
            gate_type: GateType::R1CS,
            id: None,
            input_id: None,
            input_id_R1CS: None,
            input: None,
            input_R1CS: None,
            R1CS_weights: None,
            output: None,
        }
    }

    #[allow(non_snake_case)]
    pub fn set_all(
        &mut self,
        degree: Option<u64>,
        id: Option<u64>,
        input_id: Option<[u64; 2]>,
        input_id_R1CS: Option<[Vec<u64>; 2]>,
        R1CS_weights: Option<[Vec<T>; 2]>,
    ) {
        self.degree = degree;
        self.id = id;
        self.input_id = input_id;
        self.input_id_R1CS = input_id_R1CS;
        self.R1CS_weights = R1CS_weights;
    }

    pub fn set_degree(&mut self, degree: u64) {
        self.degree = Some(degree);
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn set_input_id(&mut self, input_id: [u64; 2]) {
        self.input_id = Some(input_id);
    }

    #[allow(non_snake_case)]
    pub fn set_input_id_R1CS(&mut self, input_id_R1CS: [Vec<u64>; 2]) {
        self.input_id_R1CS = Some(input_id_R1CS);
    }

    #[allow(non_snake_case)]
    pub fn set_R1CS_weights(&mut self, R1CS_weights: [Vec<T>; 2]) {
        self.R1CS_weights = Some(R1CS_weights);
    }

    pub fn set_input(&mut self, lac: LAC<T>) {
        match self.gate_type {
            GateType::Add | GateType::Mult => {
                if self.degree == Some(1) {
                    self.input = Some([
                        lac.clone().get_basic_layer().values[&self.input_id.unwrap()[0]]
                            .value
                            .unwrap(),
                        lac.clone().get_basic_layer().values[&self.input_id.unwrap()[1]]
                            .value
                            .unwrap(),
                    ]);
                } else {
                    self.input = Some([
                        lac.clone()
                            .get_layer_by_degree(self.degree.unwrap() - 1)
                            .gates[&self.input_id.unwrap()[0]]
                            .borrow()
                            .output
                            .unwrap(),
                        lac.clone()
                            .get_layer_by_degree(self.degree.unwrap() - 1)
                            .gates[&self.input_id.unwrap()[1]]
                            .borrow()
                            .output
                            .unwrap(),
                    ]);
                }
            }
            GateType::R1CS => {
                let mut input_array: [Vec<T>; 2] = [Vec::new(), Vec::new()];
                if self.degree == Some(1) {
                    for id in &self.input_id_R1CS.as_ref().unwrap()[0] {
                        input_array[0]
                            .push(lac.clone().get_basic_layer().values[id].value.unwrap());
                    }
                    for id in &self.input_id_R1CS.as_ref().unwrap()[1] {
                        input_array[1]
                            .push(lac.clone().get_basic_layer().values[id].value.unwrap());
                    }
                } else {
                    for id in &self.input_id_R1CS.as_ref().unwrap()[0] {
                        input_array[0].push(
                            lac.clone()
                                .get_layer_by_degree(self.degree.unwrap() - 1)
                                .gates[id]
                                .borrow()
                                .output
                                .unwrap(),
                        );
                    }
                    for id in &self.input_id_R1CS.as_ref().unwrap()[1] {
                        input_array[1].push(
                            lac.clone()
                                .get_layer_by_degree(self.degree.unwrap() - 1)
                                .gates[id]
                                .borrow()
                                .output
                                .unwrap(),
                        );
                    }
                }
                self.input_R1CS = Some(input_array);
            }
        }
    }

    pub fn get_output(&mut self) -> T {
        self.evaluate();
        self.output.unwrap()
    }

    pub fn evaluate(&mut self) {
        self.output = Some(match self.gate_type {
            GateType::Add => self.input.unwrap()[0] + self.input.unwrap()[1],
            GateType::Mult => self.input.unwrap()[0] * self.input.unwrap()[1],
            GateType::R1CS => {
                let val0: T = self.R1CS_weights.as_ref().unwrap()[0]
                    .iter()
                    .zip(&self.input_R1CS.as_ref().unwrap()[0])
                    .map(|(x, y)| *x * *y)
                    .sum();
                let val1: T = self.R1CS_weights.as_ref().unwrap()[1]
                    .iter()
                    .zip(&self.input_R1CS.as_ref().unwrap()[1])
                    .map(|(x, y)| *x * *y)
                    .sum();
                val0 * val1
            }
        });
    }
}
