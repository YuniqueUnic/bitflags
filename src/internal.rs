//! Generate the internal `bitflags`-facing flags type.
//!
//! The code generated here is owned by `bitflags`, but still part of its public API.
//! Changes to the types generated here need to be considered like any other public API change.

/// Declare the `bitflags`-facing bitflags struct.
///
/// This type is part of the `bitflags` crate's public API, but not part of the user's.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __declare_internal_bitflags {
    (
        $vis:vis struct $InternalBitFlags:ident: $T:ty;
    ) => {
        // NOTE: The ABI of this type is _guaranteed_ to be the same as `T`
        // This is relied on by some external libraries like `bytemuck` to make
        // its `unsafe` trait impls sound.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $InternalBitFlags($T);
    };
}

/// Implement functions on the private (bitflags-facing) bitflags type.
///
/// Methods and trait implementations can be freely added here without breaking end-users.
/// If we want to expose new functionality to `#[derive]`, this is the place to do it.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_internal_bitflags {
    (
        $InternalBitFlags:ident: $T:ty, $PublicBitFlags:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )*
        }
    ) => {
        impl $crate::__private::PublicFlags for $PublicBitFlags {
            type Internal = $InternalBitFlags;
        }

        impl $crate::__private::core::default::Default for $InternalBitFlags {
            #[inline]
            fn default() -> Self {
                $InternalBitFlags::empty()
            }
        }

        impl $crate::__private::core::fmt::Debug for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result {
                if self.is_empty() {
                    // If no flags are set then write an empty hex flag to avoid
                    // writing an empty string. In some contexts, like serialization,
                    // an empty string is preferrable, but it may be unexpected in
                    // others for a format not to produce any output.
                    //
                    // We can remove this `0x0` and remain compatible with `FromStr`,
                    // because an empty string will still parse to an empty set of flags,
                    // just like `0x0` does.
                    $crate::__private::core::write!(f, "{:#x}", <$T as $crate::Bits>::EMPTY)
                } else {
                    $crate::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }

        impl $crate::__private::core::fmt::Display for $InternalBitFlags {
            fn fmt(&self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result {
                $crate::parser::to_writer(&$PublicBitFlags(*self), f)
            }
        }

        impl $crate::__private::core::str::FromStr for $InternalBitFlags {
            type Err = $crate::parser::ParseError;

            fn from_str(s: &str) -> $crate::__private::core::result::Result<Self, Self::Err> {
                $crate::parser::from_str::<$PublicBitFlags>(s).map(|flags| flags.0)
            }
        }

        impl $crate::__private::core::convert::AsRef<$T> for $InternalBitFlags {
            fn as_ref(&self) -> &$T {
                &self.0
            }
        }

        impl $crate::__private::core::convert::From<$T> for $InternalBitFlags {
            fn from(bits: $T) -> Self {
                Self::from_bits_retain(bits)
            }
        }

        __impl_public_bitflags! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    $Flag;
                )*
            }
        }

        __impl_public_bitflags_consts! {
            $InternalBitFlags: $T {
                $(
                    $(#[$attr $($args)*])*
                    #[allow(
                        dead_code,
                        deprecated,
                        unused_attributes,
                        non_upper_case_globals
                    )]
                    $Flag = $value;
                )*
            }
        }

        impl $InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut $T {
                &mut self.0
            }

            /// Iterate over enabled flag values.
            #[inline]
            pub const fn iter(&self) -> $crate::iter::Iter<$PublicBitFlags> {
                $crate::iter::Iter::__private_const_new(<$PublicBitFlags as $crate::Flags>::FLAGS, $PublicBitFlags::from_bits_retain(self.0), $PublicBitFlags::from_bits_retain(self.0))
            }

            /// Iterate over enabled flag values with their stringified names.
            #[inline]
            pub const fn iter_names(&self) -> $crate::iter::IterNames<$PublicBitFlags> {
                $crate::iter::IterNames::__private_const_new(<$PublicBitFlags as $crate::Flags>::FLAGS, $PublicBitFlags::from_bits_retain(self.0), $PublicBitFlags::from_bits_retain(self.0))
            }
        }
    };
}
