#![forbid(unsafe_code)]

use cyrune_control_plane::ledger::{visible_working_hash, write_working_projection};
use cyrune_control_plane::memory::SourceLayer;
use cyrune_control_plane::working::{
    WorkingCandidate, WorkingCandidateCategory, WorkingRebuildInput, WorkingSlotKind,
    rebuild_working,
};
use cyrune_core_contract::CorrelationId;
use tempfile::tempdir;

fn working_output(
    text: &str,
    evidence_id: &str,
) -> cyrune_control_plane::working::WorkingRebuildOutput {
    let correlation_id = CorrelationId::parse("RUN-20260501-0201").unwrap();
    rebuild_working(&WorkingRebuildInput {
        generated_at: "2026-05-01T00:00:00Z".to_string(),
        correlation_id,
        prior_working: None,
        candidates: vec![WorkingCandidate {
            category: WorkingCandidateCategory::TurnResult,
            kind: WorkingSlotKind::Context,
            text: text.to_string(),
            source_evidence_id: evidence_id.to_string(),
            source_layer: SourceLayer::Processing,
            updated_at: "2026-05-01T00:00:00Z".to_string(),
            updated_at_unix_ms: 1_777_574_400_000,
        }],
    })
    .unwrap()
}

#[test]
fn visible_working_hash_matches_rebuilt_projection_hash() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let output = working_output("accepted first-success projection", "EVID-1");

    write_working_projection(&cyrune_home, &output.projection).unwrap();

    assert_eq!(
        visible_working_hash(&cyrune_home).unwrap(),
        output.working_hash
    );
}

#[test]
fn missing_working_projection_is_not_a_hash_match() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");

    assert!(visible_working_hash(&cyrune_home).is_err());
}

#[test]
fn mutated_visible_working_projection_changes_raw_hash() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let output = working_output("accepted first-success projection", "EVID-1");

    write_working_projection(&cyrune_home, &output.projection).unwrap();
    std::fs::write(
        cyrune_home.join("working").join("working.json"),
        b"{\"stale\":true}\n",
    )
    .unwrap();

    assert_ne!(
        visible_working_hash(&cyrune_home).unwrap(),
        output.working_hash
    );
}

#[test]
fn stale_working_projection_from_prior_turn_has_different_hash() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let stale = working_output("stale projection", "EVID-1");
    let current = working_output("current projection", "EVID-2");

    write_working_projection(&cyrune_home, &stale.projection).unwrap();

    assert_ne!(
        visible_working_hash(&cyrune_home).unwrap(),
        current.working_hash
    );
}
