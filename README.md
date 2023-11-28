# Shuttle-CCH23
[Shuttle CCH23](https://www.shuttle.rs/cch). Going wild with trying different features and tools.

## Tools & Setup
Using many compiler boosting things in nightly:
- [mold](https://github.com/rui314/mold) linker
- [cranelift](https://github.com/rust-lang/rustc_codegen_cranelift) backend
- [parallel frontend](https://blog.rust-lang.org/2023/11/09/parallel-rustc.html)

This cuts the compilation time to about 30% (full: 24s -> 6s, incremental: 1.6s -> 0.6s) biggest improvement was `cranelift` and `mold`. Could not see a big change in the parallel frontend (but no regression so I kept it). *Running Arch on a AMD 7940HS with 64 GB memory.*

Shuttle deploy uses stable Rust and I could not find a good way of ignoring unstable features when using this so I have added `.cargo` to `.gitignore` where I keep all the above features.

#### just
[just](https://github.com/casey/just) for short commands. Listing all my options with `just`.

#### zellij
[zellij](https://github.com/zellij-org/zellij) with layouts to get multiple panes to run the server and tests together for a more comfortable development view.

#### hurl
[hurl](https://github.com/Orange-OpenSource/hurl) for running tests.