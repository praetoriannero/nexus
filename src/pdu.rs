use crate::error::ParseError;

use nexus_tid::Tid;
use serde_json::Value;
use serde_json::json;
use std::any::TypeId;

pub trait Pdu<'a>: Tid<'a> + 'a {
    fn from_bytes(bytes: &'a [u8]) -> PduResult<'a>
    where
        Self: Sized;

    fn clone(&self) -> Box<dyn Pdu<'static> + 'static>;

    fn collect(&self, chain: &mut Vec<Box<dyn Pdu<'static> + 'static>>) {
        chain.push(self.clone());
        match self.child_pdu() {
            Some(inner) => inner.collect(chain),
            None => (),
        }
    }

    fn to_json(&self) -> Result<Value, serde_json::error::Error>;

    fn to_bytes(&self) -> Vec<u8>;

    fn set_parent(&mut self, parent: Pob<'static>)
    where
        'a: 'static;

    fn set_child(&mut self, child: Pob<'static>)
    where
        'a: 'static;

    fn parent_pdu_mut(&mut self) -> &mut Pob<'a>;

    fn parent_pdu(&self) -> &Pob<'a>;

    fn child_pdu_mut(&mut self) -> &mut Pob<'a>;

    fn child_pdu(&self) -> &Pob<'a>;

    fn child_to_json(&self) -> serde_json::Value {
        if let Some(child) = self.child_pdu().as_ref() {
            child.to_json().unwrap()
        } else {
            json!("")
        }
    }

    fn pdu_chain(&mut self, chain: &mut Vec<TypeId>) {
        chain.push(self.self_id());
        if let Some(child) = self.child_pdu_mut() {
            child.pdu_chain(chain);
        }
    }

    fn as_pdu_mut(&mut self) -> Box<&mut dyn Pdu<'a>>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn as_pdu(&self) -> Box<&dyn Pdu<'a>>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[macro_export]
macro_rules! default_pdu_clone {
    ($struct_name:ident) => {
        fn clone(&self) -> Box<dyn Pdu<'static> + 'static> {
            Box::new($struct_name {
                header: Cow::Owned(self.header.to_vec()),
                parent: None,
                child: None,
            })
        }
    };
}

pub fn deserialize<'a, T: Pdu<'a>>(bytes: &'a [u8]) -> Option<T> {
    T::from_bytes(bytes).ok()?.downcast::<T>().map(|p| *p)
}

pub fn pdu_trait_assert<'a, T: Pdu<'a>>() {}

pub type PduBuilder = for<'a> fn(&'a [u8]) -> PduResult<'a>;

pub type PduResult<'a> = Result<Box<dyn Pdu<'a> + 'a>, ParseError>;

pub type Pob<'a> = Option<Box<dyn Pdu<'a> + 'a>>;

pub trait PobOwned {
    fn clone(&self) -> Pob<'static>;
}

impl<'a> PobOwned for Pob<'a> {
    fn clone(&self) -> Option<Box<dyn Pdu<'static> + 'static>>
    where
        Self: Sized,
    {
        match self {
            Some(boxed) => Some((*boxed).clone()),
            None => None,
        }
    }
}

impl<'a> dyn Pdu<'a> + 'a {
    pub fn find<T: Pdu<'a> + 'a>(&self) -> Option<&'a T> {
        if self.self_id() == T::id() {
            return unsafe { Some(&*(self as *const _ as *const T)) };
        } else {
            if let Some(child) = self.child_pdu() {
                return child.find::<T>();
            }
        }
        None
    }

    pub fn find_mut<T: Pdu<'a> + 'a>(&mut self) -> Option<&'a mut T> {
        if self.self_id() == T::id() {
            return unsafe { Some(&mut *(self as *mut _ as *mut T)) };
        } else {
            if let Some(child) = self.child_pdu_mut() {
                return child.find_mut::<T>();
            }
        }
        None
    }

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
