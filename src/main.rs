
#![allow(
    unused_mut, dead_code, non_upper_case_globals, unused_variables,
    unreachable_code, unused_assignments,
)]

mod data_mgmr;
mod structs;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verbose_str = std::env::var("VERBOSE").unwrap_or("".into());
    let is_verbose = verbose_str.len() > 0;

    let args: Vec<String> = std::env::args().collect();
    // Grab data
    let data: serde_pickle::Value = if let Some(pickle_file) = args.get(1) {
        println!("Reading sea of raw data from {}", &pickle_file);
        let pickle_bytes: Vec<u8> = std::fs::read(pickle_file)?;
        let de_options = serde_pickle::DeOptions::new()
                            .decode_strings()
                            .replace_recursive_structures()
                            .replace_unresolved_globals();
        serde_pickle::value_from_reader(pickle_bytes.as_slice(), de_options)?
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

    // Turn the data into modelable items; we construct
    //  - list of INPUT facilities
    //  - list of OUTPUT facilities
    //  - graph of pipelines, the INPUT / OUTPUT facility nearest and endpoint is recorded

    // We extract the point data w/ numbers like "<Product Type> Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)"
    // and use that as OUTPUT for that type at that location.



    // Build spatial indexes
    let aoi_center = galileo_types::geo::impls::GeoPoint2d::latlon(
        (assumptions.aoi_min_map_pt.0 + assumptions.aoi_max_map_pt.0) / 2.0,
        (assumptions.aoi_min_map_pt.1 + assumptions.aoi_max_map_pt.1) / 2.0,
    );
    let map_zoom_resolution = 14000.0;

    // Finally render a map w/ results!
    use galileo_types::geo::NewGeoPoint;
    galileo_egui::init(galileo::MapBuilder::default()
        .with_position(aoi_center)
        .with_resolution(map_zoom_resolution)
        .with_layer(
            galileo::layer::raster_tile_layer::RasterTileLayerBuilder::new_rest(
                |index| {
                /*format!(
                    "https://tile.openstreetmap.org/{}/{}/{}.png",
                    index.z, index.x, index.y
                )*/
                let mut q = String::new();
                for i in (1..index.z+1).rev() {
                    let mut digit = 0;
                    let mask = 1 << (i - 1);
                    if index.x & mask != 0 {
                        digit += 1;
                    }
                    if index.y & mask != 0 {
                        digit += 2;
                    }
                    q = format!("{q}{digit}");
                }
                format!(
                    "http://ecn.t3.tiles.virtualearth.net/tiles/a{q}.jpeg?g=1",
                    q=q
                )
            }).build()?
        )
        .with_layer(galileo::layer::FeatureLayer::new(
            vec![ galileo_types::geo::impls::GeoPoint2d::latlon(38.344335, -77.571676) ],
            galileo::symbol::CirclePointSymbol::new(galileo::Color::BLUE, 5.0),
            galileo_types::geo::Crs::WGS84,
        ))
        .build(), [])?;

    Ok(())
}











