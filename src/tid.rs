use std::any::TypeId;

pub trait Tid<'a>: 'a {
    fn self_id(&self) -> TypeId;
    fn id() -> TypeId
    where
        Self: Sized;
}

impl<'a> dyn Tid<'a> + 'a {
    pub fn downcast_ref<T: Tid<'a> + 'a>(&self) -> Option<&'a T> {
        if self.self_id() == T::id() {
            unsafe { Some(&*(self as *const _ as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Tid<'a> + 'a>(&mut self) -> Option<&'a mut T> {
        if self.self_id() == T::id() {
            unsafe { Some(&mut *(self as *mut _ as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast<T: Tid<'a> + 'a>(self: Box<Self>) -> Option<Box<T>>
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
