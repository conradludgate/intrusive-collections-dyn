use std::ptr::NonNull;

use intrusive_collections::{
    linked_list::{AtomicLink, AtomicLinkOps, Link, LinkOps, LinkedListOps},
    DefaultLinkOps,
};

use crate::{LinkedListDynLink, LinkedListDynLinkOps};

pub type AtomicDynLinkOps<D> = LinkedListDynLinkOps<AtomicLinkOps, D>;
pub type DynLinkOps<D> = LinkedListDynLinkOps<LinkOps, D>;

pub type AtomicDynLink<D> = LinkedListDynLink<AtomicLink, D>;
pub type DynLink<D> = LinkedListDynLink<Link, D>;

impl<D: ?Sized> DefaultLinkOps for AtomicDynLink<D> {
    type Ops = AtomicDynLinkOps<D>;

    const NEW: Self::Ops = AtomicDynLinkOps {
        ops: AtomicLink::NEW,
        d: std::marker::PhantomData,
    };
}

impl<D: ?Sized> Default for AtomicDynLinkOps<D> {
    fn default() -> Self {
        AtomicDynLink::NEW
    }
}

unsafe impl<D: ?Sized> intrusive_collections::LinkOps for AtomicDynLinkOps<D> {
    type LinkPtr = NonNull<AtomicDynLink<D>>;

    #[inline]
    unsafe fn acquire_link(&mut self, ptr: Self::LinkPtr) -> bool {
        self.ops.acquire_link(AtomicDynLink::<D>::to_link(ptr))
    }

    #[inline]
    unsafe fn release_link(&mut self, ptr: Self::LinkPtr) {
        self.ops.release_link(AtomicDynLink::<D>::to_link(ptr))
    }
}

unsafe impl<D: ?Sized> LinkedListOps for AtomicDynLinkOps<D> {
    #[inline]
    unsafe fn next(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .next(AtomicDynLink::to_link(ptr))
            .map(AtomicDynLink::from_link)
    }

    #[inline]
    unsafe fn set_next(&mut self, ptr: Self::LinkPtr, next: Option<Self::LinkPtr>) {
        self.ops.set_next(
            AtomicDynLink::to_link(ptr),
            next.map(AtomicDynLink::to_link),
        )
    }

    unsafe fn prev(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .prev(AtomicDynLink::to_link(ptr))
            .map(AtomicDynLink::from_link)
    }

    unsafe fn set_prev(&mut self, ptr: Self::LinkPtr, prev: Option<Self::LinkPtr>) {
        self.ops.set_prev(
            AtomicDynLink::to_link(ptr),
            prev.map(AtomicDynLink::to_link),
        )
    }
}

unsafe impl<D: ?Sized> intrusive_collections::LinkOps for DynLinkOps<D> {
    type LinkPtr = NonNull<DynLink<D>>;

    #[inline]
    unsafe fn acquire_link(&mut self, ptr: Self::LinkPtr) -> bool {
        self.ops.acquire_link(DynLink::<D>::to_link(ptr))
    }

    #[inline]
    unsafe fn release_link(&mut self, ptr: Self::LinkPtr) {
        self.ops.release_link(DynLink::<D>::to_link(ptr))
    }
}

unsafe impl<D: ?Sized> LinkedListOps for DynLinkOps<D> {
    #[inline]
    unsafe fn next(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops.next(DynLink::to_link(ptr)).map(DynLink::from_link)
    }

    #[inline]
    unsafe fn set_next(&mut self, ptr: Self::LinkPtr, next: Option<Self::LinkPtr>) {
        self.ops
            .set_next(DynLink::to_link(ptr), next.map(DynLink::to_link))
    }

    unsafe fn prev(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops.prev(DynLink::to_link(ptr)).map(DynLink::from_link)
    }

    unsafe fn set_prev(&mut self, ptr: Self::LinkPtr, prev: Option<Self::LinkPtr>) {
        self.ops
            .set_prev(DynLink::to_link(ptr), prev.map(DynLink::to_link))
    }
}
