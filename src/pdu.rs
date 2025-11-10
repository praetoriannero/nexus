use crate::error::ParseError;

pub trait Pdu<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;

    fn parent_pdu(&self) -> &Pob<'a>;
    fn child_pdu(&self) -> &Pob<'a>;

    fn pdu_type(&self) -> PduType;

    fn dyn_pdu_kind(&self) -> PduKind;
    fn static_pdu_kind() -> PduKind
    where
        Self: Sized;
}

pub type Pob<'a> = Option<Box<dyn Pdu<'a> + 'a>>;

pub enum PduType {
    Ethernet,
    Ip,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PduKind(pub fn());

impl<'a> dyn Pdu<'a> {
    pub fn downcast_ref<T: Pdu<'a>>(&self) -> Option<&T> {
        if self.dyn_pdu_kind() == T::static_pdu_kind() {
            unsafe { Some(&*(self as *const dyn Pdu<'a> as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Pdu<'a>>(&mut self) -> Option<&mut T> {
        if self.dyn_pdu_kind() == T::static_pdu_kind() {
            unsafe { Some(&mut *(self as *mut dyn Pdu<'a> as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast<T: Pdu<'a>>(self: Box<Self>) -> Option<Box<T>> {
        if self.dyn_pdu_kind() == T::static_pdu_kind() {
            let raw = Box::into_raw(self);
            unsafe { Some(Box::from_raw(raw as *mut T)) }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ethernet::Ethernet;
    use crate::ip::Ip;

    #[test]
    fn test_downcast() {
        let eth: Box<dyn Pdu> = Box::new(Ethernet::new());
        let res = eth.downcast::<Ethernet>();
        assert!(res.is_some());

        let eth_inv: Box<dyn Pdu> = Box::new(Ip::new());
        let res = eth_inv.downcast::<Ethernet>();
        assert!(!res.is_some());
    }

    #[test]
    fn test_downcast_mut() {
        let mut eth: Box<dyn Pdu> = Box::new(Ethernet::new());
        let res = eth.downcast_mut::<Ethernet>();
        assert!(res.is_some());

        let eth_inv: Box<dyn Pdu> = Box::new(Ip::new());
        let res = eth_inv.downcast_ref::<Ethernet>();
        assert!(!res.is_some());
    }
}
