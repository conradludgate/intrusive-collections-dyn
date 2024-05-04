use crate::{LinkedListDynLink, LinkedListDynLinkOps};


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
