use sha2::Digest;

use crate::message::{log::LogMessage, qr::QRMessage};

mod message;

// TODO: Hexadezimalzahlen großschreiben

static DATA: &'static str = "V0;NAMOS-00ND180490-001;Kassenbeleg-V1;
Beleg^15,45_0,00_0,00_0,00_0,00_0,00_0,00^15,45:Unbar;
281331;628393;
2022-05-12T15:43:29+02:00;2022-05-12T15:43:43+02:00;
ecdsa-plain-SHA384;unixTime;
77687BCCB39CDFDBEC746F49CDB566424831A652883
DD7999BEC160AD3451A71581B5C7D6DEEEF5B18654D
F2417901E85A6BEA0CEEAFC08A47C468D6FDC3E4325
6DB6FD0E74FF98781363C86A1F7842CFA52A7FC530E2
160A7E8359DCF45C049;
BICZGiiOoTzFL0GoNdwfkpvhuo56bNXzekbaceeJCB/FtdD
5PFjTV3/WqscNTSjv+lhgBBSjs0BrxNdWCfXF/R8LTFmGMf
D1gqsHE8u/rdNlMtnEXqcBo5/zE/oK8t2CXg==";

fn main() {
    let qr = QRMessage::new(DATA.to_string());
    println!("QR Message debug output:");
    println!("{qr:?}");
    println!("Log Message debug output:");
    let log = LogMessage::from(qr.clone());
    println!("{log}");

    println!("log hash:");
    println!(
        "{}",
        sha2::Sha384::digest(log.as_bytes())
            .to_vec()
            .iter()
            .map(|x| format!("{x:X}"))
            .collect::<String>(),
    );

    let (dx, dy) = qr.dxdy();

    println!("dx:\n{dx}\ndy:\n{dy}");

    let (r, s) = qr.rs();

    println!("r:\n{r}\ns:\n{s}");
}
