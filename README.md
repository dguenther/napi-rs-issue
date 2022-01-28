## Reproduction steps

1. Run `yarn install` to install packages.
2. Run `yarn build` to build the module.
3. Run `yarn test` to execute `test.js`.

## Observed output

You should expect to see something like this:

```
$ cross-env RUST_BACKTRACE=1 node test.js
thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 0', C:\Users\derek\.cargo\registry\src\github.com-1ecc6299db9ec823\napi-2.0.3\src\bindgen_runtime\js_values\bigint.rs:82:7
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\std\src\panicking.rs:498
   1: core::panicking::panic_fmt
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\core\src\panicking.rs:107
   2: core::panicking::panic_bounds_check
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b\/library\core\src\panicking.rs:75
   3: napi::bindgen_runtime::js_values::bigint::BigInt::get_u64
   4: core::ptr::drop_in_place<alloc::vec::Vec<&napi::js_values::object_property::Property>>
   5: node::Stop
   6: v8::internal::Builtins::code_handle
   7: v8::internal::Builtins::code_handle
   8: v8::internal::Builtins::code_handle
   9: v8::internal::Builtins::code_handle
  10: v8::internal::SetupIsolateDelegate::SetupHeap
  11: v8::internal::SetupIsolateDelegate::SetupHeap
  12: v8::internal::SetupIsolateDelegate::SetupHeap
  13: v8::internal::SetupIsolateDelegate::SetupHeap
  14: v8::internal::SetupIsolateDelegate::SetupHeap
  15: v8::internal::SetupIsolateDelegate::SetupHeap
  16: v8::internal::SetupIsolateDelegate::SetupHeap
  17: v8::internal::SetupIsolateDelegate::SetupHeap
  18: v8::internal::SetupIsolateDelegate::SetupHeap
  19: v8::internal::SetupIsolateDelegate::SetupHeap
  20: v8::internal::Execution::CallWasm
  21: v8::internal::Execution::Call
  22: v8::Function::Call
  23: node_api_get_module_file_name
  24: node::Start
  25: node::Start
  26: node::LoadEnvironment
  27: cppgc::internal::NormalPageSpace::linear_allocation_buffer
  28: node::Start
  29: RC4_options
  30: v8::internal::compiler::RepresentationChanger::Uint32OverflowOperatorFor
  31: BaseThreadInitThunk
  32: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error Command failed with exit code 3221226505.
```