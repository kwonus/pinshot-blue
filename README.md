# pinshot-blue

Formerly, pin-shot-avx, pinshot-blue is a library version of the the pinshot-svc microservice. Where it differs is that rather than delivering peg parses via REST, it returns parses using Rust's FFI.
Cosequently, the pinshot-blue library can be dynamically loaded by any language that can link with a C library (.dll or .so). This simplifies interop, and is the preferred method for executing PEG parses from the [blueprint-blue libary](https://github.com/kwonus/blueprint-blue/tree/main/Blueprint-Blue-Lib) [C# 7]. 

The other difference is that VanillaQuelle implementation is not in the repo (Only AVX-Quelle is included; thus the name).

A nuisance is that the Quelle grammar is defined in both repos, and neither is a fork of the other. This is a minor irritation that will
not likely be resolved any time in the near future.
