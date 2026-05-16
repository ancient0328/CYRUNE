#![forbid(unsafe_code)]

use cyrune_core_contract::{
    CitationBundleId, CorrelationId, DenialId, EvidenceId, ReasonKind, RequestId, RuleId,
    RunAccepted, RunId, RunOutcome, RunRejected,
};
use serde_json::{Value, json};

fn accepted_fixture() -> RunAccepted {
    let correlation_id = CorrelationId::parse("RUN-20260501-0001").unwrap();
    RunAccepted {
        outcome: RunOutcome::Accepted,
        response_to: RequestId::parse("REQ-20260501-0001").unwrap(),
        run_id: RunId::for_single_run(&correlation_id),
        evidence_id: EvidenceId::new(1),
        output: "ship-goal public first success".to_string(),
        citation_bundle_id: CitationBundleId::from_correlation_id(&correlation_id),
        working_hash_after:
            "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string(),
        policy_pack_id: "cyrune-free-default".to_string(),
        correlation_id,
    }
}

fn rejected_fixture() -> RunRejected {
    let correlation_id = CorrelationId::parse("RUN-20260501-0002").unwrap();
    let evidence_id = EvidenceId::new(2);
    RunRejected {
        outcome: RunOutcome::Rejected,
        response_to: RequestId::parse("REQ-20260501-0002").unwrap(),
        run_id: RunId::for_single_run(&correlation_id),
        denial_id: DenialId::from_evidence_id(&evidence_id),
        evidence_id,
        rule_id: RuleId::parse("POL-001").unwrap(),
        reason_kind: ReasonKind::PolicyDenied,
        message: "policy denied".to_string(),
        remediation: "fix policy input".to_string(),
        correlation_id,
    }
}

#[test]
fn accepted_payload_requires_explicit_accepted_outcome() {
    let accepted = accepted_fixture();
    let serialized = serde_json::to_value(&accepted).unwrap();

    assert_eq!(serialized["outcome"], "accepted");
    assert_eq!(
        serde_json::from_value::<RunAccepted>(serialized).unwrap(),
        accepted
    );
}

#[test]
fn rejected_payload_requires_explicit_rejected_outcome() {
    let rejected = rejected_fixture();
    let serialized = serde_json::to_value(&rejected).unwrap();

    assert_eq!(serialized["outcome"], "rejected");
    assert_eq!(
        serde_json::from_value::<RunRejected>(serialized).unwrap(),
        rejected
    );
}

#[test]
fn accepted_field_set_without_outcome_is_not_a_run_accepted_payload() {
    let mut serialized = serde_json::to_value(accepted_fixture()).unwrap();
    serialized.as_object_mut().unwrap().remove("outcome");

    assert!(serde_json::from_value::<RunAccepted>(serialized).is_err());
}

#[test]
fn rejected_field_set_without_outcome_is_not_a_run_rejected_payload() {
    let mut serialized = serde_json::to_value(rejected_fixture()).unwrap();
    serialized.as_object_mut().unwrap().remove("outcome");

    assert!(serde_json::from_value::<RunRejected>(serialized).is_err());
}

#[test]
fn accepted_payload_with_rejected_outcome_is_not_accepted() {
    let mut serialized = serde_json::to_value(accepted_fixture()).unwrap();
    serialized["outcome"] = Value::String("rejected".to_string());

    assert_ne!(serialized["outcome"], json!("accepted"));
    assert_eq!(
        serde_json::from_value::<RunAccepted>(serialized)
            .unwrap()
            .outcome,
        RunOutcome::Rejected
    );
}
