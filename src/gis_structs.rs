
use gpkg::GPKGModel;

#[derive(gpkg::GPKGModel, Debug)]
pub struct DebugPoint {
  pub msg: String,
  #[geom_field("PointZ")]
  pub geom: gpkg::types::GPKGPointZ,
}
