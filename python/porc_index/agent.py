import json

try:
from ._native import compute_porc_index_py
HAS_NATIVE = True
except ImportError:
HAS_NATIVE = False

class PorcEvaluator:
def init(self, telemetry_data: dict):
self.telemetry_data = telemetry_data
self.json_str = json.dumps(telemetry_data)

def compute_index(self) -> tuple[float, str]:
    if HAS_NATIVE:
        score, tier, _, _, _, _ = compute_porc_index_py(self.json_str)
        return score, tier
    
    t = self.telemetry_data["telemetry"]
    f = self.telemetry_data["flags"]

    c_d = min(1.0, t["compute"]["flops_per_watt_effective"] / t["compute"]["flops_per_watt_baseline"])
    l_p = 0.0 if t["compute"]["physical_distance_meters"] <= 0 else min(1.0, t["compute"]["physical_distance_meters"] / 15.0)
    a_24_7 = max(0.0, min(1.0, t["compute"]["cfe_alignment_24_7_ratio"]))
    c = 0.35 * c_d + 0.30 * (1.0 - l_p) + 0.35 * a_24_7

    eta = t["thermodynamic"]["waste_heat_reused_watts"] / t["thermodynamic"]["waste_heat_total_watts"]
    h_reac = t["thermodynamic"]["hydrothermal_exergy_efficiency"]
    s_loc = t["thermodynamic"]["local_symbiosis_score"]
    thermal_val = 0.40 * eta + 0.30 * h_reac + 0.30 * s_loc

    r_auto = min(1.0, t["resilience"]["offgrid_autonomy_hours"] / 720.0)
    r_cont = t["resilience"]["disaster_compute_continuity_ratio"]
    cm = t["resilience"]["comms_matrix"]
    r_comms = 0.35 * cm["mesh_coverage"] + 0.35 * cm["leo_satellite_link"] + 0.30 * cm["edge_ai_broadcast"]
    r = 0.35 * r_auto + 0.35 * r_cont + 0.30 * r_comms

    phi = 0.85 if f["has_fossil_backup"] or f["has_single_point_of_failure"] else 1.0

    score = round(100.0 * (c**0.35 * thermal_val**0.30 * r**0.35) * phi, 2)

    if score >= 85.0:
        tier = "Tier-0 Sovereign Class"
    elif score >= 70.0:
        tier = "Tier-1 Resilient Class"
    elif score >= 50.0:
        tier = "Tier-2 Co-located Class"
    else:
        tier = "Legacy Infrastructure Class"

    return score, tier
