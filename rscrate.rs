use std;
use std::collections::HashMap;
use std::process::Command;

use cmderror::CmdError;


///
/// Information about a crate that we need to download and compile.
///
pub struct Crate {
    /// The crate name.
    pub name: String,

    /// The crate type (lib, dylib, etc.).
    kind: &'static str,

    /// Name of the Git repository we're pulling from.
    repository: String,

    /// Path to the repository from the base URL.
    /// On GitHub this is the repository's owner.
    path: String,

    /// Base URL (default: "https://github.com/").
    base: &'static str,

    /// Git branch.
    branch: String,

    /// The filename to pass to rustc (default: "[repo]/src/lib.rs").
    filename: String,

    /// External libraries to explicit import.
    externs: HashMap<String,String>,

    /// Other configuration data to pass to the compiler.
    cfg: Vec<String>,
}

impl Crate {
    pub fn new(name: &str) -> Crate {
        Crate {
            name: name.to_string(),
            kind: "lib",
            repository: name.to_string(),
            path: "".to_string(),
            base: "https://github.com",
            branch: "master".to_string(),
            filename: "src/lib.rs".to_string(),
            externs: HashMap::new(),
            cfg: vec![],
        }
    }

    pub fn cfg(mut self, c: &str) -> Crate {
        self.cfg.push(c.to_string());
        self
    }

    pub fn feature(mut self, name: &str) -> Crate {
        self.cfg.push(format!["feature=\"{}\"", name]);
        self
    }

    pub fn extern_lib(mut self, name: &str, value: &str) -> Crate {
        self.externs.insert(name.to_string(), value.to_string());
        self
    }

    pub fn filename(mut self, filename: &str) -> Crate {
        self.filename = filename.to_string();
        self
    }

    pub fn kind(mut self, t: &'static str) -> Crate {
        self.kind = t;
        self
    }

    pub fn owner(mut self, name: &str) -> Crate {
        self.path = name.to_string();
        self
    }

    pub fn repo(mut self, name: &str) -> Crate {
        self.repository = name.to_string();
        self
    }

    pub fn target_os(mut self, os: &str) -> Crate {
        self.cfg.push(format!["target_os=\"{}\"", os]);
        self
    }

    pub fn url(&self) -> String {
        format!["{}/{}/{}.git", self.base, self.path, self.repository]
    }

    pub fn fetch(&self, subdir: &str) -> Result<String, CmdError> {
        let url = self.url();
        let dest = format!["{}/{}", subdir, self.repository];

        let result = try![
            Command::new("git").arg("clone").arg("--recurse-submodules")
                    .arg(url).arg(&dest)
                    .arg("--branch").arg(&self.branch)
                    .stdout(std::process::Stdio::inherit())
                    .output()
        ];

        if !result.status.success() {
            let err = String::from_utf8_lossy(&result.stderr);

            if !err.contains("already exists and is not an empty directory") {
                println!["{}", err];
                panic!["git clone failed: {}", result.status];
            }

            let result = try![
                Command::new("git").arg("pull").arg("--rebase")
                        .arg("--recurse-submodules")
                        .current_dir(&dest).output()
            ];

            if !result.status.success() {
                return Err(CmdError::Exit(result));
            }
        }

        Ok(dest)
    }

    pub fn compile(&self, subdir: &str, builddir: &str)
        -> Result<String, CmdError> {

        try![std::fs::create_dir_all(builddir)];

        let mut command = Command::new("rustc");

        command.arg(format!["{}/{}/{}", subdir, self.repository, self.filename])
               .arg("--crate-name").arg(&self.name)
               .arg("--crate-type").arg(&self.kind)
               .arg("-L").arg(&builddir)
               .arg("--out-dir").arg(&builddir)
               ;

        for ref c in &self.cfg {
            command.arg("--cfg").arg(c);
        }

        for (name, dir) in &self.externs {
            command.arg("--extern")
                   .arg(format!["{}={}/lib{}.rlib", name, dir, name])
                   ;
        }

        let result = try![command.output()];

        if !result.status.success() {
            return Err(CmdError::Exit(result));
        }

        Ok(String::from_utf8_lossy(&result.stdout).into_owned())
    }
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write![f, "{} ({}/{})", self.name, self.path, self.repository]
    }
}
