use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalTelemetry {
pub waste_heat_reused_watts: f64,
pub waste_heat_total_watts: f64,
pub hydrothermal_exergy_efficiency: f64,
pub local_symbiosis_score: f64,
}

pub struct ThermalSubIndex;

impl ThermalSubIndex {
pub fn calculate(telemetry: &ThermalTelemetry) -> f64 {
let eta_thermal = if telemetry.waste_heat_total_watts > 0.0 {
(telemetry.waste_heat_reused_watts / telemetry.waste_heat_total_watts).clamp(0.0, 1.0)
} else {
0.0
};
let h_reaction = telemetry.hydrothermal_exergy_efficiency.clamp(0.0, 1.0);
let s_local = telemetry.local_symbiosis_score.clamp(0.0, 1.0);
0.40 * eta_thermal + 0.30 * h_reaction + 0.30 * s_local
}
}
