use std::{marker::PhantomData, mem::offset_of, ptr::NonNull};

pub use intrusive_collections;

use intrusive_collections::{
    xor_linked_list::XorLinkedListOps, DefaultLinkOps, XorLinkedListAtomicLink,
};

pub struct LinkedListDynLink<D: ?Sized> {
    link: XorLinkedListAtomicLink,
    get_value: unsafe fn(link: NonNull<LinkedListDynLink<D>>) -> *const D,
}

impl<D: ?Sized> LinkedListDynLink<D> {
    pub const fn new<L: DynAdaptor<D>>() -> Self {
        LinkedListDynLink {
            link: XorLinkedListAtomicLink::new(),
            get_value: L::get_value,
        }
    }

    #[inline]
    fn to_link(ptr: NonNull<Self>) -> NonNull<XorLinkedListAtomicLink> {
        let offset = offset_of!(LinkedListDynLink::<D>, link);
        unsafe { NonNull::new_unchecked(ptr.as_ptr().byte_add(offset).cast()) }
    }

    #[inline]
    fn from_link(link: NonNull<XorLinkedListAtomicLink>) -> NonNull<Self> {
        let offset = offset_of!(LinkedListDynLink::<D>, link);
        unsafe { NonNull::new_unchecked(link.as_ptr().byte_sub(offset).cast()) }
    }
}

impl<D: ?Sized> DefaultLinkOps for LinkedListDynLink<D> {
    type Ops = LinkedListDynLinkOps<D>;

    const NEW: Self::Ops = LinkedListDynLinkOps {
        ops: <XorLinkedListAtomicLink as intrusive_collections::DefaultLinkOps>::NEW,
        d: PhantomData,
    };
}

pub struct LinkedListDynLinkOps<D: ?Sized> {
    ops: intrusive_collections::xor_linked_list::AtomicLinkOps,
    d: PhantomData<D>,
}

impl<D: ?Sized> Copy for LinkedListDynLinkOps<D> {}
impl<D: ?Sized> Clone for LinkedListDynLinkOps<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: ?Sized> Default for LinkedListDynLinkOps<D> {
    fn default() -> Self {
        <LinkedListDynLink<D> as DefaultLinkOps>::NEW
    }
}

unsafe impl<D: ?Sized> intrusive_collections::LinkOps for LinkedListDynLinkOps<D> {
    type LinkPtr = NonNull<LinkedListDynLink<D>>;

    #[inline]
    unsafe fn acquire_link(&mut self, ptr: Self::LinkPtr) -> bool {
        self.ops.acquire_link(LinkedListDynLink::to_link(ptr))
    }

    #[inline]
    unsafe fn release_link(&mut self, ptr: Self::LinkPtr) {
        self.ops.release_link(LinkedListDynLink::to_link(ptr))
    }
}

unsafe impl<D: ?Sized> XorLinkedListOps for LinkedListDynLinkOps<D> {
    #[inline]
    unsafe fn next(
        &self,
        ptr: Self::LinkPtr,
        prev: Option<Self::LinkPtr>,
    ) -> Option<Self::LinkPtr> {
        self.ops
            .next(
                LinkedListDynLink::to_link(ptr),
                prev.map(LinkedListDynLink::to_link),
            )
            .map(LinkedListDynLink::from_link)
    }

    #[inline]
    unsafe fn prev(
        &self,
        ptr: Self::LinkPtr,
        next: Option<Self::LinkPtr>,
    ) -> Option<Self::LinkPtr> {
        self.ops
            .prev(
                LinkedListDynLink::to_link(ptr),
                next.map(LinkedListDynLink::to_link),
            )
            .map(LinkedListDynLink::from_link)
    }

    #[inline]
    unsafe fn set(
        &mut self,
        ptr: Self::LinkPtr,
        prev: Option<Self::LinkPtr>,
        next: Option<Self::LinkPtr>,
    ) {
        self.ops.set(
            LinkedListDynLink::to_link(ptr),
            prev.map(LinkedListDynLink::to_link),
            next.map(LinkedListDynLink::to_link),
        )
    }

    #[inline]
    unsafe fn replace_next_or_prev(
        &mut self,
        ptr: Self::LinkPtr,
        old: Option<Self::LinkPtr>,
        new: Option<Self::LinkPtr>,
    ) {
        self.ops.replace_next_or_prev(
            LinkedListDynLink::to_link(ptr),
            old.map(LinkedListDynLink::to_link),
            new.map(LinkedListDynLink::to_link),
        )
    }
}

pub unsafe trait DynAdaptor<D: ?Sized> {
    /// Gets a reference to the link for the given object.
    ///
    /// # Safety
    ///
    /// `value` must be a valid pointer.
    unsafe fn get_link(value: *const Self) -> NonNull<LinkedListDynLink<D>>;

    /// Gets a reference to an object from a reference to a link in that object.
    ///
    /// # Safety
    ///
    /// `link` must be a valid pointer previously returned by `get_link`.
    unsafe fn get_value(ptr: NonNull<LinkedListDynLink<D>>) -> *const D;
}
