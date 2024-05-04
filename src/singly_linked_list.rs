use crate::LinkedListDynLink;

unsafe impl<L: DefaultLinkOps, D: ?Sized>
    intrusive_collections::singly_linked_list::SinglyLinkedListOps for LinkedListDynLinkOps<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>
        + intrusive_collections::singly_linked_list::SinglyLinkedListOps,
{
    #[inline]
    unsafe fn next(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .next(LinkedListDynLink::to_link(ptr))
            .map(LinkedListDynLink::from_link)
    }

    #[inline]
    unsafe fn set_next(&mut self, ptr: Self::LinkPtr, next: Option<Self::LinkPtr>) {
        self.ops.set_next(
            LinkedListDynLink::to_link(ptr),
            next.map(LinkedListDynLink::to_link),
        )
    }
}
