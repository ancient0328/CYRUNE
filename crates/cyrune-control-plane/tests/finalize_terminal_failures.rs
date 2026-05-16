#![forbid(unsafe_code)]

use cyrune_control_plane::citation::{
    CitationMaterial, CitationMaterialClaim, ClaimKind, EvidenceRef, SimpleReasoningRecord,
};
use cyrune_control_plane::ledger::{LedgerWriter, terminal_binding_path};
use cyrune_control_plane::memory::SourceLayer;
use cyrune_control_plane::resolved_turn_context::{
    ResolvedKernelAdapters, ResolvedTurnContext, TimeoutPolicy,
};
use cyrune_control_plane::retrieval::QuerySummary;
use cyrune_control_plane::turn::{AcceptedTurnDraft, finalize_accepted_turn};
use cyrune_control_plane::working::{
    WorkingCandidate, WorkingCandidateCategory, WorkingRebuildInput, WorkingSlotKind,
    rebuild_working,
};
use cyrune_core_contract::{CorrelationId, EvidenceId, IoMode, RequestId, RunKind, RunRequest};
use tempfile::tempdir;

fn request() -> RunRequest {
    RunRequest {
        request_id: RequestId::parse("REQ-20260501-0401").unwrap(),
        correlation_id: CorrelationId::parse("RUN-20260501-0401").unwrap(),
        run_kind: RunKind::NoLlm,
        user_input: "turn".to_string(),
        policy_pack_id: "cyrune-free-default".to_string(),
        binding_id: None,
        requested_capabilities: vec!["fs_read".to_string()],
        io_mode: IoMode::Captured,
        adapter_id: None,
        argv: None,
        cwd: None,
        env_overrides: None,
    }
}

fn context() -> ResolvedTurnContext {
    ResolvedTurnContext {
        version: 1,
        request_id: RequestId::parse("REQ-20260501-0401").unwrap(),
        correlation_id: CorrelationId::parse("RUN-20260501-0401").unwrap(),
        run_id: cyrune_core_contract::RunId::parse("RUN-20260501-0401-R01").unwrap(),
        requested_policy_pack_id: "cyrune-free-default".to_string(),
        requested_binding_id: None,
        policy_pack_id: "cyrune-free-default".to_string(),
        binding_id: "cyrune-free-default".to_string(),
        resolved_kernel_adapters: ResolvedKernelAdapters {
            working_store_adapter_id: "memory-kv-inmem".to_string(),
            processing_store_adapter_id: "memory-kv-inmem".to_string(),
            permanent_store_adapter_id: "memory-kv-inmem".to_string(),
            vector_index_adapter_id: "memory-kv-inmem".to_string(),
            embedding_engine_ref: "crane-embed-null.v0.1".to_string(),
        },
        embedding_exact_pin: None,
        memory_state_roots: None,
        allowed_capabilities: vec!["fs_read".to_string()],
        sandbox_ref: "SANDBOX_MINIMAL_CANONICAL.md#default-profile".to_string(),
        run_kind: RunKind::NoLlm,
        io_mode: IoMode::Captured,
        selected_execution_adapter: None,
        timeout_policy: TimeoutPolicy {
            turn_timeout_s: 120,
            execution_timeout_s: 120,
        },
    }
}

fn working_output() -> cyrune_control_plane::working::WorkingRebuildOutput {
    rebuild_working(&WorkingRebuildInput {
        generated_at: "2026-05-01T00:00:00Z".to_string(),
        correlation_id: CorrelationId::parse("RUN-20260501-0401").unwrap(),
        prior_working: None,
        candidates: vec![WorkingCandidate {
            category: WorkingCandidateCategory::PolicyConstraint,
            kind: WorkingSlotKind::Constraint,
            text: "keep deterministic".to_string(),
            source_evidence_id: "EVID-1".to_string(),
            source_layer: SourceLayer::Processing,
            updated_at: "2026-05-01T00:00:00Z".to_string(),
            updated_at_unix_ms: 1_777_574_400_000,
        }],
    })
    .unwrap()
}

fn accepted_draft() -> AcceptedTurnDraft {
    AcceptedTurnDraft {
        request: request(),
        context: context(),
        created_at: "2026-05-01T00:00:01Z".to_string(),
        started_at: "2026-05-01T00:00:00Z".to_string(),
        finished_at: "2026-05-01T00:00:01Z".to_string(),
        exit_status: Some(0),
        working_hash_before:
            "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        prior_working: None,
        working_output: working_output(),
        query_summary: QuerySummary {
            query_hash: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                .to_string(),
            selected_memory_ids: vec!["MEM-1".to_string()],
            rejected_reasons: Vec::new(),
        },
        output_draft: "- claim".to_string(),
        citation_material: CitationMaterial {
            claims: vec![CitationMaterialClaim {
                text: "claim".to_string(),
                claim_kind: ClaimKind::Extractive,
                evidence_refs: vec![EvidenceRef::new("EVID-1".to_string())],
            }],
        },
        rr_material: SimpleReasoningRecord {
            claims: vec!["claim".to_string()],
            decisions: Vec::new(),
            assumptions: Vec::new(),
            actions: Vec::new(),
            citations_used: vec!["EVID-1".to_string()],
        },
        stdout: String::new(),
        stderr: String::new(),
    }
}

#[test]
fn accepted_finalization_writes_terminal_marker_for_returned_evidence() {
    let temp = tempdir().unwrap();
    let mut writer = LedgerWriter::new(temp.path());

    let accepted = finalize_accepted_turn(&mut writer, accepted_draft())
        .unwrap()
        .unwrap();

    assert_eq!(accepted.outcome, cyrune_core_contract::RunOutcome::Accepted);
    assert!(terminal_binding_path(temp.path(), &accepted.evidence_id).exists());
    assert_eq!(accepted.evidence_id.as_str(), "EVID-1");
}

#[test]
fn working_update_failure_rejects_without_terminal_marker_for_accepted_evidence() {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("working").join("working.json")).unwrap();
    let mut writer = LedgerWriter::new(temp.path());

    let rejected = finalize_accepted_turn(&mut writer, accepted_draft())
        .unwrap()
        .unwrap_err();

    assert_eq!(rejected.outcome, cyrune_core_contract::RunOutcome::Rejected);
    assert_eq!(rejected.rule_id.as_str(), "WUP-001");
    assert!(!terminal_binding_path(temp.path(), &EvidenceId::new(1)).exists());
}

#[test]
fn working_hash_mismatch_rejects_without_terminal_marker_for_accepted_evidence() {
    let temp = tempdir().unwrap();
    let mut draft = accepted_draft();
    draft.working_output.working_hash =
        "sha256:9999999999999999999999999999999999999999999999999999999999999999".to_string();
    let mut writer = LedgerWriter::new(temp.path());

    let rejected = finalize_accepted_turn(&mut writer, draft)
        .unwrap()
        .unwrap_err();

    assert_eq!(rejected.outcome, cyrune_core_contract::RunOutcome::Rejected);
    assert_eq!(rejected.rule_id.as_str(), "WUP-002");
    assert!(!terminal_binding_path(temp.path(), &EvidenceId::new(1)).exists());
}

#[test]
fn terminal_binding_write_failure_rejects_without_terminal_marker() {
    let temp = tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("ledger")).unwrap();
    std::fs::write(
        temp.path().join("ledger").join("terminal-bindings"),
        b"not-a-dir",
    )
    .unwrap();
    let mut writer = LedgerWriter::new(temp.path());

    let rejected = finalize_accepted_turn(&mut writer, accepted_draft())
        .unwrap()
        .unwrap_err();

    assert_eq!(rejected.outcome, cyrune_core_contract::RunOutcome::Rejected);
    assert_eq!(rejected.rule_id.as_str(), "LDG-002");
    assert!(!terminal_binding_path(temp.path(), &EvidenceId::new(1)).exists());
}

#[test]
fn ledger_commit_failure_rejects_without_accepted_terminal_marker() {
    let temp = tempdir().unwrap();
    let mut writer = LedgerWriter::with_failures(temp.path(), 1);

    let rejected = finalize_accepted_turn(&mut writer, accepted_draft())
        .unwrap()
        .unwrap_err();

    assert_eq!(rejected.outcome, cyrune_core_contract::RunOutcome::Rejected);
    assert_eq!(rejected.rule_id.as_str(), "LDG-001");
    assert!(!terminal_binding_path(temp.path(), &EvidenceId::new(1)).exists());
}
