use std::{
    path::Path,
    process::{Command, ExitStatus},
};

pub struct Version {
    pub crate_name: String,
    pub crate_version: String,
    pub source_version: SourceVersion,
    pub compiler_mode: CompilerMode,
    pub os: String,
    pub arch: String,
    pub compiler_version: String,
}

pub enum CompilerMode {
    Debug,
    Release,
}

pub enum SourceVersion {
    None,
    Git {
        branch: String,
        hash: String,
        dirty: bool,
    },
    // TODO? mercurial?
}

impl Version {
    pub fn new<P: AsRef<Path>>(project_dir: P, crate_name: String, crate_version: String) -> Self {
        let v = Version {
            crate_name,
            crate_version,
            source_version: SourceVersion::extract(project_dir.as_ref()),
            compiler_mode: CompilerMode::guess(),
            os: ::std::env::consts::OS.to_owned(),
            arch: ::std::env::consts::ARCH.to_owned(),
            compiler_version: compiler_version(project_dir),
        };

        if let Ok(target) = std::env::var("TARGET") {
            v.with_target(target)
        } else {
            v
        }
    }

    pub fn with_target<T: AsRef<str>>(mut self, target: T) -> Self {
        if let Some(platform_target) = platforms::find(target) {
            self.arch = platform_target.target_arch.as_str().to_owned();
            self.os = platform_target.target_os.as_str().to_owned();
        }
        self
    }

    pub fn simple(&self) -> String {
        format!(
            "{crate_name} {crate_version}{source_version}",
            crate_name = self.crate_name,
            crate_version = self.crate_version,
            source_version = self.source_version.simple()
        )
    }

    pub fn hash(&self) -> String {
        self.source_version.hash()
    }

    pub fn full(&self) -> String {
        format!(
            "{crate_name} {crate_version} ({source_version}, {compiler_mode}, {os} [{arch}]) - [{compiler_version}]",
            crate_name = self.crate_name,
            crate_version = self.crate_version,
            source_version = self.source_version.full(),
            compiler_mode = self.compiler_mode.simple(),
            os = self.os,
            arch = self.arch,
            compiler_version = self.compiler_version,
        )
    }
}

impl CompilerMode {
    fn simple(&self) -> &'static str {
        match self {
            CompilerMode::Debug => "debug",
            CompilerMode::Release => "release",
        }
    }
}

#[cfg(debug_assertions)]
impl CompilerMode {
    pub fn guess() -> Self {
        CompilerMode::Debug
    }
}

#[cfg(not(debug_assertions))]
impl CompilerMode {
    pub fn guess() -> Self {
	CompilerMode::Release
    }
}

impl SourceVersion {
    pub fn extract<P: AsRef<Path>>(path: P) -> Self {
        Self::extract_git(path).unwrap_or(SourceVersion::None)
    }

    fn full(&self) -> String {
        match self {
            SourceVersion::None => "".to_owned(),
            SourceVersion::Git {
                branch,
                hash,
                dirty,
            } => format!(
                "{branch}-{hash}{dirty}",
                branch = branch,
                hash = hash,
                dirty = if *dirty { "+" } else { "" }
            ),
        }
    }

    fn simple(&self) -> String {
        match self {
            SourceVersion::None => "".to_owned(),
            SourceVersion::Git { hash, dirty, .. } => {
                format!("-{}{}", hash, if *dirty { "+" } else { "" })
            }
        }
    }

    fn hash(&self) -> String {
        match self {
            SourceVersion::None => "N/A".to_owned(),
            SourceVersion::Git { hash, .. } => hash.clone(),
        }
    }

    fn extract_git<P: AsRef<Path>>(path: P) -> Option<Self> {
        let rev = read_output(
            Command::new("git")
                .current_dir(path.as_ref())
                .arg("rev-parse")
                .arg("--short")
                .arg("HEAD"),
        )?;
        let branch = read_output(
            Command::new("git")
                .current_dir(path.as_ref())
                .arg("rev-parse")
                .arg("--abbrev-ref")
                .arg("HEAD"),
        )?;
        let dirty = get_status_code(
            Command::new("git")
                .current_dir(path)
                .arg("diff")
                .arg("--quiet")
                .arg("--exit-code")
                .arg("HEAD"),
        )?;

        Some(SourceVersion::Git {
            branch,
            hash: rev,
            dirty: !dirty.success(),
        })
    }
}

pub fn compiler_version<P: AsRef<Path>>(path: P) -> String {
    let compiler = read_output(Command::new("rustc").current_dir(path).arg("--version"));

    match compiler {
        None => "N/A".to_owned(),
        Some(version) => version,
    }
}

fn read_output(cmd: &mut Command) -> Option<String> {
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Some(
                    String::from_utf8_lossy(&output.stdout)
                        .trim_end()
                        .to_string(),
                )
            } else {
                None
            }
        }
        Err(error) => {
            use std::io::ErrorKind;
            match error.kind() {
                ErrorKind::NotFound => None,
                ErrorKind::PermissionDenied => None,
                _ => panic!("error while executing command, {:?}: {}", cmd, error),
            }
        }
    }
}

fn get_status_code(cmd: &mut Command) -> Option<ExitStatus> {
    match cmd.status() {
        Ok(status) => Some(status),
        Err(error) => {
            use std::io::ErrorKind;
            match error.kind() {
                ErrorKind::NotFound => None,
                ErrorKind::PermissionDenied => None,
                _ => panic!("error while executing command, {:?}: {}", cmd, error),
            }
        }
    }
}
