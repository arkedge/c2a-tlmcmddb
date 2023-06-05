use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Filename {
    pub component: Option<String>,
    pub telemetry: String,
}

const SYMBOL_TLM_DB: &str = "_TLM_DB_";
impl FromStr for Filename {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(basename) = s.strip_suffix(".csv") else {
            return Err(anyhow!("file extension must be .csv"));
        };
        let Some(symbol_index) = basename.find(SYMBOL_TLM_DB) else {
            return Err(anyhow!("filename must contain '_TLM_DB_'"));
        };
        let telemetry = basename[symbol_index + SYMBOL_TLM_DB.len()..].to_string();
        let head = &basename[..symbol_index];
        let Some(underscore_index) = head.rfind('_') else {
            return Ok(Self {
                component: None,
                telemetry,
            });
        };
        let component = head[underscore_index + 1..].to_string();
        Ok(Self {
            component: Some(component),
            telemetry,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let hk = "SAMPLE_MOBC_TLM_DB_HK.csv";
        let Filename {
            component,
            telemetry,
        } = hk.parse().unwrap();
        assert_eq!(Some("MOBC"), component.as_deref());
        assert_eq!("HK", telemetry);
    }
}
