
#![allow(
    unused_mut, dead_code, non_upper_case_globals, unused_variables,
    unreachable_code, unused_assignments, non_snake_case, unused_imports,
)]

mod data_mgmr;
mod structs;
mod gis_structs;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verbose_str = std::env::var("VERBOSE").unwrap_or("".into());
    let is_verbose = verbose_str.len() > 0;

    let args: Vec<String> = std::env::args().collect();
    // Grab data
    let data: serde_pickle::Value = if let Some(pickle_file) = args.get(1) {
        println!("Reading sea of raw data from {}", &pickle_file);
        let begin_t = std::time::SystemTime::now();
        let pickle_bytes: Vec<u8> = std::fs::read(pickle_file)?;
        let de_options = serde_pickle::DeOptions::new()
                            .decode_strings()
                            .replace_recursive_structures()
                            .replace_unresolved_globals();
        let data = serde_pickle::value_from_reader(pickle_bytes.as_slice(), de_options)?;
        print_time_since(&begin_t, format!("Time taken to read pickle file {}", &pickle_file ));
        data
    }
    else {
        return Err("Error: pass path to a .pickle file containing a dictionary of layers + GIS data within".into());
    };

    if verbose_str.contains("data") {
        println!("data = {:?}", data);
    }

    let assumptions: structs::AssumptionsToml = if let Some(assumptions_toml) = args.get(2) {
        println!("Reading assumptions from {}", &assumptions_toml);
        let assumptions_bytes: Vec<u8> = std::fs::read(assumptions_toml)?;
        let assumptions_string = String::from_utf8_lossy(&assumptions_bytes);
        toml::from_str(&assumptions_string)?
    }
    else {
        return Err("Error: pass path to an assumptions.toml file containing AoI assumptions and meta-data".into());
    };

    if is_verbose {
        println!("assumptions = {:#?}", assumptions);
    }

    let output_gpkg = "./data/output.gpkg".to_string();
    let output_gpkg = args.get(3).unwrap_or(&output_gpkg);
    println!("Writing Output to {}", output_gpkg);

    // Turn the data into modelable items; we construct
    //  - list of INPUT facilities
    //  - list of OUTPUT facilities
    //  - graph of pipelines, the INPUT / OUTPUT facility nearest and endpoint is recorded

    // We extract the point data w/ numbers like "<Product Type> Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)"
    // and use that as OUTPUT for that type at that location.

    let begin_t = std::time::SystemTime::now();
    let all_known_points = data_mgmr::get_all_points(&data);
    print_time_since(&begin_t, format!("Time taken to fetch {} items from the sea-of-layers", all_known_points.len() ));
    if is_verbose {
        eprintln!("all_known_points.len() = {}", all_known_points.len());
    }

    if is_verbose && std::path::Path::new(output_gpkg).exists() {
        println!("Verbose and {} exists, so we are deleting it first!", output_gpkg);
        std::fs::remove_file(output_gpkg)?;
    }

    // For now we use the output geopackage to place our logical network features + qgis to display them
    let mut gp = if !std::path::Path::new(output_gpkg).exists() {
        let gp = gpkg::GeoPackage::create(output_gpkg)?;
        gp.create_layer::<gis_structs::DebugPoint>()?;
        gp
    }
    else {
        gpkg::GeoPackage::open(output_gpkg)?
    };

    let begin_t = std::time::SystemTime::now();
    let mut debug_gis_records: Vec<gis_structs::DebugPoint> = vec![];
    for (lat_y, lon_x, debug_msg) in all_known_points.iter() {
        debug_gis_records.push(gis_structs::DebugPoint{
            msg: debug_msg.to_string(),
            geom: gpkg::types::GPKGPointZ { x: *lon_x, y: *lat_y, z: 0.0},
        });
    }
    print_time_since(&begin_t, format!("Time taken to convert {} in-memory to GIS records", debug_gis_records.len() ));

    let begin_t = std::time::SystemTime::now();
    gp.insert_many(&debug_gis_records)?;
    print_time_since(&begin_t, format!("Time taken to insert {} GIS records", debug_gis_records.len() ));

    /*
    let begin_t = std::time::SystemTime::now();
    let mut item_num = 0;
    for (lat_y, lon_x) in all_known_points.iter() {
        gp.insert_record(&gis_structs::DebugPoint{
            msg: format!("TODO"),
            geom: gpkg::types::GPKGPointZ { x: *lon_x, y: *lat_y, z: 0.0},
        })?;
        item_num += 1;
        if item_num % 1000 == 0 {
            let duration = std::time::SystemTime::now().duration_since(begin_t)?;
            let m = duration.as_secs() / 60;
            let s = duration.as_secs() - (m * 60);
            eprintln!("Have output {} / {} total! ({}m {}s elapsed)", item_num, all_known_points.len(), m, s);
        }
    }
    */


    Ok(())
}

pub fn print_time_since<S: AsRef<str> + std::fmt::Display>(begin_t: &std::time::SystemTime, message: S) {
    if let Ok(duration) = std::time::SystemTime::now().duration_since(*begin_t) {
        let m = duration.as_secs() / 60;
        let s = duration.as_secs() - (m * 60);
        let ms = duration.as_millis() - ((m * 60 * 1000) + (s * 1000)) as u128;
        eprintln!("{} ({}m {}s {}ms elapsed)", message, m, s, ms);
    }
}









