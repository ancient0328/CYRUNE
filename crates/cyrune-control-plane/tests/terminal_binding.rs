#![forbid(unsafe_code)]

use cyrune_control_plane::ledger::{
    EvidenceOutcome, TERMINAL_BINDING_SCHEMA_VERSION, TerminalBindingRecord, raw_file_sha256,
    terminal_binding_path, write_terminal_binding,
};
use cyrune_core_contract::{CitationBundleId, CorrelationId, EvidenceId, RequestId, RunId};
use serde_json::Value;
use tempfile::tempdir;

fn marker_fixture(evidence_id: EvidenceId) -> TerminalBindingRecord {
    let correlation_id = CorrelationId::parse("RUN-20260501-0101").unwrap();
    TerminalBindingRecord {
        schema_version: TERMINAL_BINDING_SCHEMA_VERSION.to_string(),
        outcome: EvidenceOutcome::Accepted,
        response_to: RequestId::parse("REQ-20260501-0101").unwrap(),
        run_id: RunId::for_single_run(&correlation_id),
        evidence_id,
        policy_pack_id: "cyrune-free-default".to_string(),
        citation_bundle_id: CitationBundleId::from_correlation_id(&correlation_id),
        working_hash_after:
            "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string(),
        evidence_manifest_hash:
            "sha256:3333333333333333333333333333333333333333333333333333333333333333".to_string(),
        evidence_hashes_hash:
            "sha256:4444444444444444444444444444444444444444444444444444444444444444".to_string(),
        working_json_hash:
            "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string(),
        created_at: "2026-05-01T00:00:00Z".to_string(),
        correlation_id,
    }
}

#[test]
fn terminal_binding_marker_is_written_at_evidence_scoped_path() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let evidence_id = EvidenceId::new(1);
    let record = marker_fixture(evidence_id.clone());

    let written = write_terminal_binding(&cyrune_home, &record).unwrap();
    let expected = terminal_binding_path(&cyrune_home, &evidence_id);
    let marker: Value = serde_json::from_slice(&std::fs::read(&expected).unwrap()).unwrap();

    assert_eq!(written, expected);
    assert_eq!(marker["schema_version"], TERMINAL_BINDING_SCHEMA_VERSION);
    assert_eq!(marker["outcome"], "accepted");
    assert_eq!(marker["evidence_id"], evidence_id.as_str());
    assert_eq!(marker["working_hash_after"], marker["working_json_hash"]);
    assert!(raw_file_sha256(&expected).unwrap().starts_with("sha256:"));
}

#[test]
fn terminal_binding_missing_marker_is_absent_at_expected_path() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");

    assert!(!terminal_binding_path(&cyrune_home, &EvidenceId::new(2)).exists());
}

#[test]
fn terminal_binding_id_mismatch_is_detectable_from_marker_payload() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let expected_evidence_id = EvidenceId::new(3);
    let wrong_record = marker_fixture(EvidenceId::new(4));

    let written = write_terminal_binding(&cyrune_home, &wrong_record).unwrap();
    let marker: Value = serde_json::from_slice(&std::fs::read(written).unwrap()).unwrap();

    assert_ne!(marker["evidence_id"], expected_evidence_id.as_str());
}

#[test]
fn terminal_binding_hash_mismatch_is_detectable_from_marker_payload() {
    let temp = tempdir().unwrap();
    let cyrune_home = temp.path().join("home");
    let mut record = marker_fixture(EvidenceId::new(5));
    record.working_json_hash =
        "sha256:5555555555555555555555555555555555555555555555555555555555555555".to_string();

    let written = write_terminal_binding(&cyrune_home, &record).unwrap();
    let marker: Value = serde_json::from_slice(&std::fs::read(written).unwrap()).unwrap();

    assert_ne!(marker["working_hash_after"], marker["working_json_hash"]);
}
