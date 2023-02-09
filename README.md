# binder_rs

The stub `libbinder_ndk.so` in ndk's sysroot hides some apis, so that we cannot link our libbiner_rs to it. However, we can rebuild a stub so to make linker happy. The stub source is generated from symbols.txt, whose contents are extracted from prebuilt `libbinder_ndk.so` from aosp-mainline.

The libbinder_ndk.so is introduced in api-29, you need to compile it with specified target api level. The compilation won't failed if api is lower than 29, because we build the dynamic library on ourselves.

We use ndk-build to build the stub so, `ANDROID_NDK_HOME` must be set in your env !


> `sys/src/include_*` from [platform/frameworks/native/libs/binder/ndk](https://android.googlesource.com/platform/frameworks/native/+/refs/heads/master/libs/binder/ndk/)


# Test

```
cd tests/
cargo ndk -t arm64-v8a --platform=29 build
```