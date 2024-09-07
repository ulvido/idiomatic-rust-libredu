use std::{error::Error, fmt::Display, num::ParseFloatError, process::Termination, str::FromStr};

/// Verilen texti para ve parabirimine ayır
/// "42 lira" -> 42 ve "lira"
/// "42 lira" -> Para
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Para {
    pub miktar: f64,
    pub para_birimi: ParaBirimi,
}

impl FromStr for Para {
    type Err = ParaError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parcalar: Vec<&str> = input.split_whitespace().collect();

        match parcalar[..] {
            [miktar, para_birimi] => Ok(Para::from_parts(miktar.parse()?, para_birimi.parse()?)),
            _ => Err(ParaError {
                kind: ParaErrorKind::Formatting,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ParaBirimi {
    #[default]
    Lira,
    Dolar,
    Euro,
}

impl Para {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parts(miktar: f64, para_birimi: ParaBirimi) -> Self {
        Self {
            miktar,
            para_birimi,
        }
    }
}

impl Display for Para {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Para => {} {}", self.miktar, self.para_birimi)
    }
}

impl FromStr for ParaBirimi {
    type Err = ParaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "lira" | "tl" => Ok(ParaBirimi::Lira),
            "dolar" | "$" | "usd" => Ok(ParaBirimi::Dolar),
            "euro" | "€" | "eur" => Ok(ParaBirimi::Euro),
            _ => Err(ParaError {
                kind: ParaErrorKind::ParaBirimi,
            }),
        }
    }
}

impl Display for ParaBirimi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParaBirimi::Lira => write!(f, "lira"),
            ParaBirimi::Dolar => write!(f, "$"),
            ParaBirimi::Euro => write!(f, "€"),
        }
    }
}

// ERROR

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParaError {
    kind: ParaErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParaErrorKind {
    Formatting,
    Miktar,
    ParaBirimi,
}

impl Error for ParaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Display for ParaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ParaErrorKind::Formatting => {
                write!(f, "Miktar ve parabirimi şeklinde verin. Ör: 42 lira")
            }
            ParaErrorKind::Miktar => write!(f, "Miktar ayrıştırma hatası"),
            ParaErrorKind::ParaBirimi => write!(f, "ParaBirimi ayrıştırma hatası"),
        }
    }
}

impl From<ParseFloatError> for ParaError {
    fn from(_: ParseFloatError) -> Self {
        ParaError {
            kind: ParaErrorKind::Miktar,
        }
    }
}

impl Termination for Para {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}

impl Termination for ParaError {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_tamsayi() {
        let result = "42 lira".parse::<Para>();
        assert_eq!(Ok(Para::from_parts(42.0, ParaBirimi::Lira)), result);
    }

    #[test]
    fn negative_tamsayi() {
        let result = "-42 lira".parse::<Para>();
        assert_eq!(Ok(Para::from_parts(-42.0, ParaBirimi::Lira)), result);
    }

    #[test]
    fn positive_ondalik() {
        let result = "42.1 lira".parse::<Para>();
        assert_eq!(Ok(Para::from_parts(42.1, ParaBirimi::Lira)), result);
    }

    #[test]
    fn negative_ondalik() {
        let result = "-42.1 lira".parse::<Para>();
        assert_eq!(Ok(Para::from_parts(-42.1, ParaBirimi::Lira)), result);
    }

    #[test]
    #[should_panic(expected = "ParaError { kind: Formatting }")]
    fn tek_oge() {
        "42".parse::<Para>().unwrap();
    }

    #[test]
    #[should_panic(expected = "ParaError { kind: Formatting }")]
    fn cok_oge() {
        "42 lira dolar".parse::<Para>().unwrap();
    }

    #[test]
    #[should_panic(expected = "ParaError { kind: Miktar }")]
    fn hatali_miktar() {
        "42a lira".parse::<Para>().unwrap();
    }
}
