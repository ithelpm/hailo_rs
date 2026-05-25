// Entry point for bindgen — generates Rust FFI bindings from the HailoRT C API.
// Header search paths are injected by build.rs:
//   -I/usr/include
//   -I<workspace_root>                    (resolves <hailo/hailort.h>)
//   -I<manifest>/aarch64_sysroot/include  (cross-compilation sysroot)
#include <hailo/hailort.h>