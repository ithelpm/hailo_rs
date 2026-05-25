# hailo-rs

Rust FFI bindings and safe device abstractions for the [Hailo AI accelerator](https://hailo.ai/) via the HailoRT C API.

Targets **aarch64** (e.g. Rockchip RK3588) with cross-compilation support via [`cross`](https://github.com/cross-rs/cross).

## Crate layout

```
hailo_rs/
├── src/
│   ├── lib.rs              # Public re-exports
│   ├── hailo_ffi/          # Raw bindgen-generated FFI (hailo_sys)
│   │   └── mod.rs
│   ├── hw/                 # Hardware abstraction
│   │   ├── device.rs       # HailoDevice  — PCIe device scan & connect
│   │   └── vdevice.rs      # HailoVDevice — virtual device (auto-binds to chip)
│   └── infer/              # Inference pipeline
│       ├── hef.rs          # HailoHef     — load .hef model from file or buffer
│       ├── network.rs      # HailoNetworkGroup — configure network onto device
│       └── vstream.rs      # HailoVStreams — read/write raw inference tensors
├── examples/
│   └── basic_inference.rs  # Minimal end-to-end example
├── aarch64_sysroot/lib/    # Pre-built libhailort.so (committed for cross-builds)
├── wrapper.h               # bindgen entry point
└── build.rs                # Generates FFI bindings via bindgen
```

## Prerequisites

| Requirement | Notes |
|---|---|
| HailoRT SDK | Headers must be on the include path (see `build.rs`) |
| `libhailort.so` | Committed under `aarch64_sysroot/lib/` for cross-builds; must be present on target at runtime |
| Rust `aarch64-unknown-linux-gnu` toolchain | Via `rustup target add aarch64-unknown-linux-gnu` |
| [`cross`](https://github.com/cross-rs/cross) | For cross-compilation (`cargo install cross`) |

## Build

### Cross-compile for aarch64

```bash
cross build --release --target aarch64-unknown-linux-gnu
```

The custom `Dockerfile` installs `clang` and copies `libhailort.so` into the
cross-compilation sysroot automatically.

### Native build (on aarch64 target)

```bash
cargo build --release
```

Ensure `libhailort.so` is accessible (e.g. in `/usr/lib/` or `LD_LIBRARY_PATH`).

## Usage

```rust
use hailo_rs::hw::HailoVDevice;
use hailo_rs::infer::{HailoHef, HailoNetworkGroup, HailoVStreams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdevice = HailoVDevice::new()?;
    let hef     = HailoHef::from_file("model.hef")?;
    let network = HailoNetworkGroup::configure(&vdevice, &hef)?;
    let vstreams = HailoVStreams::create(&network)?;

    // Write input tensor, read output tensor
    vstreams.write_input(input.as_ptr(), input.len())?;
    vstreams.read_output(output.as_mut_ptr(), output.len())?;
    Ok(())
}
```

See [examples/basic_inference.rs](hailo_rs/examples/basic_inference.rs) for the full annotated example.

## License

MIT
