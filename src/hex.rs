pub trait Hex {
    fn hex(&self) -> String;
}

impl<T: AsRef<[u8]>> Hex for T {
    fn hex(&self) -> String {
        self.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}