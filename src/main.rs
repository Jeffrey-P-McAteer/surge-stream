fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<u8> = std::fs::read("data/raw-layer-data.pickle")?;

    let mut reader = serde_pickle::value_from_reader(data.as_slice(), Default::default());
    println!("{:?}", reader);

    Ok(())
}
