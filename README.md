# binder_rs

The stub `libbinder_ndk.so` in ndk's sysroot hides some apis, so that we cannot link our `libbiner_rs` to it. However, we can rebuild a stub so to make linker happy. The stub source is generated from symbols.txt, whose contents are extracted from prebuilt `libbinder_ndk.so` from aosp-mainline.


The libbinder_ndk.so is introduced in api-29, you need to compile it with specified target api level. The compilation won't failed if api is lower than 29, because we build the dynamic library on ourselves.


We use ndk-build to build the stub so, `ANDROID_NDK_HOME` must be set in your env !


> `sys/src/include_*` from [platform/frameworks/native/libs/binder/ndk](https://android.googlesource.com/platform/frameworks/native/+/refs/heads/master/libs/binder/ndk/)


# Test

## My Demo Service

1. Build

```
cd tests/
cargo ndk -t arm64-v8a --platform=29 build
```

2. Push file to phone

```
adb push target/aarch64-linux-android/release/binder-tests /data/local/tmp
```

3. Run 2 adb shell

Run server:

```
adb shell
su -c /data/local/tmp/binder-tests server
```

Run client:

```
adb shell
su -c /data/local/tmp/binder-tests client
```

## Integration Test

1. Build

```
cd tests/
cargo ndk -t arm64-v8a --platform=29 build
```

2. Push files to phone

```
adb push target/aarch64-linux-android/release/integration /data/local/tmp
```

3. Build unittest binary

```
cargo ndk -t arm64-v8a --platform=29 --bindgen test --bin integration
```

Error occors, but it's expected, cause we have to run it in the phone!
```
Caused by:
  process didn't exit successfully: `/Users/ylarod/Github/binder_rs/tests/target/aarch64-linux-android/debug/deps/integration-cebf3b1eff4b3100` (exit status: 126)
```

4. Push files to phone

```
adb push /Users/ylarod/Github/binder_rs/tests/target/aarch64-linux-android/debug/deps/integration-cebf3b1eff4b3100 /data/local/tmp
```

5. Run test

```
adb shell
cd /data/local/tmp/ && su -c ./integration-cebf3b1eff4b3100
```

if you see error like this

```
CANNOT LINK EXECUTABLE "/data/local/tmp/integration-cebf3b1eff4b3100": cannot locate symbol "AIBinder_DeathRecipient_setOnUnlinked" referenced by "/data/local/tmp/integration-cebf3b1eff4b3100"...
```

This is because your android version is too low, the source is from android-mainline. You can try in avd, with Android U