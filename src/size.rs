/// BytesSize 可以计算字节单位
///
/// # 例如：
/// #[cfg(test)]
// mod tests {
//     use std::ops::Sub;
//     use crate::size::{add, ByteSize, ByteSizeUnit};
//     use std::str::FromStr;
//     use super::*;
//
//     #[test]
//     fn parse() {
//         assert_eq!(ByteSize::from_str("12.3 PB").unwrap().to_size(), ByteSize::pb(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 TB").unwrap().to_size(), ByteSize::tb(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 GB").unwrap().to_size(), ByteSize::gb(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 MB").unwrap().to_size(), ByteSize::mb(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 KB").unwrap().to_size(), ByteSize::kb(12.3).to_size());
//
//         assert_eq!(ByteSize::from_str("12.3 PiB").unwrap().to_size(), ByteSize::pib(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 TiB").unwrap().to_size(), ByteSize::tib(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 GiB").unwrap().to_size(), ByteSize::gib(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 MiB").unwrap().to_size(), ByteSize::mib(12.3).to_size());
//         assert_eq!(ByteSize::from_str("12.3 KiB").unwrap().to_size(), ByteSize::kib(12.3).to_size());
//
//         assert_eq!("12.3 PB",ByteSize::pb(12.3).to_pb().to_string())
//     }
//
//     #[test]
//     fn convert() {
//         assert_eq!(ByteSize::from_str("12.3 PB").unwrap().to_tb().to_string(), ByteSize::pb(12.3).to_tb().to_string());
//         // ...
//     }
//
//     #[test]
//     fn calculate(){
//         let a=ByteSize::gb(2.0);
//         let b=ByteSize::mb(1000.0);
//         assert_eq!(add(a,b).unwrap().to_size(),ByteSize::gb(3.0).to_size())
//     }
// }


use std::ops::Add;
use std::str::FromStr;
use crate::ByteSize;


const B: u64 = 1;
const KB: u64 = 1000;
const MB: u64 = KB.pow(2);
const GB: u64 = KB.pow(3);
const TB: u64 = KB.pow(4);
const PB: u64 = KB.pow(5);

const KIB: u64 = 1024;
const MIB: u64 = KIB.pow(2);
const GIB: u64 = KIB.pow(3);
const TIB: u64 = KIB.pow(4);
const PIB: u64 = KIB.pow(5);

enum ByteUnit {
    KB,
    MB,
    GB,
    TB,
    PB,
    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
    B,
    IB,
}

pub struct ByteSizeUnit {
    size: f64,
    unit: ByteUnit,
}

impl ByteSize {
    pub fn from_str(str: &str) -> Result<Self, String> {
        let mut index = 0;
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            if !(byte >= b'0' && byte <= b'9' || byte == b'.') {
                index = i;
                break;
            }
        }
        let num = match str[..index].parse::<f64>() {
            Ok(num) => { num }
            Err(_) => {
                return Err(format!("解析错误：{}", str));
            }
        };

        let unit = Self::parse_unit(&str[index..].trim())?;
        Ok(Self((num * unit as f64) as u64))

        // let mut uindex: usize = 0;
        // let mut unit: usize = 0;
        // match (unitStr[uindex] as char).to_ascii_uppercase() {
        //     'K' => { unit = 1 }
        //     'M' => { unit = 2 }
        //     'G' => { unit = 3 }
        //     'T' => { unit = 4 }
        //     'P' => { unit = 5 }
        //     _ => { uindex -= 1 }
        // }
        // uindex += 1;
        // // for (i, x) in byte_unit.iter().enumerate() {
        // //     if (unitStr[uindex] as char).to_string().to_ascii_uppercase() == x.to_string() {
        // //         uindex += 1;
        // //         unit = i;
        // //         break;
        // //     }
        // // }
        // if unitStr[uindex] as char == 'i' {
        //     num *= 1024f64.powf(unit as f64);
        //     uindex += 1
        // } else {
        //     num *= 1000f64.powf(unit as f64);
        // }
        // if unitStr[uindex] as char == 'B' { () } else if unitStr[uindex] as char == 'b' {
        //     num /= 8f64;
        // } else {
        //     return Err(format!("解析错误：{}", str));
        // }
        // Ok(ByteSize(num as u64))
    }

    pub fn to_size(&self) -> u64 {
        return self.0;
    }

    fn parse_unit(unit: &str) -> Result<u64, String> {
        return match unit.to_uppercase().as_str() {
            "PB" => { Ok(PB) }
            "TB" => { Ok(TB) }
            "GB" => { Ok(GB) }
            "MB" => { Ok(MB) }
            "KB" => { Ok(KB) }
            "PIB" => { Ok(PIB) }
            "TIB" => { Ok(TIB) }
            "GIB" => { Ok(GIB) }
            "MIB" => { Ok(MIB) }
            "KIB" => { Ok(KIB) }
            "IB" | "B" => { Ok(B) }
            _ => { Err(format!("单位解析错误：{}", unit.to_string())) }
        };
    }
}

impl ByteSize {
    pub fn pib(size: f64) -> Self {
        Self((size * PIB as f64) as u64)
    }
    pub fn tib(size: f64) -> Self {
        Self((size * TIB as f64) as u64)
    }
    pub fn gib(size: f64) -> Self {
        Self((size * GIB as f64) as u64)
    }
    pub fn mib(size: f64) -> Self {
        Self((size * MIB as f64) as u64)
    }
    pub fn kib(size: f64) -> Self {
        Self((size * KIB as f64) as u64)
    }
    pub fn pb(size: f64) -> Self {
        Self((size * PB as f64) as u64)
    }
    pub fn tb(size: f64) -> Self {
        Self((size * TB as f64) as u64)
    }
    pub fn gb(size: f64) -> Self {
        Self((size * GB as f64) as u64)
    }
    pub fn mb(size: f64) -> Self {
        Self((size * MB as f64) as u64)
    }
    pub fn kb(size: f64) -> Self {
        Self((size * KB as f64) as u64)
    }
    pub fn b(size: f64) -> Self {
        Self(size as u64)
    }
    pub fn ib(size: f64) -> Self {
        Self(size as u64)
    }
}

impl ByteSize {
    pub fn to_pib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / PIB as f64,
            unit: ByteUnit::PiB,
        }
    }
    pub fn to_tib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / TIB as f64,
            unit: ByteUnit::TiB,
        }
    }
    pub fn to_gib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / GIB as f64,
            unit: ByteUnit::GiB,
        }
    }
    pub fn to_mib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / MIB as f64,
            unit: ByteUnit::MiB,
        }
    }
    pub fn to_kib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / KIB as f64,
            unit: ByteUnit::KiB,
        }
    }
    pub fn to_ib(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64,
            unit: ByteUnit::IB,
        }
    }
    pub fn to_pb(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / PB as f64,
            unit: ByteUnit::PB,
        }
    }
    pub fn to_tb(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / TB as f64,
            unit: ByteUnit::TB,
        }
    }
    pub fn to_gb(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / GB as f64,
            unit: ByteUnit::GB,
        }
    }
    pub fn to_mb(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / MB as f64,
            unit: ByteUnit::MB,
        }
    }
    pub fn to_kb(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64 / KB as f64,
            unit: ByteUnit::KB,
        }
    }
    pub fn to_b(&self) -> ByteSizeUnit {
        ByteSizeUnit {
            size: self.to_size() as f64,
            unit: ByteUnit::B,
        }
    }
}


impl ToString for ByteSizeUnit {
    fn to_string(&self) -> String {
        let unit = match self.unit {
            ByteUnit::KB => { "KB" }
            ByteUnit::MB => { "MB" }
            ByteUnit::GB => { "GB" }
            ByteUnit::TB => { "TB" }
            ByteUnit::PB => { "PB" }
            ByteUnit::KiB => { "KiB" }
            ByteUnit::MiB => { "MiB" }
            ByteUnit::GiB => { "GiB" }
            ByteUnit::TiB => { "TiB" }
            ByteUnit::PiB => { "PiB" }
            ByteUnit::B => { "B" }
            ByteUnit::IB => { "iB" }
        };
        format!("{} {}", self.size, unit)
    }
}

impl ByteSizeUnit {
    pub fn to_bytes_size(&self) -> ByteSize {
        let num: u64 = match self.unit {
            ByteUnit::KB => { (self.size * KB as f64) as u64 }
            ByteUnit::MB => { (self.size * MB as f64) as u64 }
            ByteUnit::GB => { (self.size * GB as f64) as u64 }
            ByteUnit::TB => { (self.size * TB as f64) as u64 }
            ByteUnit::PB => { (self.size * PB as f64) as u64 }
            ByteUnit::KiB => { (self.size * KIB as f64) as u64 }
            ByteUnit::MiB => { (self.size * MIB as f64) as u64 }
            ByteUnit::GiB => { (self.size * GIB as f64) as u64 }
            ByteUnit::TiB => { (self.size * TIB as f64) as u64 }
            ByteUnit::PiB => { (self.size * PIB as f64) as u64 }
            ByteUnit::B => { self.size as u64 }
            ByteUnit::IB => { self.size as u64 }
        };
        ByteSize(num)
    }
}

pub trait Calculate {
    fn add(&self, a: Self) -> Option<ByteSize>;
    fn sub(&self, r: Self) -> Option<ByteSize>;
}

pub fn add<T: Calculate>(s: T, a: T) -> Option<ByteSize> {
    return s.add(a);
}

pub fn sub<T: Calculate>(s: T, a: T) -> Option<ByteSize> {
    return s.add(a);
}

impl Calculate for ByteSizeUnit {
    fn add(&self, a: Self) -> Option<ByteSize> {
        let s = self.to_bytes_size().to_size().checked_add(a.to_bytes_size().to_size())?;
        Some(ByteSize(s))
    }
    fn sub(&self, r: Self) -> Option<ByteSize> {
        let s = self.to_bytes_size().to_size().checked_sub(r.to_bytes_size().to_size())?;
        Some(ByteSize(s))
    }
}

impl Calculate for ByteSize {
    fn add(&self, a: Self) -> Option<ByteSize> {
        let s = self.to_size().checked_add(a.to_size())?;
        Some(ByteSize(s))
    }
    fn sub(&self, r: Self) -> Option<ByteSize> {
        let s = self.to_size().checked_sub(r.to_size())?;
        Some(ByteSize(s))
    }
}
