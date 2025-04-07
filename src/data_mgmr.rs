

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


pub fn get_all_lines(data_sea: &serde_pickle::Value) -> Vec<crate::gis_structs::DebugLine> {
  let mut lines: Vec<crate::gis_structs::DebugLine> = vec![];

  match data_sea {
    serde_pickle::Value::Dict(map) => {
      if let Some(geometry_v) = map.get( &serde_pickle::value::HashableValue::String("geometry".into()) ) {
        if let serde_pickle::Value::Dict(geometry_map) = geometry_v {
          if let Some(serde_pickle::value::Value::List(paths_list)) = geometry_map.get( &serde_pickle::value::HashableValue::String("paths".into()) ) {

            // Combine all attributes into Key=Value string; these attributes are shared across ALL path components below
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

            // Skip this if it is a railway; // TODO return to these features l8ter
            if debug_s.contains("Railroad") {
              return lines;
            }

            for paths_list_v in paths_list.iter() {
              let mut line_string_vec: Vec<(f64, f64)> = vec![];
              if let serde_pickle::Value::List(one_path_list) = paths_list_v {
                for path_coordinate_v in one_path_list.iter() {
                  if let serde_pickle::Value::List(one_coordinate_list) = path_coordinate_v {
                    if one_coordinate_list.len() > 1 {
                      if let (serde_pickle::value::Value::F64(lon_x_val), serde_pickle::value::Value::F64(lat_y_val)) = (one_coordinate_list[0].clone(), one_coordinate_list[1].clone()) {
                        line_string_vec.push((lon_x_val, lat_y_val));
                      }
                    }
                  }
                }
              }
              if line_string_vec.len() > 1 {
                lines.push(crate::gis_structs::DebugLine {
                  msg: debug_s.clone(),
                  geom: gpkg::types::GPKGLineString( line_string_vec.into() ),
                });
              }
            }

            // lines.push( (*y_val, *x_val, debug_s) ); // lat, lon

          }
        }
      }
      else {
        for (k,v) in map.iter() {
          let mut sub_lines = get_all_lines(v);
          lines.append(&mut sub_lines);
        }
      }
    }
    serde_pickle::Value::List(list) => {
      for v in list.iter() {
        let mut sub_lines = get_all_lines(v);
        lines.append(&mut sub_lines);
      }
    }
    _unused => {

    }
  }

  return lines;
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
                let mut contains_mw_key_indicating_reciever_of_fuel = false;
                for (k,v) in attributes_map.iter() {
                  if let serde_pickle::value::HashableValue::String(key_str) = k {
                    if key_str.ends_with("_MW") || key_str.ends_with("_mw") {
                      contains_mw_key_indicating_reciever_of_fuel = true;
                    }
                  }
                }
                if contains_mw_key_indicating_reciever_of_fuel {
                  return points; // Skip this item, as it contains info making it 100% a consumer of a resource
                }
                for (k,v) in attributes_map.iter() {
                  let v_string_lower = format!("{:?}", v).to_lowercase();
                  if v_string_lower.contains("gas") && v_string_lower.contains("process") && v_string_lower.contains("plant") {
                    // Is DEFINITELY a natural gas producer!
                    is_a_producer = true;
                    product_type_s = "natural gas".to_string();
                  }

                  /*let k_string = if let serde_pickle::value::HashableValue::String(k_string_val) = k {
                    k_string_val.clone()
                  }
                  else {
                    format!("{}", k)
                  };
                  let v_string = format!("{:?}", v);
                  debug_s += &format!("{k_string}={v_string}\n");*/
                }
                if is_a_producer {
                  amount_thousand_barrels_per_day = read_number(&["Plant_Flow", "!"], attributes_map);
                  name_s = read_string_containing(&["Name", "name", "NAME", "Company"], attributes_map);
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


// lat_y, lon_x, Facility Name, type of product produced, quantity in throusand barrels/day
pub fn get_all_consumers(data_sea: &serde_pickle::Value) -> Vec<(f64, f64, String, String, f64)> {
  let mut points: Vec<(f64, f64, String, String, f64)> = vec![];

  match data_sea {
    serde_pickle::Value::Dict(map) => {
      if let Some(geometry_v) = map.get( &serde_pickle::value::HashableValue::String("geometry".into()) ) {
        if let serde_pickle::Value::Dict(geometry_map) = geometry_v {
          if let (Some(serde_pickle::value::Value::F64(x_val)), Some(serde_pickle::value::Value::F64(y_val))) = (geometry_map.get( &serde_pickle::value::HashableValue::String("x".into()) ), geometry_map.get( &serde_pickle::value::HashableValue::String("y".into()) )) {

            // Combine all attributes into Key=Value string
            let mut is_a_consumer = false;
            let mut name_s = String::new();
            let mut product_type_s = String::new();
            let mut amount_thousand_barrels_per_day = 0.0;

            if let Some(attributes_v) = map.get( &serde_pickle::value::HashableValue::String("attributes".into()) ) {
              if let serde_pickle::Value::Dict(attributes_map) = attributes_v {

                let mut contains_mw_key_indicating_reciever_of_fuel = false;
                for (k,v) in attributes_map.iter() {
                  if let serde_pickle::value::HashableValue::String(key_str) = k {
                    if key_str.ends_with("_MW") || key_str.ends_with("_mw") {
                      contains_mw_key_indicating_reciever_of_fuel = true;
                    }
                  }
                  if let serde_pickle::value::Value::String(val_str) = v {
                    if val_str.ends_with("MW") || val_str.ends_with("mw") {
                      contains_mw_key_indicating_reciever_of_fuel = true;
                    }
                  }
                }
                if contains_mw_key_indicating_reciever_of_fuel {
                  // This looks like a plant turning fuel into electricity, thus it is a consumer of that resource
                  for (k,v) in attributes_map.iter() {
                    let v_string_lower = format!("{:?}", v).to_lowercase();
                    if v_string_lower.contains("gas") && v_string_lower.contains("process") && v_string_lower.contains("plant") {
                      // Is Some-sort of a natural gas plant, and we already know it produces electricity.
                      is_a_consumer = true;
                      product_type_s = "natural gas".to_string();
                    }
                    if v_string_lower.contains("petroleum") && v_string_lower.contains("power") && v_string_lower.contains("plant") {
                      // Is DEFINITELY a petroleum power plant!
                      is_a_consumer = true;
                      product_type_s = "petroleum".to_string();
                    }
                  }
                }

                // After the 2x attribute EP decision, do searches for wastewater and terminal plants
                if ! is_a_consumer {
                  for (k,v) in attributes_map.iter() {
                    let v_string_lower = format!("{:?}", v).to_lowercase();
                    if v_string_lower.contains("wastewater") && v_string_lower.contains("plant") {
                      // We assume wastewater plants intake petroleum
                      is_a_consumer = true;
                      product_type_s = "petroleum".to_string();
                    }
                    if v_string_lower.contains("petroleumproduct_terminals") {
                      // These are typically near airports, just big terminals unloading into smaller industrial things
                      is_a_consumer = true;
                      product_type_s = "petroleum".to_string();
                    }
                  }

                }

                // If we are labeled as a consumer of a product, read from some numbers to see how MUCH product is being consumed.
                // Note that EP facilities will generally have these as Megawatts, and so some math downstream needs to go back to product units.
                if is_a_consumer {
                  name_s = read_string_containing(&["Name", "name", "NAME", "Company"], attributes_map);
                  amount_thousand_barrels_per_day = read_number(&["Plant_Flow", "Total_MW", "NG_MW", "Crude_MW"], attributes_map);
                }

              }
            }

            if is_a_consumer {
              points.push( (*y_val, *x_val, name_s, product_type_s, amount_thousand_barrels_per_day) ); // lat, lon
            }

          }
        }
      }
      else {
        for (k,v) in map.iter() {
          let mut sub_points = get_all_consumers(v);
          points.append(&mut sub_points);
        }
      }
    }
    serde_pickle::Value::List(list) => {
      for v in list.iter() {
        let mut sub_points = get_all_consumers(v);
        points.append(&mut sub_points);
      }
    }
    _unused => {

    }
  }

  return points;
}



pub fn read_number(possible_names: &[&'static str], attribute_map: &std::collections::btree_map::BTreeMap<serde_pickle::value::HashableValue, serde_pickle::value::Value>) -> f64 {
  let val = 0.0;
  for name in possible_names.iter() {
    if *name == "!" {
      eprintln!("Cannot find any of {:?} in {:#?}", possible_names, attribute_map);
      panic!("Cannot find an attribute we expected!");
    }
    if let Some(val) = attribute_map.get( &serde_pickle::value::HashableValue::String((*name).into()) ) {
      match val {
        serde_pickle::value::Value::F64(as_f64) => {
          return *as_f64;
        }
        serde_pickle::value::Value::I64(as_i64) => {
          return (*as_i64) as f64;
        }
        serde_pickle::value::Value::Int(big_int) => {
          panic!("TODO implement transforming serde_pickle::value::Value::Int into 64 bits of data... somehow.");
        }
        _unused => {

        }
      }
    }
  }
  return val;
}


pub fn read_string(possible_names: &[&'static str], attribute_map: &std::collections::btree_map::BTreeMap<serde_pickle::value::HashableValue, serde_pickle::value::Value>) -> String {
  let val = String::new();
  for name in possible_names.iter() {
    if *name == "!" {
      eprintln!("Cannot find any of {:?} in {:#?}", possible_names, attribute_map);
      panic!("Cannot find an attribute we expected!");
    }
    if let Some(val) = attribute_map.get( &serde_pickle::value::HashableValue::String((*name).into()) ) {
      match val {
        serde_pickle::value::Value::F64(as_f64) => {
          return format!("{}", as_f64);
        }
        serde_pickle::value::Value::I64(as_i64) => {
          return format!("{}", as_i64);
        }
        serde_pickle::value::Value::Int(big_int) => {
          panic!("TODO implement transforming serde_pickle::value::Value::Int into 64 bits of data... somehow.");
        }
        serde_pickle::value::Value::String(string_val) => {
          return string_val.clone();
        }
        serde_pickle::value::Value::Bytes(bytes_val) => {
          return String::from_utf8_lossy(bytes_val).to_string();
        }
        _unused => {

        }
      }
    }
  }
  return val;
}


pub fn read_string_containing(possible_names: &[&'static str], attribute_map: &std::collections::btree_map::BTreeMap<serde_pickle::value::HashableValue, serde_pickle::value::Value>) -> String {
  let val = String::new();
  for (k,val) in attribute_map.iter() {
    let k_string = format!("{:?}", k);
    for name in possible_names.iter() {
      if k_string.contains(name) {
        match val {
          serde_pickle::value::Value::F64(as_f64) => {
            return format!("{}", as_f64);
          }
          serde_pickle::value::Value::I64(as_i64) => {
            return format!("{}", as_i64);
          }
          serde_pickle::value::Value::Int(big_int) => {
            panic!("TODO implement transforming serde_pickle::value::Value::Int into 64 bits of data... somehow.");
          }
          serde_pickle::value::Value::String(string_val) => {
            return string_val.clone();
          }
          serde_pickle::value::Value::Bytes(bytes_val) => {
            return String::from_utf8_lossy(bytes_val).to_string();
          }
          _unused => {

          }
        }
      }
    }
  }
  return val;
}




