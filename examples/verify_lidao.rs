use porc_index_core::PorcEvaluator;
use std::fs;

fn main() {
    println!("============================================================");
    println!("PoRC-SCS Evaluation Engine v1.0.0 (Rust Core)");
    println!("Target Node: ONYX-GLB-REORG-LIDAO-01 (Taitung, Taiwan)");
    println!("============================================================");

    let json_data = fs::read_to_string("examples/lidao_tier0_sample.json")
        .expect("Unable to read lidao_tier0_sample.json benchmark file");

    let evaluator = PorcEvaluator::from_json(&json_data)
        .expect("Failed to parse telemetry JSON");

    let (c, t, r, phi) = evaluator.calculate_sub_indices();
    let (score, tier) = evaluator.compute_index();

    println!("[Compute Sub-Index (C)]:       {:.4}", c);
    println!("[Thermodynamic Sub-Index (T)]: {:.4}", t);
    println!("[Resilience Sub-Index (R)]:    {:.4}", r);
    println!("------------------------------------------------------------");
    println!("Penalty Factor (Phi):          {:.4}", phi);
    println!("FINAL PoRC INDEX SCORE:        {:.2} / 100.00", score);
    println!("RATING CLASSIFICATION:         {}", tier);
    println!("============================================================");

    let expected_score = 88.94;
    assert!(
        (score - expected_score).abs() < 0.01,
        "Verification Failed: Score {} does not match benchmark 88.94!", score
    );
    println!("STATUS: VERIFIED TIER-0 SOVEREIGN BENCHMARK PASS!");
}
