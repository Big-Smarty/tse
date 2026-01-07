use base64::{alphabet::BIN_HEX, prelude::*};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct QRMessage {
    pub(crate) version: String,
    pub(crate) client_id: String,
    pub(crate) kassen_id: String,
    pub(crate) process_data: String,
    pub(crate) transaction_number: String,
    pub(crate) signature_counter: String,
    pub(crate) log_time: String,
    pub(crate) crypto_suite: String,
    pub(crate) time_format: String,
    pub(crate) signature: String,
    pub(crate) pubkey: Vec<u8>,
}

impl QRMessage {
    pub fn new(input: String) -> Self {
        let mut splits = input
            .split(";")
            .map(|x| x.replace("\n", "").to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();

        // remove TSE begin time
        splits.remove(6);

        Self {
            version: splits[0].clone(),
            client_id: splits[1].clone(),
            kassen_id: splits[2].clone(),
            process_data: splits[3].clone(),
            transaction_number: splits[4].clone(),
            signature_counter: splits[5].clone(),
            log_time: splits[6].clone(),
            crypto_suite: splits[7].clone(),
            time_format: splits[8].clone(),
            signature: splits[9].clone(),
            pubkey: BASE64_STANDARD.decode(splits[10].clone()).unwrap(),
        }
    }
}

impl Display for QRMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.client_id)?;
        writeln!(f, "{}", self.kassen_id)?;
        writeln!(f, "{}", self.process_data)?;
        writeln!(f, "{}", self.transaction_number)?;
        writeln!(f, "{}", self.signature_counter)?;
        writeln!(f, "{}", self.log_time)?;
        writeln!(f, "{}", self.crypto_suite)?;
        writeln!(f, "{}", self.time_format)?;
        writeln!(f, "{}", self.signature)?;
        writeln!(f, "{:x?}", self.pubkey)
    }
}
