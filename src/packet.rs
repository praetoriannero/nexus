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
    }
}
