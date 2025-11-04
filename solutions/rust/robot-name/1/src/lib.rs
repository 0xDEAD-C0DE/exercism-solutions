use rand::Rng;
#[derive(Debug)]
pub struct Robot {
    name: String,
}
static mut NAMES: Vec<String> = vec![];

impl Robot {

    pub fn new() -> Self {
        let mut robot_name = gen_name();

        unsafe { 
            if !NAMES.contains(&robot_name) { NAMES.push(robot_name.clone());
            } else {
                robot_name = gen_name();
            } 
        }
                Self {
            name: robot_name,
        }

    }

    pub fn name(&self) -> &str {
        &self.name
    }
     
    pub fn reset_name(&mut self) {
        self.name = gen_name();
    }
}

pub fn gen_name() -> String {
    let mut name: String = String::new(); 
    name.push(char::from_u32(rand::thread_rng().gen_range(0x0041..=0x005A)).unwrap());
    name.push(char::from_u32(rand::thread_rng().gen_range(0x0041..=0x005A)).unwrap());
    name.push_str(&rand::thread_rng().gen_range(100..1000).to_string());
    name

}