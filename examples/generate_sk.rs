use anyhow::Result;
use pasetors::{
    keys::{SymmetricKey, Generate},
    version4::V4, paserk::FormatAsPaserk,
};

fn main() -> Result<()> {
    let key = SymmetricKey::<V4>::generate()?;
    let mut s = String::new();
    key.fmt(&mut s)?;
    println!("{}", s);
    Ok(())
}
