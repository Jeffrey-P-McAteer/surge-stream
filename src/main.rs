
#![allow(
    unused_mut, dead_code, non_upper_case_globals, unused_variables,
    unreachable_code,
)]

mod data_mgmr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    // Grab data
    let data: serde_pickle::Value;
    if let Some(pickle_file) = args.get(1) {
        let pickle_bytes: Vec<u8> = std::fs::read(pickle_file)?;
        let de_options = serde_pickle::DeOptions::new()
                            .decode_strings()
                            .replace_recursive_structures()
                            .replace_unresolved_globals();
        data = serde_pickle::value_from_reader(pickle_bytes.as_slice(), de_options)?;
    }
    else {
        println!("Warning: Generating RANDOM gis data. Pass a path to pickled data to use real GIS data (eg \"data/raw-layer-data.pickle\" as arg1)");
        println!("         Pass SEED_NONCE to set repeadable random data.");
        data = data_mgmr::gen_rand_raw_layer_data();
    };

    println!("data = {:#?}", data);

    // We record some "entire system" assumptions here.
    // These particular numbers came from:
    //  - https://www.eia.gov/international/data/world/petroleum-and-other-liquids/annual-petroleum-and-other-liquids-production?pd=5&p=0000001&u=0&f=A&v=heatmap&a=-&i=none&vo=value&vb=34&t=C&g=00000000000000000000000000000000000000000000000001&l=249-ruvvvvvfvtvnvv1vrvvvvfvvvvvvfvvvou20evvvvvvvvvvnvvvs0008&s=94694400000&e=1609459200000&ev=true
    //  - https://www.eia.gov/international/data/world/petroleum-and-other-liquids/annual-petroleum-and-other-liquids-production?pd=5&p=0000000000000000000000000000000000g&u=0&f=A&v=heatmap&a=-&i=none&vo=value&vb=170&t=C&g=00000000000000000000000000000000000000000000000001&l=249-ruvvvvvfvtvnvv1vrvvvvfvvvvvvfvvvou20evvvvvvvvvvnvvvs0008&s=94694400000&e=1609459200000&ev=true
    const aoi_petroleum_annual_production_thousand_barrels_per_day: f64 = 19_036.0;
    const aoi_petroleum_annual_consumption_thousand_barrels_per_day: f64 = 19_890.0;

    // If the import is negative, AoI is net exporter. Math does not change.
    let aoi_imported_petroleum_thousand_barrels_per_day = aoi_petroleum_annual_consumption_thousand_barrels_per_day - aoi_petroleum_annual_production_thousand_barrels_per_day;
    let aoi_imported_petroleum_percent_of_cosumption = 100.0 * (aoi_imported_petroleum_thousand_barrels_per_day / aoi_petroleum_annual_consumption_thousand_barrels_per_day).abs();
    if aoi_imported_petroleum_thousand_barrels_per_day > 0.0 {
        println!("The AoI is a net Importer of petroleum; {} thousand barrels/day are imported. ({:.2}% of total consumption)",
            aoi_imported_petroleum_thousand_barrels_per_day.abs(),
            aoi_imported_petroleum_percent_of_cosumption
        );
    }
    else {
        println!("The AoI is a net Exporter of petroleum; {} thousand barrels/day are exported. ({:.2}% of total consumption)",
            aoi_imported_petroleum_thousand_barrels_per_day.abs(),
            aoi_imported_petroleum_percent_of_cosumption
        );
    }

    // Turn the data into modelable items; we construct
    //  - list of INPUT facilities
    //  - list of OUTPUT facilities
    //  - graph of pipelines, the INPUT / OUTPUT facility nearest and endpoint is recorded

    // We extract the point data w/ numbers like "<Product Type> Downstream Charge Capacity, Current Year (Barrels Per Calendar Day)"
    // and use that as OUTPUT for that type at that location.



    // Build spatial indexes



    Ok(())
}
