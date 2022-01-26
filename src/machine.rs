pub trait Machine {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, value: u8);
}