#[derive(Debug)]
pub struct UsdProperty {
    pub name: String,
    pub kind: String,
    pub value: String,
}
#[derive(Debug)]
pub struct UsdNode {
    pub name: String,
    pub kind: String,
    pub properties: Vec<UsdProperty>,
}
#[derive(Debug)]
pub struct Usd {
    pub nodes: Vec<UsdNode>,
}

fn parse_usd(&_: &str) -> Result<Usd, &'static str> {
    Ok(Usd { nodes: vec![] })
}

impl Usd {
    pub fn parse(buffer: &[u8]) -> Result<Usd, &'static str> {
        if let Ok(buffer_str) = std::str::from_utf8(buffer) {
            return parse_usd(buffer_str);
        } else {
            return Err("Invalid UTF-8");
        }
    }
}
