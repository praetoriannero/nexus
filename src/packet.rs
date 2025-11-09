use crate::pdu::Pdu;

#[macro_export]
macro_rules! packet {
    ( $( $pdu:expr ),* $(,)? ) => {{
        let mut v: Vec<Box<dyn Pdu>> = Vec::new();
        $( v.push(Box::new($pdu)); )*
        v
    }};
}

pub struct Packet<'a> {
    pub pdu_chain: Vec<Box<dyn Pdu<'a>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ethernet::Ethernet;
    use crate::ip::Ip;

    #[test]
    fn test_pdu() {
        packet!(Ethernet::new(), Ip::new());
        let boxed_pdu = Box::new(Ethernet::new());
        println!("{:?}", boxed_pdu);
        let unboxed_pdu = boxed_pdu.as_any().downcast_ref::<Ethernet>();
        // println!("{:?}", &unboxed_pdu);
    }
}
