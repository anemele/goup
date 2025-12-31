use std::str::FromStr;

/// support toolchain
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Toolchain {
    Stable,
    Unstable,
    Beta,
    Version(String),
    Nightly,
}

impl FromStr for Toolchain {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "stable" => Self::Stable,
            "unstable" => Self::Unstable,
            "nightly" | "tip" | "gotip" => Self::Nightly,
            "beta" => Self::Beta,
            _ => Self::Version(s.to_owned()),
        })
    }
}

/// a toolchain filter.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ToolchainFilter {
    Stable,
    Unstable,
    Beta,
    Filter(String),
}

impl ToolchainFilter {
    pub fn re(self) -> String {
        use ToolchainFilter::*;
        match self {
            Stable => r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?\b"#.to_string(),
            Unstable => {
                r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?(?:rc(?:0|[1-9]\d*))"#
                    .to_string()
            }
            Beta => r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?(?:beta(?:0|[1-9]\d*))"#
                .to_string(),
            Filter(s) => format!("(.*{s}.*)"),
        }
    }
}

impl FromStr for ToolchainFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "stable" => Self::Stable,
            "unstable" => Self::Unstable,
            "beta" => Self::Beta,
            _ => Self::Filter(s.to_owned()),
        })
    }
}
