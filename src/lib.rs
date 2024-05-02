use std::{marker::PhantomData, mem::offset_of, ptr::NonNull};

pub use intrusive_collections;

use intrusive_collections::DefaultLinkOps;

pub struct LinkedListDynLink<L, D: ?Sized> {
    link: L,
    get_value: unsafe fn(link: NonNull<LinkedListDynLink<L, D>>) -> *const D,
}

impl<L: Default, D: ?Sized> LinkedListDynLink<L, D> {
    pub fn new<A: DynAdaptor<L, D>>() -> Self {
        LinkedListDynLink {
            link: L::default(),
            get_value: A::get_value,
        }
    }
}

impl<L, D: ?Sized> LinkedListDynLink<L, D> {
    #[inline]
    fn to_link(ptr: NonNull<Self>) -> NonNull<L> {
        let offset = offset_of!(LinkedListDynLink::<L, D>, link);
        unsafe { NonNull::new_unchecked(ptr.as_ptr().byte_add(offset).cast()) }
    }

    #[inline]
    fn from_link(link: NonNull<L>) -> NonNull<Self> {
        let offset = offset_of!(LinkedListDynLink::<L, D>, link);
        unsafe { NonNull::new_unchecked(link.as_ptr().byte_sub(offset).cast()) }
    }
}

impl<L: DefaultLinkOps, D: ?Sized> DefaultLinkOps for LinkedListDynLink<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>,
{
    type Ops = LinkedListDynLinkOps<L, D>;

    const NEW: Self::Ops = LinkedListDynLinkOps {
        ops: <L as intrusive_collections::DefaultLinkOps>::NEW,
        d: PhantomData,
    };
}

pub struct LinkedListDynLinkOps<L: DefaultLinkOps, D: ?Sized> {
    ops: L::Ops,
    d: PhantomData<D>,
}

impl<L: DefaultLinkOps, D: ?Sized> Copy for LinkedListDynLinkOps<L, D> where L::Ops: Copy {}
impl<L: DefaultLinkOps, D: ?Sized> Clone for LinkedListDynLinkOps<L, D>
where
    L::Ops: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: DefaultLinkOps, D: ?Sized> Default for LinkedListDynLinkOps<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>,
{
    fn default() -> Self {
        <LinkedListDynLink<L, D> as DefaultLinkOps>::NEW
    }
}

unsafe impl<L: DefaultLinkOps, D: ?Sized> intrusive_collections::LinkOps
    for LinkedListDynLinkOps<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>,
{
    type LinkPtr = NonNull<LinkedListDynLink<L, D>>;

    #[inline]
    unsafe fn acquire_link(&mut self, ptr: Self::LinkPtr) -> bool {
        self.ops
            .acquire_link(LinkedListDynLink::<L, D>::to_link(ptr))
    }

    #[inline]
    unsafe fn release_link(&mut self, ptr: Self::LinkPtr) {
        self.ops
            .release_link(LinkedListDynLink::<L, D>::to_link(ptr))
    }
}

unsafe impl<L: DefaultLinkOps, D: ?Sized> intrusive_collections::xor_linked_list::XorLinkedListOps
    for LinkedListDynLinkOps<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>
        + intrusive_collections::xor_linked_list::XorLinkedListOps,
{
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

pub unsafe trait DynAdaptor<L, D: ?Sized> {
    /// Gets a reference to the link for the given object.
    ///
    /// # Safety
    ///
    /// `value` must be a valid pointer.
    unsafe fn get_link(value: *const Self) -> NonNull<LinkedListDynLink<L, D>>;

    /// Gets a reference to an object from a reference to a link in that object.
    ///
    /// # Safety
    ///
    /// `link` must be a valid pointer previously returned by `get_link`.
    unsafe fn get_value(ptr: NonNull<LinkedListDynLink<L, D>>) -> *const D;
}
