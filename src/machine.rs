pub trait Value {}

impl Value for i32 {}
impl Value for u32 {}
impl Value for i64 {}
impl Value for u64 {}
impl Value for f32 {}
impl Value for f64 {}

#[derive(Default)]
struct Machine {
    instructions: Vec<String>,
    stack: Vec<Box<dyn Value>>,
    memory: Vec<u8>,
}

impl Machine {
    fn execute(&self) {
        println!("Execute called!");
    }
}