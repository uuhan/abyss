use serde::{Deserialize, Serialize};

use chrono::prelude::*;
use ctp::api::*;

#[derive(Serialize, Deserialize)]
#[repr(C)]
struct InstrumentKey<'a> {
    instrument: &'a str,
    timestamp: &'a str,
}

pub fn main() -> sled::Result<()> {
    let _ = simple_logger::init_by_env();
    let db = sled::open("abyss.db")?;

    let key1 = InstrumentKey {
        instrument: "OI101",
        timestamp: "2020-09-02 13:30:00.000000 +08:00",
    };

    let key2 = InstrumentKey {
        instrument: "OI101",
        timestamp: "2020-09-02 13:31:00.000000 +08:00",
    };

    let k1 = bincode::serialize(&key1).unwrap();
    let k2 = bincode::serialize(&key2).unwrap();

    // let v = db.get_gt(&k1)?;
    // let (k1, _) = v.unwrap();
    // let v = db.get_gt(&k2)?;
    // let (k2, _) = v.unwrap();
    // println!("{:?}", &k2);
    // let v = bincode::deserialize::<CThostFtdcDepthMarketDataField>(&v).unwrap();
    // println!("{}", v.LastPrice);

    // for kv in db.iter() {
    //     let (k, v) = kv?;
    //     println!("{:?}", &k);
    //     let i = bincode::deserialize::<InstrumentKey>(&k).unwrap();
    //     println!("{} - {}", i.instrument, i.timestamp);
    // }
    let mut idx = 0;
    // println!("{:?}", &k1);
    for kv in db.range(k1..k2) {
        // if idx > 5 { break; }

        let (k, v) = kv?;
        let i = bincode::deserialize::<InstrumentKey>(&k).unwrap();
        if let Ok(v) = bincode::deserialize::<CThostFtdcDepthMarketDataField>(&v) {
            println!(
                "{}, {} LastPrice: {}",
                i.instrument, i.timestamp, v.LastPrice
            );
        } else {
            // db.del(&k)?;
        }

        idx = idx + 1;
    }

    println!("total: {}", idx);

    // let mut iter = db.range(bincode::serialize(&key1).unwrap()..);
    // let (k, v) = iter.next().unwrap().unwrap();
    // println!("Timestamp: {}", v.timestamp);

    Ok(())
}
