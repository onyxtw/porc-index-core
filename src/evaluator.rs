use crate::compute::{ComputeSubIndex, ComputeTelemetry};
use crate::resilience::{ResilienceSubIndex, ResilienceTelemetry};
use crate::thermal::{ThermalSubIndex, ThermalTelemetry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFlags {
pub has_fossil_backup: bool,
pub has_single_point_of_failure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
pub compute: ComputeTelemetry,
pub thermodynamic: ThermalTelemetry,
pub resilience: ResilienceTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTelemetryInput {
pub node_id: String,
pub timestamp: String,
pub facility_class_target: String,
pub telemetry: TelemetryData,
pub flags: NodeFlags,
}

pub struct PorcEvaluator {
pub input: NodeTelemetryInput,
}

impl PorcEvaluator {
pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
let input: NodeTelemetryInput = serde_json::from_str(json_str)?;
Ok(Self { input })
}

pub fn calculate_sub_indices(&self) -> (f64, f64, f64, f64) {
    let c = ComputeSubIndex::calculate(&self.input.telemetry.compute);
    let t = ThermalSubIndex::calculate(&self.input.telemetry.thermodynamic);
    let r = ResilienceSubIndex::calculate(&self.input.telemetry.resilience);

    let phi_penalty = if self.input.flags.has_fossil_backup
        || self.input.flags.has_single_point_of_failure
    {
        0.85
    } else {
        1.0
    };

    (c, t, r, phi_penalty)
}

pub fn compute_index(&self) -> (f64, String) {
    let (c, t, r, phi_penalty) = self.calculate_sub_indices();
    let raw_score = 100.0 * (c.powf(0.35) * t.powf(0.30) * r.powf(0.35)) * phi_penalty;
    let score = (raw_score * 100.0).round() / 100.0;

    let classification = match score {
        s if s >= 85.0 => "Tier-0 Sovereign Class",
        s if s >= 70.0 => "Tier-1 Resilient Class",
        s if s >= 50.0 => "Tier-2 Co-located Class",
        _ => "Legacy Infrastructure Class",
    }
    .to_string();

    (score, classification)
}
}
