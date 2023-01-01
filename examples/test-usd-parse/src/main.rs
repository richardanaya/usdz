use usdz::*;

fn main() {
    let u = Usd::parse(b"{}").unwrap();
    println!("{:?}", u);
}
