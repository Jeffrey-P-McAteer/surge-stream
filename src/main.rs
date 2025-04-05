
#![allow(unused_mut)]


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab data
    let data: Vec<u8> = std::fs::read("data/raw-layer-data.pickle")?;
    let de_options = serde_pickle::DeOptions::new()
                        .decode_strings()
                        .replace_recursive_structures()
                        .replace_unresolved_globals();
    let mut data: serde_pickle::Value = serde_pickle::value_from_reader(data.as_slice(), de_options)?;
    println!("data = {:#?}", data);

    // Build spatial indexes



    Ok(())
}
