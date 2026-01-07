use std::{fmt::Display, str::FromStr};

use chrono::prelude::*;
use hex;
use sha2::{Digest, Sha256};

use crate::message::qr::QRMessage;

// TODO: FESTE WERTE SIND MEHRERE INTEGER; SPEICHERE INTEGER IN EINEM VEC AB UND KONVERTIERE SIE EINZELN ZU HEX

const VERSION_TAG: &'static str = "02";
const DATATYPE_TAG: &'static str = "06";
const OPERATIONTYPE_TAG: &'static str = "80";
const CLIENTID_TAG: &'static str = "81";
const PROCESSDATA_TAG: &'static str = "82";
const PROCESSTYPE_TAG: &'static str = "83";
const TRANSACTIONNUMBER_TAG: &'static str = "85";
const SERIALNUMBER_TAG: &'static str = "04";
const SIGNATUREALGORITHM_TAG: &'static str = "30";
const SIGNATURECOUNTER_TAG: &'static str = "02";
const LOGTIME_TAG: &'static str = "02";

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub(crate) version: u8,
    pub(crate) data_type: String,
    pub(crate) operation_type: String,
    pub(crate) client_id: String,
    pub(crate) process_data: String,
    pub(crate) process_type: String,
    pub(crate) transaction_number: u32,
    pub(crate) serial_number: String,
    pub(crate) signature_algorithm: String,
    pub(crate) signature_counter: u32,
    pub(crate) log_time: DateTime<Utc>,
}

impl LogMessage {}

impl From<QRMessage> for LogMessage {
    fn from(value: QRMessage) -> Self {
        Self {
            version: 2,
            data_type: "060904007F000703070101".to_string(),
            operation_type: "801146696E6973685472616E73616374696F6E".to_string(),
            client_id: value.client_id.clone(),
            process_data: value.process_data.clone(),
            process_type: value.kassen_id,
            transaction_number: value.transaction_number.parse().unwrap(),
            serial_number: Sha256::digest(value.pubkey)
                .to_vec()
                .iter()
                .map(|x| format!("{x:x}"))
                .collect::<String>(),
            signature_algorithm: "300C060A04007F00070101040104".to_string(),
            signature_counter: value.signature_counter.parse().unwrap(),
            log_time: DateTime::from_str(&value.log_time.clone()).unwrap(),
        }
    }
}

impl Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let transaction_number_hex = format!("{:x}", self.transaction_number);
        let transaction_number_bytes_count =
            (transaction_number_hex.len() as f32 / 2.0).ceil() as usize;
        let transaction_number_string_length = transaction_number_bytes_count * 2;

        let signature_counter_hex = format!("{:x}", self.signature_counter);
        let signature_counter_bytes_count =
            (signature_counter_hex.len() as f32 / 2.0).ceil() as usize;
        let signature_counter_string_length = signature_counter_bytes_count * 2;

        let log_time_string = self.log_time.timestamp().to_string();
        let log_time_size = 8;
        let log_time_string_length = (log_time_string.len() as f32 / 2.0).ceil() as usize * 2;

        write!(
            f,
            "{VERSION_TAG:02}{version_length:02}{version:02}{datatype}{operationtype}{PROCESSDATA_TAG:02}{processdatasize:02x}{processdata}{PROCESSTYPE_TAG:02}{processtypesize:02x}{processtype}{TRANSACTIONNUMBER_TAG:02}{transactionnumbersize:02x}{transactionnumber:0>transaction_number_string_length$}{SERIALNUMBER_TAG:02}{serialnumbersize:02x}{serialnumber:0>serialnumberlength$}{signaturealgorithm}{SIGNATURECOUNTER_TAG:02}{signature_counter_bytes_count:02x}{signature_counter_hex:0>signature_counter_string_length$}{LOGTIME_TAG:02}{log_time_size:02x}{log_time_string:0>log_time_string_length$}",
            version_length = { size_of::<u8>() },
            version = self.version,
            datatype = self.data_type, //directly use general datatype
            operationtype = self.operation_type, //directly use general operationtype
            processdatasize = self.process_data.len(),
            processdata = hex::encode(self.process_data.clone()),
            processtypesize = self.process_type.len(),
            processtype = hex::encode(self.process_type.clone()),
            transactionnumbersize = transaction_number_bytes_count,
            transactionnumber = transaction_number_hex,
            serialnumbersize = self.serial_number.len(),
            serialnumber = self.serial_number,
            serialnumberlength = (self.serial_number.len() as f32 / 2.0).ceil() as usize * 2,
            signaturealgorithm = self.signature_algorithm.clone(),
        )
    }
}
