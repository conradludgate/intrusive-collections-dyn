#![feature(arbitrary_self_types)]

use std::{
    marker::PhantomData,
    mem::offset_of,
    ptr::{addr_of_mut, NonNull},
};

pub use intrusive_collections;

pub mod linked_list;
// pub mod rbtree;
// pub mod singly_linked_list;
// pub mod xor_linked_list;

use intrusive_collections::{Adapter, LinkOps, PointerOps};

pub struct LinkedListDynLink<L, D: ?Sized> {
    link: L,
    get_value: unsafe fn(link: *mut Self) -> *const D,
}

impl<L: Default, D: ?Sized> LinkedListDynLink<L, D> {
    pub fn new<T: DynAdaptor<A>, A: Adapter>() -> Self
    where
        A::LinkOps: LinkOps<LinkPtr = NonNull<LinkedListDynLink<A::LinkOps, D>>>,
        A::PointerOps: PointerOps<Value = D>,
    {
        unsafe fn get_value<L, T: DynAdaptor<A>, A: Adapter, D: ?Sized>(
            link: *mut LinkedListDynLink<L, D>,
        ) -> *const D
        where
            A::LinkOps: LinkOps<LinkPtr = NonNull<LinkedListDynLink<A::LinkOps, D>>>,
            A::PointerOps: PointerOps<Value = D>,
        {
            <T as DynAdaptor<A>>::get_value(NonNull::new_unchecked(link.cast()))
        }

        LinkedListDynLink {
            link: L::default(),
            get_value: get_value::<L, T, A, D>,
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

    #[doc(hidden)]
    pub unsafe fn get_dyn_value(link: NonNull<Self>) -> *const D {
        let get_value = *addr_of_mut!((*link.as_ptr()).get_value);
        get_value(link.as_ptr())
    }
}

pub struct LinkedListDynLinkOps<Ops, D: ?Sized> {
    ops: Ops,
    d: PhantomData<D>,
}

impl<O: Copy, D: ?Sized> Copy for LinkedListDynLinkOps<O, D> {}
impl<O: Copy, D: ?Sized> Clone for LinkedListDynLinkOps<O, D> {
    fn clone(&self) -> Self {
        *self
    }
}

pub unsafe trait DynAdaptor<A: Adapter> {
    /// Gets a reference to the link for the given object.
    ///
    /// # Safety
    ///
    /// `value` must be a valid pointer.
    unsafe fn get_link(
        value: *const <A::PointerOps as PointerOps>::Value,
    ) -> <A::LinkOps as LinkOps>::LinkPtr;

    /// Gets a reference to an object from a reference to a link in that object.
    ///
    /// # Safety
    ///
    /// `link` must be a valid pointer previously returned by `get_link`.
    unsafe fn get_value(
        link: <A::LinkOps as LinkOps>::LinkPtr,
    ) -> *const <A::PointerOps as PointerOps>::Value;
}

/// Macro to generate an implementation of `Adapter` and `DynAdaptor` for a given set of types.
/// In particular this will automatically generate implementations of the
/// `get_value` and `get_link` methods for a given named field in a struct.
///
/// The basic syntax to create an adapter is:
///
/// ```rust,ignore
/// intrusive_dyn_adapter!(Adapter = Pointer: Value { link_field: LinkType });
/// ```
///
/// You can create a new instance of an adapter using the `new` method or the
/// `NEW` associated constant. The adapter also implements the `Default` trait.
///
/// # Generics
///
/// This macro supports generic arguments:
///
/// ```rust,ignore
/// intrusive_dyn_adapter!(
///     Adapter<'lifetime, Type, Type2> =
///         Pointer: Value {
///             link_field: LinkType
///         }
///         where
///             Type: Copy,
///             Type2: ?Sized + 'lifetime
///     );
/// ```
///
/// Note that due to macro parsing limitations, `T: Trait` bounds are not
/// supported in the generic argument list. You must list any trait bounds in
/// a separate `where` clause at the end of the macro.
///
/// # Examples
///
/// ```
/// use intrusive_collections::{LinkedListLink, RBTreeLink};
/// use intrusive_collections_dyn::intrusive_dyn_adapter;
///
/// pub struct Test {
///     link: LinkedListLink,
///     link2: RBTreeLink,
/// }
/// pub trait DynTrait {}
/// impl DynTrait for Test {}
///
/// intrusive_dyn_adapter!(MyAdapter = Arc<Test> as Arc<dyn DynTrait>: Test { link: LinkedListLink });
/// intrusive_dyn_adapter!(pub MyAdapter2 = Arc<Test> as Arc<dyn DynTrait>: Test { link2: RBTreeLink });
/// ```
#[macro_export]
macro_rules! intrusive_dyn_adapter {
    (@impl
        $(#[$attr:meta])* $vis:vis $name:ident
        = ($($args:tt),*) $data:ty as $dyn_pointer:ty:
        $value:path { $field:ident: $link:ty }
        $($where_:tt)*
    ) => {
        #[allow(explicit_outlives_requirements)]
        $(#[$attr])*
        $vis struct $name $($where_)* {
            link_ops: <$link as $crate::intrusive_collections::DefaultLinkOps>::Ops,
            pointer_ops: $crate::intrusive_collections::DefaultPointerOps<$dyn_pointer>,
        }
        unsafe impl Send for $name $($where_)* {}
        unsafe impl Sync for $name $($where_)* {}
        impl Copy for $name $($where_)* {}
        impl Clone for $name $($where_)* {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }
        impl Default for $name $($where_)* {
            #[inline]
            fn default() -> Self {
                Self::NEW
            }
        }
        #[allow(dead_code)]
        impl $name $($where_)* {
            pub const NEW: Self = $name {
                link_ops: <$link as $crate::intrusive_collections::DefaultLinkOps>::NEW,
                pointer_ops: $crate::intrusive_collections::DefaultPointerOps::<$dyn_pointer>::new(),
            };
            #[inline]
            pub fn new() -> Self {
                Self::NEW
            }
        }
        #[allow(dead_code, unsafe_code)]
        unsafe impl $crate::intrusive_collections::Adapter for $name $($where_)* {
            type LinkOps = <$link as $crate::intrusive_collections::DefaultLinkOps>::Ops;
            type PointerOps = $crate::intrusive_collections::DefaultPointerOps<$dyn_pointer>;

            #[inline]
            unsafe fn get_value(&self, link: <Self::LinkOps as $crate::intrusive_collections::LinkOps>::LinkPtr) -> *const <Self::PointerOps as $crate::intrusive_collections::PointerOps>::Value {
                $crate::LinkedListDynLink::get_dyn_value(link)
            }
            #[inline]
            unsafe fn get_link(&self, value: *const <Self::PointerOps as $crate::intrusive_collections::PointerOps>::Value) -> <Self::LinkOps as $crate::intrusive_collections::LinkOps>::LinkPtr {
                value.get_link()
            }
            #[inline]
            fn link_ops(&self) -> &Self::LinkOps {
                &self.link_ops
            }
            #[inline]
            fn link_ops_mut(&mut self) -> &mut Self::LinkOps {
                &mut self.link_ops
            }
            #[inline]
            fn pointer_ops(&self) -> &Self::PointerOps {
                &self.pointer_ops
            }
        }

        #[allow(dead_code, unsafe_code)]
        unsafe impl <$($args),*> $crate::DynAdaptor<$name> for $data $($where_)* {
            #[inline]
            unsafe fn get_value(
                link: <<$name as $crate::intrusive_collections::Adapter>::LinkOps as $crate::intrusive_collections::LinkOps>::LinkPtr
            ) -> *const <<$name as $crate::intrusive_collections::Adapter>::PointerOps as $crate::intrusive_collections::PointerOps>::Value {
                link.as_ptr().cast_const().byte_sub($crate::intrusive_collections::offset_of!($value, $field))
                    as *const $value as *const _
            }
            #[inline]
            unsafe fn get_link(
                value: *const <<$name as $crate::intrusive_collections::Adapter>::PointerOps as $crate::intrusive_collections::PointerOps>::Value
            ) -> <<$name as $crate::intrusive_collections::Adapter>::LinkOps as $crate::intrusive_collections::LinkOps>::LinkPtr {
                // We need to do this instead of just accessing the field directly
                // to strictly follow the stack borrow rules.
                let ptr = (value as *const u8).add($crate::intrusive_collections::offset_of!($value, $field));
                core::ptr::NonNull::new_unchecked(ptr as *mut _)
            }
        }
    };
    (@find_generic
        $(#[$attr:meta])* $vis:vis $name:ident = ($($prev:tt)*) > $($rest:tt)*
    ) => {
        intrusive_dyn_adapter!(@impl
            $(#[$attr])* $vis $name ($($prev)*) $($rest)*
        );
    };
    (@find_generic
        $(#[$attr:meta])* $vis:vis $name:ident = ($($prev:tt)*) $cur:tt $($rest:tt)*
    ) => {
        intrusive_dyn_adapter!(@find_generic
            $(#[$attr])* $vis $name ($($prev)* $cur) $($rest)*
        );
    };
    (@find_if_generic
        $(#[$attr:meta])* $vis:vis $name:ident = for < $($rest:tt)*
    ) => {
        intrusive_dyn_adapter!(@find_generic
            $(#[$attr])* $vis $name = () $($rest)*
        );
    };
    (@find_if_generic
        $(#[$attr:meta])* $vis:vis $name:ident = $($rest:tt)*
    ) => {
        intrusive_dyn_adapter!(@impl
            $(#[$attr])* $vis $name = () $($rest)*
        );
    };
    ($(#[$attr:meta])* $vis:vis $name:ident = $($rest:tt)*) => {
        intrusive_dyn_adapter!(@find_if_generic
            $(#[$attr])* $vis $name = $($rest)*
        );
    };
}

#[cfg(test)]
mod tests {
    use std::{ptr::NonNull, sync::Arc};

    use crate::linked_list::AtomicDynLink;
    use crate::{intrusive_dyn_adapter, DynAdaptor};

    pub struct Test {
        link: AtomicDynLink<dyn DynTrait>,
        // link2: RBTreeLink,
    }
    pub trait DynTrait {
        unsafe fn get_link(self: *const Self) -> NonNull<AtomicDynLink<dyn DynTrait>>;
    }
    impl DynTrait for Test {
        unsafe fn get_link(self: *const Self) -> NonNull<AtomicDynLink<dyn DynTrait>> {
            <Self as DynAdaptor<MyAdapter>>::get_link(self)
        }
    }

    intrusive_dyn_adapter!(MyAdapter = Test as Arc<dyn DynTrait>: Test { link: AtomicDynLink<dyn DynTrait> });

    #[test]
    fn happy() {}
}
