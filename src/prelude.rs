pub(crate) extern crate alloc;

pub(crate) use core::alloc::Layout;
pub(crate) use core::cell::Cell;
pub(crate) use core::cmp::max;
pub(crate) use core::fmt;
pub(crate) use core::iter::FusedIterator;
pub(crate) use core::marker::PhantomData;
pub(crate) use core::mem::ManuallyDrop;
pub(crate) use core::mem::MaybeUninit;
pub(crate) use core::mem;
pub(crate) use core::num::NonZeroU128;
pub(crate) use core::num::NonZeroU64;
pub(crate) use core::ops::Index;
pub(crate) use core::ops::IndexMut;
pub(crate) use core::ptr::NonNull;
pub(crate) use core::ptr;
pub(crate) use crate::ptr::Ptr;
pub(crate) use crate::rng::Rng;
pub(crate) use crate::rng;

#[inline(always)]
pub(crate) unsafe fn assume(p: bool) {
  if ! p { unsafe { core::hint::unreachable_unchecked() } }
}

#[inline(always)]
#[cold]
pub(crate) fn cold() {}

#[inline(always)]
pub(crate) fn expect(p: bool, q: bool) -> bool {
  if p != q { cold() }
  p
}
