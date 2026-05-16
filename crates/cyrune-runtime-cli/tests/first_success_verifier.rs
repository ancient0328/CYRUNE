#![forbid(unsafe_code)]

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::{TempDir, tempdir};

const REQUEST_ID: &str = "REQ-20260501-0301";
const CORRELATION_ID: &str = "RUN-20260501-0301";
const RUN_ID: &str = "RUN-20260501-0301-R01";
const EVIDENCE_ID: &str = "EVID-1";
const POLICY_PACK_ID: &str = "cyrune-free-default";
const CITATION_BUNDLE_ID: &str = "CB-20260501-0301";
const TERMINAL_SCHEMA_VERSION: &str = "cyrune.free.terminal-binding.v1";

struct FixtureState {
    _temp: TempDir,
    state_root: PathBuf,
    cyrune_home: PathBuf,
    daemon_bin: PathBuf,
    payload_path: PathBuf,
    evidence_dir: PathBuf,
    terminal_marker: PathBuf,
}

impl FixtureState {
    fn run_verifier(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_cyrune-runtime-cli"))
            .arg("verify")
            .arg("first-success")
            .env("CYRUNE_HOME", &self.cyrune_home)
            .env("CYRUNE_DAEMON_BIN", &self.daemon_bin)
            .env("CYRUNE_FAKE_PAYLOAD", &self.payload_path)
            .output()
            .unwrap()
    }

    fn write_payload(&self, payload: &Value) {
        write_json(&self.payload_path, payload);
    }
}

fn valid_fixture() -> FixtureState {
    let temp = tempdir().unwrap();
    let state_root = temp.path().join("state");
    let cyrune_home = state_root.join("home");
    let evidence_dir = cyrune_home
        .join("ledger")
        .join("evidence")
        .join(EVIDENCE_ID);
    let terminal_dir = cyrune_home.join("ledger").join("terminal-bindings");
    let working_dir = cyrune_home.join("working");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::create_dir_all(&terminal_dir).unwrap();
    fs::create_dir_all(&working_dir).unwrap();

    let working_hash = write_json(
        &working_dir.join("working.json"),
        &json!({
            "version": 1,
            "generated_at": "2026-05-01T00:00:00Z",
            "correlation_id": CORRELATION_ID,
            "limit": 12,
            "slots": []
        }),
    );
    let manifest_hash = write_json(
        &evidence_dir.join("manifest.json"),
        &json!({
            "evidence_id": EVIDENCE_ID,
            "correlation_id": CORRELATION_ID,
            "run_id": RUN_ID,
            "outcome": "accepted",
            "created_at": "2026-05-01T00:00:00Z",
            "policy_pack_id": POLICY_PACK_ID,
            "working_hash_before": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
            "working_hash_after": working_hash,
            "citation_bundle_id": CITATION_BUNDLE_ID,
            "rr_present": true
        }),
    );
    write_json(
        &evidence_dir.join("run.json"),
        &json!({
            "request_id": REQUEST_ID,
            "correlation_id": CORRELATION_ID,
            "run_id": RUN_ID
        }),
    );
    write_json(
        &evidence_dir.join("policy.json"),
        &json!({"policy_pack_id": POLICY_PACK_ID}),
    );
    write_json(
        &evidence_dir.join("citation_bundle.json"),
        &json!({"bundle_id": CITATION_BUNDLE_ID}),
    );
    write_json(
        &evidence_dir.join("rr.json"),
        &json!({"reasoning_record": "present"}),
    );
    write_json(
        &evidence_dir.join("working_delta.json"),
        &json!({
            "correlation_id": CORRELATION_ID,
            "added_slots": [],
            "removed_slots": [],
            "resulting_hash": working_hash
        }),
    );
    write_text(&evidence_dir.join("stdout.log"), "");
    write_text(&evidence_dir.join("stderr.log"), "");
    let hashes_hash = write_hashes(&evidence_dir);

    let terminal_marker = terminal_dir.join(format!("{EVIDENCE_ID}.json"));
    write_json(
        &terminal_marker,
        &json!({
            "schema_version": TERMINAL_SCHEMA_VERSION,
            "outcome": "accepted",
            "response_to": REQUEST_ID,
            "correlation_id": CORRELATION_ID,
            "run_id": RUN_ID,
            "evidence_id": EVIDENCE_ID,
            "policy_pack_id": POLICY_PACK_ID,
            "citation_bundle_id": CITATION_BUNDLE_ID,
            "working_hash_after": working_hash,
            "evidence_manifest_hash": manifest_hash,
            "evidence_hashes_hash": hashes_hash,
            "working_json_hash": working_hash,
            "created_at": "2026-05-01T00:00:00Z"
        }),
    );

    let daemon_bin = temp.path().join("fake-daemon.py");
    write_fake_daemon(&daemon_bin);
    let payload_path = temp.path().join("daemon-payload.json");
    let state = FixtureState {
        _temp: temp,
        state_root,
        cyrune_home,
        daemon_bin,
        payload_path,
        evidence_dir,
        terminal_marker,
    };
    state.write_payload(&accepted_payload(&working_hash));
    state
}

fn accepted_payload(working_hash: &str) -> Value {
    json!({
        "outcome": "accepted",
        "response_to": REQUEST_ID,
        "correlation_id": CORRELATION_ID,
        "run_id": RUN_ID,
        "evidence_id": EVIDENCE_ID,
        "output": "ship-goal public first success",
        "citation_bundle_id": CITATION_BUNDLE_ID,
        "working_hash_after": working_hash,
        "policy_pack_id": POLICY_PACK_ID
    })
}

fn write_hashes(evidence_dir: &Path) -> String {
    let mut files = BTreeMap::new();
    for file_name in [
        "manifest.json",
        "run.json",
        "policy.json",
        "citation_bundle.json",
        "rr.json",
        "working_delta.json",
        "stdout.log",
        "stderr.log",
    ] {
        files.insert(
            file_name.to_string(),
            raw_file_sha256(&evidence_dir.join(file_name)),
        );
    }
    write_json(
        &evidence_dir.join("hashes.json"),
        &json!({
            "files": files,
            "prev_evidence_id": null,
            "prev_hash": null
        }),
    )
}

fn write_json(path: &Path, value: &Value) -> String {
    let mut bytes = serde_json::to_vec_pretty(value).unwrap();
    bytes.push(b'\n');
    fs::write(path, &bytes).unwrap();
    sha256_bytes(&bytes)
}

fn write_text(path: &Path, value: &str) -> String {
    let mut text = value.replace("\r\n", "\n").replace('\r', "\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    fs::write(path, text.as_bytes()).unwrap();
    sha256_bytes(text.as_bytes())
}

fn raw_file_sha256(path: &Path) -> String {
    sha256_bytes(&fs::read(path).unwrap())
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::from("sha256:");
    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn write_fake_daemon(path: &Path) {
    fs::write(
        path,
        r#"#!/usr/bin/env python3
import json
import os
import sys

request = json.loads(sys.stdin.readline())
with open(os.environ["CYRUNE_FAKE_PAYLOAD"], "r", encoding="utf-8") as handle:
    payload = json.load(handle)
print("fake-daemon-diagnostic", file=sys.stderr)
print(json.dumps({
    "version": "cyrune.free.ipc.v1",
    "message_id": "MSG-FAKE-RESPONSE",
    "response_to": request["message_id"],
    "status": "ok",
    "payload": payload,
}))
"#,
    )
    .unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn first_success_verifier_accepts_bound_evidence_and_terminal_marker() {
    let state = valid_fixture();

    let output = state.run_verifier();
    let report = report(&output);

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    assert_eq!(
        report["schema_version"],
        "cyrune.free.first-success-verifier-report.v1"
    );
    assert_eq!(report["verified"], true);
    assert_eq!(report["outcome"], "accepted");
    assert_eq!(report["failure_code"], Value::Null);
    assert_eq!(report["diagnostics"], json!(["fake-daemon-diagnostic"]));
    assert_eq!(
        report["state_root"],
        fs::canonicalize(&state.state_root)
            .unwrap()
            .display()
            .to_string()
    );
    assert_eq!(
        report["cyrune_home"],
        fs::canonicalize(&state.cyrune_home)
            .unwrap()
            .display()
            .to_string()
    );
    assert_eq!(
        report["terminal_binding_schema_version"],
        TERMINAL_SCHEMA_VERSION
    );
}

#[test]
fn first_success_verifier_rejects_rejected_payload() {
    let state = valid_fixture();
    let mut payload =
        accepted_payload("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    payload["outcome"] = json!("rejected");
    state.write_payload(&payload);

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["verified"], false);
    assert_eq!(report["failure_code"], "FSV-REJECTED-PAYLOAD");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        "FSV-REJECTED-PAYLOAD"
    );
}

#[test]
fn first_success_verifier_rejects_missing_required_id() {
    let state = valid_fixture();
    let mut payload =
        accepted_payload("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    payload.as_object_mut().unwrap().remove("run_id");
    state.write_payload(&payload);

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-MISSING-FIELD");
}

#[test]
fn first_success_verifier_rejects_wrong_policy_pack() {
    let state = valid_fixture();
    let mut payload =
        accepted_payload("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    payload["policy_pack_id"] = json!("other-policy");
    state.write_payload(&payload);

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-POLICY-MISMATCH");
}

#[test]
fn first_success_verifier_rejects_wrong_citation_bundle() {
    let state = valid_fixture();
    let mut payload =
        accepted_payload("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    payload["citation_bundle_id"] = json!("CB-20260501-9999");
    state.write_payload(&payload);

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-CITATION-MISMATCH");
}

#[test]
fn first_success_verifier_rejects_missing_evidence_directory() {
    let state = valid_fixture();
    fs::remove_dir_all(&state.evidence_dir).unwrap();

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-EVIDENCE-MISSING");
}

#[test]
fn first_success_verifier_rejects_stale_evidence_hash() {
    let state = valid_fixture();
    fs::write(state.evidence_dir.join("stdout.log"), b"mutated\n").unwrap();

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-EVIDENCE-HASH-MISMATCH");
}

#[test]
fn first_success_verifier_rejects_working_hash_mismatch() {
    let state = valid_fixture();
    fs::write(
        state.cyrune_home.join("working").join("working.json"),
        b"{\"stale\":true}\n",
    )
    .unwrap();

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-WORKING-HASH-MISMATCH");
}

#[test]
fn first_success_verifier_rejects_missing_terminal_marker() {
    let state = valid_fixture();
    fs::remove_file(&state.terminal_marker).unwrap();

    let output = state.run_verifier();
    let report = report(&output);

    assert!(!output.status.success());
    assert_eq!(report["failure_code"], "FSV-TERMINAL-BINDING-MISSING");
}
