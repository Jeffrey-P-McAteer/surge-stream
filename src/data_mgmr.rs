

pub fn get_all_points(data_sea: &serde_pickle::Value) -> Vec<(f64, f64, String)> {
  let mut points: Vec<(f64, f64, String)> = vec![];

  match data_sea {
    serde_pickle::Value::Dict(map) => {
      if let Some(geometry_v) = map.get( &serde_pickle::value::HashableValue::String("geometry".into()) ) {
        if let serde_pickle::Value::Dict(geometry_map) = geometry_v {
          if let (Some(serde_pickle::value::Value::F64(x_val)), Some(serde_pickle::value::Value::F64(y_val))) = (geometry_map.get( &serde_pickle::value::HashableValue::String("x".into()) ), geometry_map.get( &serde_pickle::value::HashableValue::String("y".into()) )) {

            // Combine all attributes into Key=Value string
            let mut debug_s = String::new();
            if let Some(attributes_v) = map.get( &serde_pickle::value::HashableValue::String("attributes".into()) ) {
              if let serde_pickle::Value::Dict(attributes_map) = attributes_v {
                for (k,v) in attributes_map.iter() {
                  let k_string = if let serde_pickle::value::HashableValue::String(k_string_val) = k {
                    k_string_val.clone()
                  }
                  else {
                    format!("{}", k)
                  };
                  let v_string = format!("{:?}", v);
                  debug_s += &format!("{k_string}={v_string}\n");
                }
              }
            }

            points.push( (*y_val, *x_val, debug_s) ); // lat, lon

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

// lat_y, lon_x, Facility Name, type of product produced, quantity in throusand barrels/day
pub fn get_all_producers(data_sea: &serde_pickle::Value) -> Vec<(f64, f64, String, String, f64)> {
  let mut points: Vec<(f64, f64, String, String, f64)> = vec![];

  match data_sea {
    serde_pickle::Value::Dict(map) => {
      if let Some(geometry_v) = map.get( &serde_pickle::value::HashableValue::String("geometry".into()) ) {
        if let serde_pickle::Value::Dict(geometry_map) = geometry_v {
          if let (Some(serde_pickle::value::Value::F64(x_val)), Some(serde_pickle::value::Value::F64(y_val))) = (geometry_map.get( &serde_pickle::value::HashableValue::String("x".into()) ), geometry_map.get( &serde_pickle::value::HashableValue::String("y".into()) )) {

            // Combine all attributes into Key=Value string
            let mut is_a_producer = false;
            let mut name_s = String::new();
            let mut product_type_s = String::new();
            let mut amount_thousand_barrels_per_day = 0.0;

            if let Some(attributes_v) = map.get( &serde_pickle::value::HashableValue::String("attributes".into()) ) {
              if let serde_pickle::Value::Dict(attributes_map) = attributes_v {
                for (k,v) in attributes_map.iter() {
                  /*let k_string = if let serde_pickle::value::HashableValue::String(k_string_val) = k {
                    k_string_val.clone()
                  }
                  else {
                    format!("{}", k)
                  };
                  let v_string = format!("{:?}", v);
                  debug_s += &format!("{k_string}={v_string}\n");*/
                }
              }
            }

            if is_a_producer {
              points.push( (*y_val, *x_val, name_s, product_type_s, amount_thousand_barrels_per_day) ); // lat, lon
            }

          }
        }
      }
      else {
        for (k,v) in map.iter() {
          let mut sub_points = get_all_producers(v);
          points.append(&mut sub_points);
        }
      }
    }
    serde_pickle::Value::List(list) => {
      for v in list.iter() {
        let mut sub_points = get_all_producers(v);
        points.append(&mut sub_points);
      }
    }
    _unused => {

    }
  }

  return points;
}


