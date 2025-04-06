
use gpkg::GPKGModel;

#[derive(gpkg::GPKGModel, Debug)]
pub struct DebugPoint {
  pub msg: String,
  #[geom_field("PointZ")]
  pub geom: gpkg::types::GPKGPointZ,
}


#[derive(gpkg::GPKGModel, Debug)]
pub struct DebugLine {
  pub msg: String,
  #[geom_field("LineString")]
  pub geom: gpkg::types::GPKGLineString,
}


#[derive(gpkg::GPKGModel, Debug)]
pub struct ProductionPoint {
  pub facility_name: String,
  pub product_name: String,
  pub quantity_thousand_barrels_per_day: f64,
  #[geom_field("PointZ")]
  pub geom: gpkg::types::GPKGPointZ,
}


#[derive(gpkg::GPKGModel, Debug)]
pub struct ConsumptionPoint {
  pub facility_name: String,
  pub product_name: String,
  pub quantity_thousand_barrels_per_day: f64,
  #[geom_field("PointZ")]
  pub geom: gpkg::types::GPKGPointZ,
}
