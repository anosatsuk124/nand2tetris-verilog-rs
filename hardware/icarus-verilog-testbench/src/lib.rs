use std::path::PathBuf;

pub struct IVerilogTest {
    args: Vec<String>,
}

pub struct IVerilogTestBuilder {
    paths: Vec<PathBuf>,
    args: Vec<String>,
}

impl IVerilogTest {
    pub fn build() -> IVerilogTestBuilder {
        IVerilogTestBuilder {
            paths: Vec::new(),
            args: Vec::new(),
        }
    }

    pub fn test(&self, out: &PathBuf) -> Result<String, String> {
        let mut iverilog = std::process::Command::new("iverilog");
        iverilog.args(&self.args);
        iverilog.args(&["-o", out.to_str().unwrap()]);
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
            args: Vec::new(),
        }
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.paths.push(path);
        self
    }

    pub fn arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    pub fn build(self) -> IVerilogTest {
        IVerilogTest { args: self.args }
    }
}
