use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::PathBuf,
    process::{Command, Output},
};

use tempfile::TempDir;

struct Fixture {
    _temp: TempDir,
    root: PathBuf,
}

impl Fixture {
    fn valid() -> Self {
        let temp = TempDir::new().expect("temp dir");
        let root = temp.path().to_path_buf();
        let fixture = Self { _temp: temp, root };

        fixture.write("skills/_shared/templates.md", "templates\n");
        fixture.write("skills/_shared/workflow-language.md", "workflow\n");
        fixture.write(
            "skills/demo/SKILL.md",
            "Use [workflow](references/workflow-language.md).\n",
        );
        fixture.write(
            "skills/demo/skill.toml",
            r#"[references]
shared = [
  "workflow-language",
]
owned = []
"#,
        );
        fixture.write("skills/demo/references/workflow-language.md", "workflow\n");

        fixture
    }

    fn write(&self, path: &str, content: &str) {
        let path = self.path(path);
        fs::create_dir_all(path.parent().expect("parent dir")).expect("create parent dir");
        fs::write(path, content).expect("write fixture file");
    }

    fn path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    fn remove(&self, path: &str) {
        fs::remove_file(self.root.join(path)).expect("remove fixture file");
    }

    fn remove_dir(&self, path: &str) {
        fs::remove_dir_all(self.root.join(path)).expect("remove fixture dir");
    }

    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.root.join(path)).expect("read fixture file")
    }

    fn exists(&self, path: &str) -> bool {
        self.path(path).exists()
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_wflow"))
            .arg("--root")
            .arg(&self.root)
            .args(args)
            .output()
            .expect("run wflow")
    }

    fn assert_ok(&self, args: &[&str]) {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "expected success, got stderr:\n{}",
            stderr(&output)
        );
    }

    fn assert_err_contains(&self, args: &[&str], expected: &str) {
        let output = self.run(args);
        assert!(!output.status.success(), "expected failure");
        assert!(
            stderr(&output).contains(expected),
            "expected stderr to contain {expected:?}, got:\n{}",
            stderr(&output)
        );
    }
}

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

    fixture.assert_err_contains(&["refs", "verify"], "invalid reference name");
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

    let fixture = Fixture { _temp: temp, root };
    fixture.assert_err_contains(&["refs", "sync"], "skill directory is symlinked");

    assert!(
        external.path().join("references/templates.md").exists(),
        "sync deleted an external generated reference through a symlinked skill dir"
    );
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

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}
