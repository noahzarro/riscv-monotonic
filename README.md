# `riscv-monotonic`

This crate implements the monotonic trait from [this repository](https://docs.rs/rtic-monotonic/1.0.0/rtic_monotonic/) with the built-in timer in RISC-V.
It uses [fugit](https://docs.rs/crate/fugit/latest) as the unterlying time library.

It is closly releated to the [monotonic implementation](https://docs.rs/systick-monotonic/1.0.1/systick_monotonic/) inside the systick.
