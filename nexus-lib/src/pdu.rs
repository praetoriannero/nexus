use crate::error::ParseError;
use std::any::TypeId;

pub trait Pdu<'a>: 'a {
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn to_bytes(&self) -> Vec<u8>;

    fn parent_pdu(&self) -> &Pob<'a>;

    fn child_pdu(&self) -> &Pob<'a>;

    fn pdu_chain(&self, chain: &mut Vec<TypeId>) {
        chain.push(self.self_id());
        if let Some(child) = self.child_pdu() {
            child.pdu_chain(chain);
        }
    }

    fn self_id(&self) -> TypeId;

    fn id() -> TypeId
    where
        Self: Sized;
}

pub type Pob<'a> = Option<Box<dyn Pdu<'a> + 'a>>;

impl<'a> dyn Pdu<'a> + 'a {
    pub fn downcast_ref<T: Pdu<'a> + 'a>(&self) -> Option<&'a T> {
        if self.self_id() == T::id() {
            unsafe { Some(&*(self as *const _ as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Pdu<'a> + 'a>(&mut self) -> Option<&'a mut T> {
        if self.self_id() == T::id() {
            unsafe { Some(&mut *(self as *mut _ as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast<T: Pdu<'a> + 'a>(self: Box<Self>) -> Option<Box<T>>
    where
        T: 'a,
    {
        if self.self_id() == T::id() {
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
