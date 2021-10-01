//! Closures: `(fn(EXINF), EXINF)`
use crate::abi;
use core::mem::MaybeUninit;

/// A bundle of a function pointer and associated data.
///
/// # Safety
///
/// When calling the function pointer, the second value must be provided as
/// the parameter.
///
/// If the original closure was `!Send`, it can only be called from the creator
/// thread.
pub type Closure = (unsafe extern "C" fn(abi::EXINF), abi::EXINF);

/// Conversion to [`Closure`].
pub trait IntoClosure {
    /// Convert `self` to `Closure`.
    fn into_closure(self) -> Closure;
}

/// Trivial conversion.
impl IntoClosure for (extern "C" fn(abi::EXINF), abi::EXINF) {
    #[inline]
    fn into_closure(self) -> Closure {
        (self.0, self.1)
    }
}

/// # Example
///
/// ```
/// use itron::closure::IntoClosure;
/// let (fp, data) = (|| dbg!()).into_closure();
/// unsafe { fp(data) };
///
/// let captured_value = 42u16;
/// let (fp, data) = (move || { assert_eq!(captured_value, 42); }).into_closure();
/// unsafe { fp(data) };
///
/// let captured_value = &"hello";
/// let (fp, data) = (move || { assert_eq!(*captured_value, "hello"); }).into_closure();
/// unsafe { fp(data) };
/// ```
///
/// The source type must fit in [`abi::EXINF`]:
///
/// ```compile_fail
/// # use itron::closure::IntoClosure;
/// let captured_value = [0usize; 2]; // too large!
/// let _ = (move || { dbg!(captured_value); }).into_closure();
/// ```
///
/// The source type must not contain a reference to a local variable:
///
/// ```compile_fail
/// # use itron::closure::IntoClosure;
/// let captured_value = 42usize;
/// let _ = (|| { dbg!(&captured_value); }).into_closure(); // capturing by reference
/// ```
impl<T: Fn() + Copy + 'static> IntoClosure for T {
    #[inline]
    fn into_closure(self) -> Closure {
        // Make sure `T` fits
        trait AssertSize {
            const X: ();
        }
        impl<T> AssertSize for T {
            const X: () = if core::mem::size_of::<T>() > core::mem::size_of::<abi::EXINF>() {
                let zero = 0;
                // compile-time panicking is not stable yet
                #[allow(unconditional_panic)]
                #[allow(non_snake_case)]
                let __T_is_too_large_to_fit_in_EXINF__ = 1 / zero;
                #[allow(clippy::empty_loop)]
                loop {}
            };
        }
        let () = <T as AssertSize>::X;

        extern "C" fn trampoline<T: Fn() + Copy + 'static>(x: abi::EXINF) {
            // Safety: `x` is a reinterpretation of the original `T`. This
            //         function reconstitutes `T` every time it's called, but
            //         this is safe because `T: Copy`.
            let t: T = unsafe { core::mem::transmute_copy(&x) };
            t();
        }

        // Makes sure the transmutation source type is large enough to
        // cover `EXINF` as required by `transmute_copy`.
        #[repr(C)]
        struct PadWithZero<T> {
            x: MaybeUninit<T>,
            zero: MaybeUninit<abi::EXINF>,
        }

        (trampoline::<T>, unsafe {
            core::mem::transmute_copy(&PadWithZero {
                x: MaybeUninit::new(self),
                zero: MaybeUninit::uninit(),
            })
        })
    }
}
