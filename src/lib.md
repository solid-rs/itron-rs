Rust interface to interact with operating systems based on μITRON and its derivatives.

*This crate currently requires the recently-added [`extended_key_value_attributes`] compiler feature, which is only stabilized in the nightly compiler (at the point of writing).*

[`extended_key_value_attributes`]: https://caniuse.rs/features/extended_key_value_attrs

## Kernel Selection

This crate exposes a Cargo feature for each supported RTOS kernel
implementation. The following ones are supported:

 - `asp3`: [TOPPERS/ASP3](https://toppers.jp/asp3-kernel.html)
     - Additional features supported: `dcre` (dynamic object creation), `rstr_task` (restricted tasks), `messagebuf` (message buffers), `ovrhdr` (overrun handlers), `subprio` (task subpriorities), `pi_mutex` (priority inheritance; this is a [SOLID] extension)
 - `fmp3`: [TOPPERS/FMP3](https://toppers.jp/fmp3-kernel.html)
 - `none` (default): Stub implementation that exposes all functions but always panics

It's an error to enable more than one of these features. It's unsafe to specify an incorrect kernel because the ABIs differ between kernels. This crate assumes it's inherently safe to call the specified kernel's API functions (provided the usage is correct).

Items are `cfg`-gated according to the selected kernel's supported feature set so that the uses of non-existent features are detected at compile time.

[SOLID]: https://solid.kmckk.com/SOLID/

## Cargo Features

In addition to the kernel selection features described above, this package
supports the following Cargo features:

 - `nightly` enables nightly-only features. Currently, this feature enables the use of [`doc_cfg`] and exposes `itron::time::timeout!`.
 - `unstable` enables unstable (in terms of API stability), experimental features that may be changed or removed in the future.

[`doc_cfg`]: https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html

## API Design

*This section is relevant only when the `unstable` feature is enabled.*

### Object ID Wrappers

Kernel object IDs are encapsulated in opaque wrappers, which can be constructed either by calling the creation methods or by converting from raw object IDs. Although interacting with arbitrary kernel objects do not exhibit memory unsafety by itself, conversion from raw object IDs has to go through `unsafe` calls because the created wrappers could be used to interfere with other code's usage of such objects, breaking its assumptions, possibly violating memory safety. Deleting unowned objects is `unsafe` as well because such objects could be still in use by their actual owners, and the actual owners would touch supposedly-unrelated objects if the IDs were reused.

It's allowed to [get] an object ID wrapper for the current task of the current processor. However, the wrapper created in this way must not outlive the originating task.

[get]: crate::task::current

### Kernel Assumed to be Operational

**It's assumed that this crate's functions are called while the kernel is operational** (i.e., `sns_ker` returns `FALSE`). It's up to application programmers to make sure they are not called inside initialization or termination routines.
