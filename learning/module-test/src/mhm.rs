use super::test::CoolStruct;

impl CoolStruct {
    pub fn print_test(&self) {
        println!("{}", self.test);
    }
}
