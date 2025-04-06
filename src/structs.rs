
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AssumptionsToml {
  pub aoi_name: String,

  pub aoi_min_map_pt: (f64, f64), // MinX, MinY
  pub aoi_max_map_pt: (f64, f64), // MaxX, MaxY

  pub aoi_annual_production: Vec<MeasuredProductAmount_AssumptionsFormat>,
  pub aoi_annual_consumption: Vec<MeasuredProductAmount_AssumptionsFormat>,
}

#[allow(non_camel_case_types)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MeasuredProductAmount_AssumptionsFormat {
  pub product: String,
  pub thousand_barrels_per_day: f64,
}


pub type ProductID = u32;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Product {
  pub id: ProductID,
  pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MeasuredProductAmount {
  pub product: ProductID,
  pub thousand_barrels_per_day: f64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ConvertedProductCapability {
  pub input_product: ProductID,
  pub input_thousand_barrels_per_day: f64,
  pub output_multiplier: f64, // if 1.0, output is perfect. If 0.5, for every 1000 barrels in 500 come out.
  pub output_product: ProductID,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProductProducer {
  pub name: String,
  pub map_position: (f64, f64),
  pub products: Vec<MeasuredProductAmount>, // How much is produced/day of operation?
}


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProductConverter {
  pub name: String,
  pub map_position: (f64, f64),
  pub products: Vec<ConvertedProductCapability>, // How much is converted/day of operation?
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProductConsumer {
  pub name: String,
  pub map_position: (f64, f64),
  pub products: Vec<MeasuredProductAmount>, // How much is consumed/day of operation?
}

// This models eg an oil pipeline and makes endpoints + types easy to query
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ProductFlow {
  pub product: ProductID,
  pub max_capacity_thousand_barrels_per_day: f64,
  pub input_map_position: (f64, f64),
  pub output_map_position: (f64, f64),
  pub bidirectional: bool,
  pub map_path: Vec<(f64, f64)>,
}









