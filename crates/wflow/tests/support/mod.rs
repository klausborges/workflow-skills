use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use tempfile::TempDir;

pub struct Fixture {
    _temp: TempDir,
    root: PathBuf,
}

impl Fixture {
    pub fn valid() -> Self {
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

    #[allow(dead_code)]
    pub const fn from_parts(temp: TempDir, root: PathBuf) -> Self {
        Self { _temp: temp, root }
    }

    #[allow(dead_code)]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn write(&self, path: &str, content: &str) {
        let path = self.path(path);
        fs::create_dir_all(path.parent().expect("parent dir")).expect("create parent dir");
        fs::write(path, content).expect("write fixture file");
    }

    pub fn path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    #[allow(dead_code)]
    pub fn remove(&self, path: &str) {
        fs::remove_file(self.path(path)).expect("remove fixture file");
    }

    #[allow(dead_code)]
    pub fn remove_dir(&self, path: &str) {
        fs::remove_dir_all(self.path(path)).expect("remove fixture dir");
    }

    #[allow(dead_code)]
    pub fn read(&self, path: &str) -> String {
        fs::read_to_string(self.path(path)).expect("read fixture file")
    }

    #[allow(dead_code)]
    pub fn exists(&self, path: &str) -> bool {
        self.path(path).exists()
    }

    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_wflow"))
            .arg("--root")
            .arg(&self.root)
            .args(args)
            .output()
            .expect("run wflow")
    }

    pub fn assert_err_contains(&self, args: &[&str], expected: &str) {
        let output = self.run(args);
        assert!(!output.status.success(), "expected failure");
        assert!(
            stderr(&output).contains(expected),
            "expected stderr to contain {expected:?}, got:\n{}",
            stderr(&output)
        );
    }

    #[allow(dead_code)]
    pub fn assert_ok(&self, args: &[&str]) {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "expected success, got stderr:\n{}",
            stderr(&output)
        );
    }
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}
