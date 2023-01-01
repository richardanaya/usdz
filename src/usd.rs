pub struct Usd {}

impl Usd {
    pub fn parse(_buffer: &[u8]) -> Result<Usd, &'static str> {
        Ok(Usd {})
    }
}
