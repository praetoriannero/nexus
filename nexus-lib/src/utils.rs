use num_traits::{FromPrimitive, PrimInt, Unsigned};
use pcap::TimestampType;
use std::mem;
use std::ops::BitOrAssign;

pub enum Endian {
    Big,
    Little,
}

static BYTE_SIZE: usize = 8;

pub fn parse_bytes<T>(bytes: &[u8], endian: Endian) -> T
where
    T: Unsigned + PrimInt + BitOrAssign + FromPrimitive,
{
    let size = mem::size_of::<T>();
    let mut result = T::zero();
    match endian {
        Endian::Big => {
            for shift in (0..size).rev() {
                result |= ((T::from_u8(bytes[shift as usize])).unwrap()
                    << ((size - shift - 1) * BYTE_SIZE)) as T;
            }
            result
        }
        Endian::Little => {
            for shift in 0..size {
                result |= (((T::from_u8(bytes[shift as usize])).unwrap() as T)
                    << (shift * BYTE_SIZE)) as T;
            }
            result
        }
    }
}

pub fn printable_ascii(input: &[u8]) -> String {
    input
        .iter()
        .map(|&b| {
            if (0x20..=0x7E).contains(&b) {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}

pub fn get_header_ts(header: &pcap::PacketHeader) -> f64 {
    let mut ts = header.ts.tv_sec as f64;
    ts += header.ts.tv_usec as f64 / 1_000_000.0;
    ts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u8() {
        let bytes = [251];
        println!("{}", parse_bytes::<u8>(&bytes, Endian::Little));
        assert!(parse_bytes::<u8>(&bytes, Endian::Big) == 251);
        assert!(parse_bytes::<u8>(&bytes, Endian::Little) == 251);
    }

    #[test]
    fn test_parse_u16() {
        let bytes = [1, 0];
        assert!(parse_bytes::<u16>(&bytes, Endian::Big) == 256);
        assert!(parse_bytes::<u16>(&bytes, Endian::Little) == 1);
    }

    #[test]
    fn test_parse_u32() {
        let bytes = [1, 1, 0, 0];
        assert!(parse_bytes::<u32>(&bytes, Endian::Big) == 16842752);
        println!("{}", parse_bytes::<u32>(&bytes, Endian::Little));
        assert!(parse_bytes::<u32>(&bytes, Endian::Little) == 257);
    }

    #[test]
    fn test_parse_u64() {
        let bytes = [1, 0, 1, 0, 0, 0, 0, 1];
        assert!(parse_bytes::<u64>(&bytes, Endian::Big) == 72058693549555713);
        assert!(parse_bytes::<u64>(&bytes, Endian::Little) == 72057594037993473);
    }
}
