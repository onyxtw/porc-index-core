use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsMatrix {
pub mesh_coverage: f64,
pub leo_satellite_link: f64,
pub edge_ai_broadcast: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceTelemetry {
pub offgrid_autonomy_hours: f64,
pub disaster_compute_continuity_ratio: f64,
pub comms_matrix: CommsMatrix,
}

pub struct ResilienceSubIndex;

impl ResilienceSubIndex {
pub fn calculate(telemetry: &ResilienceTelemetry) -> f64 {
let r_autonomy = (telemetry.offgrid_autonomy_hours / 720.0).clamp(0.0, 1.0);
let r_continuity = telemetry.disaster_compute_continuity_ratio.clamp(0.0, 1.0);
let r_comms = (0.35 * telemetry.comms_matrix.mesh_coverage
+ 0.35 * telemetry.comms_matrix.leo_satellite_link
+ 0.30 * telemetry.comms_matrix.edge_ai_broadcast)
.clamp(0.0, 1.0);
0.35 * r_autonomy + 0.35 * r_continuity + 0.30 * r_comms
}
}
