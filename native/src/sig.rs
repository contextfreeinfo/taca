use base64::{prelude::BASE64_STANDARD, Engine};

#[derive(Clone, Debug)]
pub struct Sig {
    pub kind: SigKind,
    pub sig: Vec<u8>,
}

#[derive(Copy, Clone, Debug)]
pub enum SigKind {
    Sha256,
}

impl Sig {
    pub fn parse(text: &str) -> Option<Sig> {
        let mut words = text.split_ascii_whitespace();
        (words.next()? == "taca").then_some(())?;
        let kind = match words.next()? {
            "sha256" => SigKind::Sha256,
            _ => None?,
        };
        let sig = BASE64_STANDARD.decode(words.next()?).ok()?;
        Some(Sig { kind, sig })
    }
}
