use std::fmt::Display;

pub use gha_main_proc_macro::gha_main;

pub type GitHubActionResult = anyhow::Result<GitHubOutputs>;

/// A single GitHub Action output
pub struct GitHubOutput {
    key: String,
    value: String,
}

impl Display for GitHubOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl GitHubOutput {
    pub fn new(key: &str, value: impl Display) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

/// Multiple GitHub Action outputs
pub struct GitHubOutputs {
    outputs: Vec<GitHubOutput>,
}

impl GitHubOutputs {
    pub fn single(key: &str, value: impl Display) -> Self {
        Self {
            outputs: vec![GitHubOutput::new(key, value)],
        }
    }

    pub fn multiple(outputs: Vec<GitHubOutput>) -> Self {
        Self { outputs }
    }
}

impl Display for GitHubOutputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let outputs = self
            .outputs
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>();
        write!(f, "{}", outputs.join(","))
    }
}

#[macro_export]
macro_rules! gha_result {
    ($($value:ident),+) => {
        {
            let mut v = Vec::new();
            $(
                v.push(gha_main::GitHubOutput::new(stringify!($value), &$value.to_string()));
            )+
            gha_main::GitHubOutputs::multiple(v)
        }
    };
}
