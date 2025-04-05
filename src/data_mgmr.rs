


pub fn gen_rand_raw_layer_data() -> serde_pickle::Value {
  if let Ok(s) = std::env::var("SEED_NONCE") {
    let s = s.parse::<u64>().ok().expect("Seed must be an integer");
    eprintln!("SEED_NONCE = {}", s);
    fastrand::seed( s );
  }

  let data = serde_pickle::Value::Dict( std::collections::btree_map::BTreeMap::new() );


  return data;
}
