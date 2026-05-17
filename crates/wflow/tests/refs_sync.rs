use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};

use tempfile::TempDir;

mod support;

use support::Fixture;

#[test]
fn sync_copies_declared_shared_references_and_prunes_extra_generated_files() {
    let fixture = Fixture::valid();
    fixture.write("skills/demo/references/workflow-language.md", "stale\n");
    fixture.write("skills/demo/references/templates.md", "templates\n");

    fixture.assert_ok(&["refs", "sync"]);

    assert_eq!(
        fixture.read("skills/demo/references/workflow-language.md"),
        "workflow\n"
    );
    assert!(!fixture.exists("skills/demo/references/templates.md"));
}

#[test]
fn sync_creates_missing_references_directory() {
    let fixture = Fixture::valid();
    fixture.remove_dir("skills/demo/references");

    fixture.assert_ok(&["refs", "sync"]);

    assert_eq!(
        fixture.read("skills/demo/references/workflow-language.md"),
        "workflow\n"
    );
}

#[test]
fn sync_preserves_declared_skill_owned_references() {
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
    fixture.write("skills/demo/references/local-notes.md", "local\n");

    fixture.assert_ok(&["refs", "sync"]);

    assert_eq!(
        fixture.read("skills/demo/references/local-notes.md"),
        "local\n"
    );
}

#[test]
fn sync_does_not_rewrite_unchanged_generated_references() {
    let fixture = Fixture::valid();
    let path = fixture.path("skills/demo/references/workflow-language.md");
    let mut permissions = fs::metadata(&path).expect("metadata").permissions();
    permissions.set_mode(0o444);
    fs::set_permissions(&path, permissions).expect("set readonly permissions");

    fixture.assert_ok(&["refs", "sync"]);
}

#[test]
fn sync_rejects_symlinked_references_directory_without_mutating_external_target() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    fixture.remove_dir("skills/demo/references");
    symlink(external.path(), fixture.path("skills/demo/references"))
        .expect("symlink references dir");

    fixture.assert_err_contains(&["refs", "sync"], "symlinked references directory");

    assert!(
        !external.path().join("workflow-language.md").exists(),
        "sync wrote through a symlinked references directory"
    );
}

#[test]
fn sync_rejects_symlinked_reference_file_without_mutating_external_target() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("workflow-language.md");
    fs::write(&external_file, "external\n").expect("write external file");
    fixture.remove("skills/demo/references/workflow-language.md");
    symlink(
        &external_file,
        fixture.path("skills/demo/references/workflow-language.md"),
    )
    .expect("symlink reference file");
    fixture.write("skills/_shared/workflow-language.md", "changed\n");

    fixture.assert_err_contains(&["refs", "sync"], "symlinked reference file");

    assert_eq!(
        fs::read_to_string(external_file).expect("read external file"),
        "external\n"
    );
}

#[test]
fn sync_rejects_hardlinked_reference_file_without_mutating_external_target() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("workflow-language.md");
    fs::write(&external_file, "external\n").expect("write external file");
    fixture.remove("skills/demo/references/workflow-language.md");
    fs::hard_link(
        &external_file,
        fixture.path("skills/demo/references/workflow-language.md"),
    )
    .expect("hardlink reference file");
    fixture.write("skills/_shared/workflow-language.md", "changed\n");

    fixture.assert_err_contains(&["refs", "sync"], "hardlinked reference file");

    assert_eq!(
        fs::read_to_string(external_file).expect("read external file"),
        "external\n"
    );
}

#[test]
fn sync_rejects_symlinked_shared_reference_source_without_copying_external_content() {
    let fixture = Fixture::valid();
    let external = TempDir::new().expect("external temp dir");
    let external_file = external.path().join("workflow-language.md");
    fs::write(&external_file, "external\n").expect("write external file");
    fixture.remove("skills/_shared/workflow-language.md");
    symlink(
        &external_file,
        fixture.path("skills/_shared/workflow-language.md"),
    )
    .expect("symlink shared reference source");

    fixture.assert_err_contains(&["refs", "sync"], "symlinked shared reference file");

    assert_eq!(
        fixture.read("skills/demo/references/workflow-language.md"),
        "workflow\n"
    );
}

#[test]
fn sync_rejects_symlinked_skill_directory_without_mutating_external_target() {
    let temp = TempDir::new().expect("temp dir");
    let root = temp.path().to_path_buf();
    fs::create_dir_all(root.join("skills")).expect("create skills dir");
    fs::create_dir_all(root.join("skills/_shared")).expect("create shared dir");
    fs::write(root.join("skills/_shared/templates.md"), "templates\n").expect("write templates");
    fs::write(
        root.join("skills/_shared/workflow-language.md"),
        "workflow\n",
    )
    .expect("write workflow");

    let external = TempDir::new().expect("external temp dir");
    fs::create_dir_all(external.path().join("references")).expect("create external refs");
    fs::write(
        external.path().join("SKILL.md"),
        "Use [workflow](references/workflow-language.md).\n",
    )
    .expect("write external skill");
    fs::write(
        external.path().join("skill.toml"),
        r#"[references]
shared = [
  "workflow-language",
]
owned = []
"#,
    )
    .expect("write external metadata");
    fs::write(
        external.path().join("references/templates.md"),
        "templates\n",
    )
    .expect("write external undeclared reference");
    fs::write(
        external.path().join("references/workflow-language.md"),
        "workflow\n",
    )
    .expect("write external declared reference");

    symlink(external.path(), root.join("skills/demo")).expect("symlink skill dir");

    let fixture = Fixture::from_parts(temp, root);
    fixture.assert_err_contains(&["refs", "sync"], "skill directory is symlinked");

    assert!(
        external.path().join("references/templates.md").exists(),
        "sync deleted an external generated reference through a symlinked skill dir"
    );
}

#[test]
fn sync_reports_preflight_errors_before_writing() {
    let fixture = Fixture::valid();
    fixture.write(
        "skills/demo/skill.toml",
        r#"[references]
shared = [
  "missing-source",
]
owned = []
"#,
    );
    fixture.write("skills/demo/references/workflow-language.md", "stale\n");

    fixture.assert_err_contains(&["refs", "sync"], "missing-source");

    assert_eq!(
        fixture.read("skills/demo/references/workflow-language.md"),
        "stale\n"
    );
}
