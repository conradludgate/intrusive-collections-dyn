use crate::LinkedListDynLink;

unsafe impl<L: DefaultLinkOps, D: ?Sized> intrusive_collections::rbtree::RBTreeOps
    for LinkedListDynLinkOps<L, D>
where
    L::Ops: intrusive_collections::LinkOps<LinkPtr = NonNull<L>>
        + intrusive_collections::rbtree::RBTreeOps,
{
    unsafe fn left(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .left(LinkedListDynLink::to_link(ptr))
            .map(LinkedListDynLink::from_link)
    }

    unsafe fn right(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .right(LinkedListDynLink::to_link(ptr))
            .map(LinkedListDynLink::from_link)
    }

    unsafe fn parent(&self, ptr: Self::LinkPtr) -> Option<Self::LinkPtr> {
        self.ops
            .parent(LinkedListDynLink::to_link(ptr))
            .map(LinkedListDynLink::from_link)
    }

    unsafe fn color(&self, ptr: Self::LinkPtr) -> intrusive_collections::rbtree::Color {
        self.ops.color(LinkedListDynLink::to_link(ptr))
    }

    unsafe fn set_left(&mut self, ptr: Self::LinkPtr, left: Option<Self::LinkPtr>) {
        self.ops.set_left(
            LinkedListDynLink::to_link(ptr),
            left.map(LinkedListDynLink::to_link),
        )
    }

    unsafe fn set_right(&mut self, ptr: Self::LinkPtr, right: Option<Self::LinkPtr>) {
        self.ops.set_right(
            LinkedListDynLink::to_link(ptr),
            right.map(LinkedListDynLink::to_link),
        )
    }

    unsafe fn set_parent(&mut self, ptr: Self::LinkPtr, parent: Option<Self::LinkPtr>) {
        self.ops.set_parent(
            LinkedListDynLink::to_link(ptr),
            parent.map(LinkedListDynLink::to_link),
        )
    }

    unsafe fn set_color(
        &mut self,
        ptr: Self::LinkPtr,
        color: intrusive_collections::rbtree::Color,
    ) {
        self.ops.set_color(LinkedListDynLink::to_link(ptr), color)
    }
}
