use std::path::PathBuf;

pub struct IVerilogTest {
    args: Vec<String>,
}

pub struct IVerilogTestBuilder {
    paths: Vec<PathBuf>,
    top: Option<String>,
}

impl IVerilogTest {
    pub fn builder() -> IVerilogTestBuilder {
        IVerilogTestBuilder {
            paths: Vec::new(),
            top: None,
        }
    }

    pub fn test(&self, out: &PathBuf) -> Result<String, String> {
        let mut iverilog = std::process::Command::new("iverilog");
        iverilog.args(&["-o", out.to_str().unwrap()]);
        iverilog.args(&self.args);
        let output = iverilog.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "iverilog failed with status {:?}

{}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let mut vvp = std::process::Command::new("vvp");
        vvp.arg(out);
        let output = vvp.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            Err(format!(
                "vvp failed with status {:?} {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ))
        } else {
            Ok(String::from_utf8(output.stdout).expect("invalid utf8"))
        }
    }
}

impl IVerilogTestBuilder {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            top: None,
        }
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.paths.push(path);
        self
    }

    pub fn paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.paths.extend(paths);
        self
    }

    pub fn top(mut self, top: &str) -> Self {
        self.top = Some(top.to_string());
        self
    }

    pub fn build(self) -> IVerilogTest {
        let mut args = Vec::new();
        if let Some(top) = self.top {
            args.push("-s".to_string());
            args.push(top);
        }
        args.extend(
            self.paths
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string()),
        );
        IVerilogTest { args }
    }
}
