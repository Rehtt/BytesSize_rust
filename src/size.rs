#[cfg(test)]
mod tests {
    use std::ops::Sub;
    use crate::{add, ByteSizeUnit,ByteSize};
    use std::str::FromStr;
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(ByteSize::from_str("12.3 PB").unwrap().to_size(), ByteSize::pb(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 TB").unwrap().to_size(), ByteSize::tb(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 GB").unwrap().to_size(), ByteSize::gb(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 MB").unwrap().to_size(), ByteSize::mb(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 KB").unwrap().to_size(), ByteSize::kb(12.3).to_size());

        assert_eq!(ByteSize::from_str("12.3 PiB").unwrap().to_size(), ByteSize::pib(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 TiB").unwrap().to_size(), ByteSize::tib(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 GiB").unwrap().to_size(), ByteSize::gib(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 MiB").unwrap().to_size(), ByteSize::mib(12.3).to_size());
        assert_eq!(ByteSize::from_str("12.3 KiB").unwrap().to_size(), ByteSize::kib(12.3).to_size());

        assert_eq!("12.3 PB",ByteSize::pb(12.3).to_pb().to_string())
    }

    #[test]
    fn convert() {
        assert_eq!(ByteSize::from_str("12.3 PB").unwrap().to_tb().to_string(), ByteSize::pb(12.3).to_tb().to_string());
        // ...
    }

    #[test]
    fn calculate(){
        let a=ByteSize::gb(2.0);
        let b=ByteSize::mb(1000.0);
        assert_eq!(add(a,b).unwrap().to_size(),ByteSize::gb(3.0).to_size())
    }
}

