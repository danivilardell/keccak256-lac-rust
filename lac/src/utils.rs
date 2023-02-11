use std::ops::{Add, Mul};
use std::collections::HashMap;
use std::cell::RefCell;

pub struct LAC<T> {
    basic_layer: BasicLayer<T>,
    layers: Vec<Layer<T>>,
    output_id: Option<u64>,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> LAC<T> {

    pub fn new() -> Self {
        LAC {basic_layer: BasicLayer::new(), layers: Vec::new(), output_id: None}
    }

    pub fn set_basic_layer(&mut self, basic_layer: BasicLayer<T>) {
        self.basic_layer = basic_layer;
    }

    pub fn append_layer(&mut self, layer: Layer<T>) {
        self.layers.push(layer);
    }

    pub fn set_output_id(&mut self, output_id: u64) {
        self.output_id = Some(output_id);
    }

    pub fn evaluate(&mut self) -> T {
        for layer in self.layers.iter_mut() {
            layer.evaluate();
        }
        self.layers.last().unwrap().gates[&self.output_id.unwrap()].borrow().output.unwrap()
    }

}

pub struct Layer<T> {
    degree: Option<u64>,
    gates: HashMap<u64, RefCell<Gate<T>>>, //id -> gate
    output: HashMap<u64, T>,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Layer<T> {

    pub fn new() -> Self {
        Layer{ degree: None, gates: HashMap::new(), output: HashMap::new()}
    }

    pub fn append_gate(&mut self, gate: Gate<T>) {
        self.gates.insert(gate.id.unwrap(), RefCell::new(gate));
    }

    pub fn set_degree(&mut self, degree: u64) {
        self.degree = Some(degree);
    }

    fn evaluate(&mut self) {
        for(id, gate) in self.gates.iter() {
            let mut g = gate.borrow_mut();
            g.evaluate();
            self.output.insert(*id, g.output.unwrap());
        }
    }

}

#[derive(Clone)]
pub struct BasicLayer<T> {
    values: HashMap<u64, Value<T>>,
}

impl<T> BasicLayer<T> {
    pub fn new() -> Self {
        BasicLayer{values: HashMap::new()}
    }

    pub fn append_value(&mut self, value: Value<T>) {
        self.values.insert(value.id.unwrap(), value);
    }
}

#[derive(Clone)]
pub struct Value<T> {
    id: Option<u64>,
    value: Option<T>,
}

impl<T> Value<T> {

    pub fn new() -> Self {
        Value{id: None, value: None}
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

}

enum GateType {
    Add,
    Mult
}

pub struct Gate<T> {
    degree: Option<u64>,
    parent_layer: Option<Layer<T>>,
    basic_layer: Option<BasicLayer<T>>,
    gate_type: GateType,
    id: Option<u64>,
    input_id: Option<[u64; 2]>, //input id
    input: Option<[T; 2]>,
    output: Option<T>, //output value
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Gate<T> {

    pub fn new_add_gate() -> Self {
        Gate{
            degree: None,
            parent_layer: None,
            basic_layer: None,
            gate_type: GateType::Add,
            id: None,
            input_id: None,
            input: None,
            output: None,
        }
    }

    pub fn new_mult_gate() -> Self {
        Gate{
            degree: None,
            parent_layer: None,
            basic_layer: None,
            gate_type: GateType::Mult,
            id: None,
            input_id: None,
            input: None,
            output: None,
        }
    }

    pub fn set_degree(&mut self, degree: u64) {
        self.degree = Some(degree);
    }

    pub fn set_parent_layer(&mut self, parent_layer: Layer<T>) {
        self.parent_layer = Some(parent_layer);
    }

    pub fn set_basic_layer(&mut self, basic_layer: BasicLayer<T>) {
        self.basic_layer = Some(basic_layer);
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn set_input_id(&mut self, input_id: [u64; 2]) {
        self.input_id = Some(input_id);
    }

    pub fn set_input(&mut self) {
        if self.degree == Some(1) {
            self.input = Some([self.basic_layer.as_ref().unwrap().values[&self.input_id.unwrap()[0]].value.unwrap(),
                        self.basic_layer.as_ref().unwrap().values[&self.input_id.unwrap()[1]].value.unwrap()]);
        }
        else {
            self.input.unwrap()[0] = self.parent_layer.as_ref().unwrap().gates[&self.input_id.unwrap()[0]].borrow().output.unwrap();
            self.input.unwrap()[1] = self.parent_layer.as_ref().unwrap().gates[&self.input_id.unwrap()[1]].borrow().output.unwrap();
        }
    }

    pub fn get_output(&mut self) -> T {
        self.evaluate();
        self.output.unwrap()
    }

    pub fn evaluate(&mut self) {
        self.output = match self.gate_type {
            GateType::Add=> Some(self.input.unwrap()[0] + self.input.unwrap()[1]),
            GateType::Mult => Some(self.input.unwrap()[0]*self.input.unwrap()[1]),
        };
    }

}
