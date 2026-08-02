use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeTelemetry {
pub flops_per_watt_effective: f64,
pub flops_per_watt_baseline: f64,
pub physical_distance_meters: f64,
pub cfe_alignment_24_7_ratio: f64,
}

pub struct ComputeSubIndex;

impl ComputeSubIndex {
pub fn calculate(telemetry: &ComputeTelemetry) -> f64 {
let c_d = (telemetry.flops_per_watt_effective / telemetry.flops_per_watt_baseline).clamp(0.0, 1.0);
let l_p = if telemetry.physical_distance_meters <= 0.0 {
0.0
} else if telemetry.physical_distance_meters >= 15.0 {
1.0
} else {
telemetry.physical_distance_meters / 15.0
};
let a_24_7 = telemetry.cfe_alignment_24_7_ratio.clamp(0.0, 1.0);
0.35 * c_d + 0.30 * (1.0 - l_p) + 0.35 * a_24_7
}
}
