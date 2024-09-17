# `rust-vfs` on `wasm32-unknown-unknown`

Reproduces an issue with `rust-vfs` on `wasm32-unknown-unknown`.
See https://github.com/manuel-woelker/rust-vfs/issues/68 for details.

Requires [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to be installed.

## Usage

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox # or --chrome
```

### Error logs

```
running 1 test

test web::pass ... FAIL

failures:

---- web::pass output ----
    error output:
        panicked at library/std/src/sys/pal/wasm/../unsupported/time.rs:31:9:
        time not implemented on this platform
        
        Stack:
        
        Error
            at http://127.0.0.1:8000/wasm-bindgen-test:607:21
            at logError (http://127.0.0.1:8000/wasm-bindgen-test:227:18)
            at imports.wbg.__wbg_new_abda76e883ba8a5f (http://127.0.0.1:8000/wasm-bindgen-test:606:66)
            at web-2c96fb9c4b22de72.wasm.console_error_panic_hook::Error::new::h633542b1838b6d43 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1069]:0x7850f)
            at web-2c96fb9c4b22de72.wasm.console_error_panic_hook::hook_impl::hd480bb66b175409d (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[339]:0x54cf3)
            at web-2c96fb9c4b22de72.wasm.console_error_panic_hook::hook::h1a1eb462957a2a8b (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1485]:0x7f20c)
            at web-2c96fb9c4b22de72.wasm.core::ops::function::Fn::call::hc95eafbab36544b1 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1266]:0x7bffd)
            at web-2c96fb9c4b22de72.wasm.std::panicking::rust_panic_with_hook::h33fe77d38d305ca3 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[604]:0x67bfe)
            at web-2c96fb9c4b22de72.wasm.std::panicking::begin_panic_handler::{{closure}}::h98de848d678bad07 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[803]:0x70c7c)
            at web-2c96fb9c4b22de72.wasm.std::sys::backtrace::__rust_end_short_backtrace::h2bcfc60c3cf0a312 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1813]:0x81dda)
        
        
    
    JS exception that was thrown:
        RuntimeError: unreachable
            at web-2c96fb9c4b22de72.wasm.__rust_start_panic (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1832]:0x81e6a)
            at web-2c96fb9c4b22de72.wasm.rust_panic (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1762]:0x81b54)
            at web-2c96fb9c4b22de72.wasm.std::panicking::rust_panic_with_hook::h33fe77d38d305ca3 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[604]:0x67c29)
            at web-2c96fb9c4b22de72.wasm.std::panicking::begin_panic_handler::{{closure}}::h98de848d678bad07 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[803]:0x70c7c)
            at web-2c96fb9c4b22de72.wasm.std::sys::backtrace::__rust_end_short_backtrace::h2bcfc60c3cf0a312 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1813]:0x81dda)
            at web-2c96fb9c4b22de72.wasm.rust_begin_unwind (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1131]:0x79974)
            at web-2c96fb9c4b22de72.wasm.core::panicking::panic_fmt::hde8b7aa66e2831e1 (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1184]:0x7a978)
            at web-2c96fb9c4b22de72.wasm.std::time::SystemTime::now::hbc98278d839eb2dc (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[1534]:0x7fc55)
            at web-2c96fb9c4b22de72.wasm.vfs::impls::memory::MemoryFsImpl::new::h0840208a7e4e716a (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[179]:0x3f196)
            at web-2c96fb9c4b22de72.wasm.vfs::impls::memory::MemoryFS::new::h88375684b5be455b (http://127.0.0.1:8000/wasm-bindgen-test_bg.wasm:wasm-function[867]:0x72fe9)

failures:

    web::pass

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 filtered out

```