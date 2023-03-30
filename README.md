# pin-shot-avx

pin-shot-avx is effectively a fork of the pin-shot-blue microservice. Where it differs is that rather delivering peg parses via Rust, it returns parses using Rust's FFI.
Therefore, the pin-shot-avx library can be dynamically loaded by any language that can link with a C library (.dll or .so). This simplifies interop, and is the preferred
method for executing PEG parses from the blueprint-blue libary [C# 7].

The other difference is that VanillaQuelle implementation is not in the repo (Only AVX-Quelle is included; thus the name).

A nuisance is that AVX-Quelle is defined in both repos and this repo is not a true fork. This is a minor irritation that will
not likely be resolved any time in the near future.
