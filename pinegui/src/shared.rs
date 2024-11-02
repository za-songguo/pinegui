extern crate alloc;

use core::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};
use critical_section::Mutex;

#[cfg(feature = "use_rc")]
use alloc::rc::Rc;

#[cfg(feature = "use_arc")]
use alloc::sync::Arc;

#[cfg(feature = "use_rc")]
pub type SmartRc<T> = Rc<T>;

#[cfg(feature = "use_arc")]
pub type SmartRc<T> = Arc<T>;

#[derive(Debug)]
pub struct Shared<T> {
    inner: SmartRc<Mutex<RefCell<T>>>,
}

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: SmartRc::new(Mutex::new(RefCell::new(value))),
        }
    }

    pub fn get_inner(&self) -> SmartRc<Mutex<RefCell<T>>> {
        self.inner.clone()
    }

    pub fn borrow_mut(&self) -> impl DerefMut<Target = T> + '_ {
        unsafe { &mut *critical_section::with(|cs| &mut *self.inner.borrow_ref_mut(cs) as *mut T) }
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*critical_section::with(|cs| &*self.inner.borrow_ref(cs) as *const T) }
    }
}

impl<T> AsRef<T> for Shared<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for Shared<T> {}

impl<T> From<T> for Shared<T> {
    fn from(value: T) -> Self {
        Shared::new(value)
    }
}

impl<T: Clone> From<&T> for Shared<T> {
    fn from(value: &T) -> Self {
        Shared::new(value.clone())
    }
}

impl<T: Clone> From<&mut T> for Shared<T> {
    fn from(value: &mut T) -> Self {
        Shared::new(value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared() {
        let shared = Shared::new(5);
        assert_eq!(*shared, 5);
        *shared.borrow_mut() = 10;
        assert_eq!(*shared, 10);
        let shared2 = shared.clone();
        assert_eq!(*shared2, 10);
        assert_eq!(*shared, 10);
    }
}
