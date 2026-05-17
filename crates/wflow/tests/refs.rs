use std::{fs, os::unix::fs::symlink};

use tempfile::TempDir;

mod support;

use support::{Fixture, stderr};

#[test]
fn verify_accepts_valid_metadata() {
    Fixture::valid().assert_ok(&["refs", "verify"]);
}

#[test]
fn verify_rejects_missing_metadata() {
    let fixture = Fixture::valid();
    fixture.remove("skills/demo/skill.toml");

    fixture.assert_err_contains(&["refs", "verify"], "missing metadata");
}

#[test]
fn verify_rejects_unknown_metadata_keys() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language",
]
owned = []
surprise = true
"#,
    );

    fixture.assert_err_contains(&["refs", "verify"], "unknown field");
}

#[test]
fn verify_invalid_toml_reports_source_context() {
    let fixture = Fixture::valid();
    fixture.write("skills/demo/skill.toml", "[references\nshared = []\n");

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("invalid metadata"), "{stderr}");
    assert!(stderr.contains("skills/demo/skill.toml"), "{stderr}");
    assert!(stderr.contains("[references"), "{stderr}");
}

#[test]
fn verify_invalid_toml_does_not_skip_package_file_errors() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("SKILL.md");
    fs::write(
        &external_file,
        "Use [workflow](references/workflow-language.md).\n",
    )
    .expect("write external skill markdown");
    fixture.remove("skills/demo/SKILL.md");
    symlink(&external_file, fixture.path("skills/demo/SKILL.md")).expect("symlink skill markdown");
    fixture.write("skills/demo/skill.toml", "[references\nshared = []\n");

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("invalid metadata"), "{stderr}");
    assert!(
        stderr.contains("symlinked skill instruction file"),
        "{stderr}"
    );
}

#[test]
fn verify_rejects_unknown_top_level_metadata_keys() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"surprise = true

[references]
shared = [
  "workflow-language",
]
owned = []
"#,
    );

    fixture.assert_err_contains(&["refs", "verify"], "unknown field");
}

#[test]
fn verify_rejects_missing_required_reference_lists() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language",
]
"#,
    );

    fixture.assert_err_contains(&["refs", "verify"], "missing field");
}

#[test]
fn verify_rejects_unsorted_and_duplicate_metadata() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [templates](references/templates.md) and [workflow](references/workflow-language.md).\n",
    );
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language",
  "templates",
  "templates",
]
owned = []
"#,
    );
    fixture.write("skills/demo/references/templates.md", "templates\n");

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("is not sorted"), "{stderr}");
    assert!(
        stderr.contains("duplicate reference `templates`"),
        "{stderr}"
    );
}

#[test]
fn verify_rejects_invalid_reference_names() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [workflow](references/workflow-language.md).\n",
    );
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language.md",
]
owned = [
  "../notes",
]
"#,
    );

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("invalid reference name"), "{stderr}");
    assert!(stderr.contains("workflow-language.md"), "{stderr}");
    assert!(stderr.contains("../notes"), "{stderr}");
}

#[test]
fn verify_invalid_reference_name_span_ignores_comment_occurrences() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"# "../notes"
[references]
shared = [
  "workflow-language",
]
owned = [
  "../notes",
]
"#,
    );

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("invalid reference name"), "{stderr}");
    assert!(stderr.contains("6 │ owned = ["), "{stderr}");
    assert!(stderr.contains("7 │   \"../notes\","), "{stderr}");
}

#[test]
fn verify_ignores_skill_markdown_reference_links() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [parent](../references/workflow-language.md), [absolute](/references/workflow-language.md), [url](https://example.test/references/workflow-language.md), and [bad](references/workflow-language.md.bak).\n",
    );

    fixture.assert_ok(&["refs", "verify"]);
}

#[test]
fn verify_rejects_missing_shared_source() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [missing](references/missing-source.md).\n",
    );
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "missing-source",
]
owned = []
"#,
    );

    fixture.assert_err_contains(&["refs", "verify"], "missing-source");
}

#[test]
fn verify_rejects_missing_generated_file() {
    let fixture = Fixture::valid();
    fixture.remove("skills/demo/references/workflow-language.md");

    fixture.assert_err_contains(&["refs", "verify"], "missing declared generated reference");
}

#[test]
fn verify_rejects_stale_generated_file() {
    let fixture = Fixture::valid();
    fixture.write("skills/demo/references/workflow-language.md", "stale\n");

    fixture.assert_err_contains(&["refs", "verify"], "differs from shared source");
}

#[test]
fn verify_rejects_extra_generated_file() {
    let fixture = Fixture::valid();
    fixture.write("skills/demo/references/templates.md", "templates\n");

    fixture.assert_err_contains(&["refs", "verify"], "undeclared generated shared reference");
}

#[test]
fn verify_rejects_temporary_reference_artifacts() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/references/.workflow-language.md.tmp.123.0",
        "temporary\n",
    );

    fixture.assert_err_contains(&["refs", "verify"], "temporary reference artifact");
}

#[test]
fn verify_rejects_undeclared_skill_owned_reference() {
    let fixture = Fixture::valid();
    fixture.write("skills/demo/references/local-notes.md", "local\n");

    fixture.assert_err_contains(&["refs", "verify"], "undeclared skill-owned reference");
}

#[test]
fn verify_rejects_missing_declared_skill_owned_reference() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [workflow](references/workflow-language.md) and [local](references/local-notes.md).\n",
    );
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language",
]
owned = [
  "local-notes",
]
"#,
    );

    fixture.assert_err_contains(
        &["refs", "verify"],
        "missing declared skill-owned reference",
    );
}

#[test]
fn verify_reports_all_discovered_errors() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language",
]
owned = [
  "local-notes",
]
"#,
    );
    fixture.write("skills/demo/references/workflow-language.md", "stale\n");

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("differs from shared source"), "{stderr}");
    assert!(
        stderr.contains("missing declared skill-owned reference"),
        "{stderr}"
    );
}

#[test]
fn verify_reports_errors_across_metadata_package_generated_and_filesystem_categories() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("SKILL.md");
    fs::write(
        &external_file,
        "Use [workflow](references/workflow-language.md).\n",
    )
    .expect("write external skill markdown");
    fixture.remove("skills/demo/SKILL.md");
    symlink(&external_file, fixture.path("skills/demo/SKILL.md")).expect("symlink skill markdown");
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "workflow-language.md",
]
owned = [
  "local-notes",
]
"#,
    );

    let output = fixture.run(&["refs", "verify"]);
    let stderr = stderr(&output);
    assert!(!output.status.success(), "expected failure");
    assert!(stderr.contains("invalid reference name"), "{stderr}");
    assert!(stderr.contains("workflow-language.md"), "{stderr}");
    assert!(stderr.contains("skills/demo/skill.toml"), "{stderr}");
    assert!(
        stderr.contains("symlinked skill instruction file"),
        "{stderr}"
    );
    assert!(
        stderr.contains("undeclared generated shared reference"),
        "{stderr}"
    );
    assert!(
        stderr.contains("missing declared skill-owned reference"),
        "{stderr}"
    );
}

#[test]
fn verify_accepts_skill_markdown_links_not_declared_in_metadata() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/SKILL.md",
        "Use [workflow](references/workflow-language.md) and [templates](references/templates.md).\n",
    );

    fixture.assert_ok(&["refs", "verify"]);
}

#[test]
fn verify_accepts_declared_reference_not_linked_from_skill_markdown() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "templates",
  "workflow-language",
]
owned = []
"#,
    );
    fixture.write("skills/demo/references/templates.md", "templates\n");

    fixture.assert_ok(&["refs", "verify"]);
}

#[test]
fn verify_rejects_symlinked_skill_metadata() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("skill.toml");
    fs::write(
        &external_file,
        r#"[references]
shared = [
  "workflow-language",
]
owned = []
"#,
    )
    .expect("write external metadata");
    fixture.remove("skills/demo/skill.toml");
    symlink(&external_file, fixture.path("skills/demo/skill.toml"))
        .expect("symlink skill metadata");

    fixture.assert_err_contains(&["refs", "verify"], "symlinked skill metadata file");
}

#[test]
fn verify_rejects_symlinked_skill_markdown() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("SKILL.md");
    fs::write(
        &external_file,
        "Use [workflow](references/workflow-language.md).\n",
    )
    .expect("write external skill markdown");
    fixture.remove("skills/demo/SKILL.md");
    symlink(&external_file, fixture.path("skills/demo/SKILL.md")).expect("symlink skill markdown");

    fixture.assert_err_contains(&["refs", "verify"], "symlinked skill instruction file");
}

#[test]
fn verify_rejects_skill_owned_shared_name_collision() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = []
owned = [
  "workflow-language",
]
"#,
    );

    fixture.assert_err_contains(&["refs", "verify"], "collides with a shared reference");
}
