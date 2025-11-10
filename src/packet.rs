use crate::pdu::Pdu;

#[macro_export]
macro_rules! packet {
    ( $( $pdu:expr ),* $(,)? ) => {{
        let mut v: Vec<Box<dyn Pdu>> = Vec::new();
        $( v.push(Box::new($pdu)); )*
        Packet{ pdu_chain: v }
    }};
}

pub struct Packet<'a> {
    pub pdu_chain: Vec<Box<dyn Pdu<'a>>>,
}

impl<'a> Packet<'a> {
    pub fn serialize(&mut self) -> Vec<u8> {
        let mut packet = Vec::new();
        for idx in 0..self.pdu_chain.len() {
            packet.extend_from_slice(&self.pdu_chain[idx].to_bytes());
        }
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ethernet::Ethernet;
    use crate::ip::Ip;

    #[test]
    fn test_pdu() {
        let packet = packet!(Ethernet::new(), Ip::new());
        for pdu in packet.pdu_chain {
            // println!("{:?}", pdu);
        }
        // let boxed_pdu = Box::new(Ethernet::new());
        // println!("{:?}", boxed_pdu);
        // let unboxed_pdu = boxed_pdu.as_any().downcast_ref::<Ethernet>();
        // println!("{:?}", &unboxed_pdu);
    }
}
