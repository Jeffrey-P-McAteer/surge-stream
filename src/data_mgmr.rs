


pub fn gen_rand_raw_layer_data() -> serde_pickle::Value {
  if let Ok(s) = std::env::var("SEED_NONCE") {
    let s = s.parse::<u64>().ok().expect("Seed must be an integer");
    eprintln!("SEED_NONCE = {}", s);
    fastrand::seed( s );
  }

  let data = serde_pickle::Value::Dict( std::collections::btree_map::BTreeMap::new() );


  return data;
}

pub fn get_all_points(data_sea: &serde_pickle::Value) -> Vec<(f64, f64)> {
  let mut points: Vec<(f64, f64)> = vec![];

  match data_sea {
    serde_pickle::Value::Dict(map) => {
      if let Some(geometry_v) = map.get( &serde_pickle::value::HashableValue::String("geometry".into()) ) {
        if let serde_pickle::Value::Dict(geometry_map) = geometry_v {
          if let (Some(serde_pickle::value::Value::F64(x_val)), Some(serde_pickle::value::Value::F64(y_val))) = (geometry_map.get( &serde_pickle::value::HashableValue::String("x".into()) ), geometry_map.get( &serde_pickle::value::HashableValue::String("y".into()) )) {
            points.push( (*y_val, *x_val) ); // lat, lon
          }
        }
      }
      else {
        for (k,v) in map.iter() {
          let mut sub_points = get_all_points(v);
          points.append(&mut sub_points);
        }
      }
    }
    serde_pickle::Value::List(list) => {
      for v in list.iter() {
        let mut sub_points = get_all_points(v);
        points.append(&mut sub_points);
      }
    }
    _unused => {

    }
  }

  return points;
}



