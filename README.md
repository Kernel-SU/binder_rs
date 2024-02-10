# binder_rs

The stub `libbinder_ndk.so` in ndk's sysroot hides some apis, so that we cannot link our `libbiner_rs` to it. However, we can rebuild a stub so to make linker happy. The stub source is generated from symbols.txt, whose contents are extracted from prebuilt `libbinder_ndk.so` from aosp-mainline.

The libbinder_ndk.so is introduced in api-29, you need to compile it with specified target api level. 
The compilation won't fail if api is lower than 29, because we build the dynamic library ourselves.

## Adding Android targets via rustup

For each Android target you wish to build for you must e.g.

```bash
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

## Building using the `cargo-ndk` tooling

`cargo-ndk` simplifies the process of building libraries and binaries
for Android targets.

1. Install Android Studio

2. Install the NDK from within Android Studio

3. Install `cargo-ndk` with:

```bash
cargo install cargo-ndk
```

We use `cargo ndk` to build the stub so, `ANDROID_NDK_HOME` must be set in your env !

## Building your own `libbinder_ndk.so` from the AOSP

If you follow the steps below in `Generate Rust code from AIDL from the AOSP`
you will get a `libbinder_ndk.so` build 'for free' for your target architecture.

Copy that `libbinder_ndk.so` from where it's built in the aosp root to where
it should belong in the path to your NDK install.

In my case I copied it from:
```
<aosp-root>/out/target/product/generic_x86_64/system/lib64/libbinder_ndk.so
```
to
```
~/Android/Sdk/ndk/<version-of-ndk>/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/x86_64-linux-android/29/libbinder_ndk.so
```
since I was going to compile with platform API level 29 and for x86_64.

## Side note on where the sources are from

> `sys/src/include_*` from [platform/frameworks/native/libs/binder/ndk](https://android.googlesource.com/platform/frameworks/native/+/refs/heads/master/libs/binder/ndk/)

# Example

1. Build

    ```
    cargo ndk -t arm64-v8a --platform=29 --bindgen build
    ```

2. Push file to phone

    ```
    adb push target/aarch64-linux-android/debug/binder-example /data/local/tmp
    ```

3. Run 2 adb shell

    Run server:

    ```
    adb shell
    su -c /data/local/tmp/binder-example server
    ```

    Run client:

    ```
    adb shell
    su -c /data/local/tmp/binder-example client
    ```

## Test

1. Build

    ```
    cargo ndk -t arm64-v8a --platform=29 --bindgen build
    ```

2. Push files to phone

    ```
    adb push target/aarch64-linux-android/debug/binder-tests /data/local/tmp
    ```

3. Build unittest binary

    ```
    cargo ndk -t arm64-v8a --platform=29 --bindgen test --bin binder-tests --no-run
    ```

4. Push files to phone

    ```
    adb push target/aarch64-linux-android/debug/deps/binder_tests-acf830ec15b8864e /data/local/tmp
    ```

5. Run test

    ```
    adb shell
    cd /data/local/tmp/ && su -c ./binder_tests-acf830ec15b8864e
    ```

    if you see error like this

    ```
    CANNOT LINK EXECUTABLE "/data/local/tmp/binder_tests-acf830ec15b8864e": cannot locate symbol "AIBinder_DeathRecipient_setOnUnlinked" referenced by "/data/local/tmp/binder_tests-acf830ec15b8864e"...
    ```

    This is because your android version is too low, the source is from android-mainline. You can try in avd, with Android U

## Using AIDLs with UnstructuredParcelable

### Generate Rust code from AIDL from the AOSP

1. Create AIDL + any `UnstructuredParcelable` you want to use
   in your AIDL interface
   * Reference `unstructured_parcelable_example/aosp_module_to_generate_aidl`

Note the structure:
* `my_simple_parcelable_service/`
  * `Android.bp`: Soong build file
  * `aidl/`
    * `Android.bp`: Soong build file
    * `com/example/mysimpleparcelableservice`
      * `IMySimpleParcelableService.aidl`: The service interface, note
        we are including `MySimpleParcelable` as a return parameter
      * `MySimpleParcelable.aidl`: Essentially a bit of a "wrapper"
        which points to the actual definition of the
        UnstructuredParcelable in the `rust` folder, see below
      * `rust/`
        * `my_simple_parcelable.rs`: Note that we have to impl
          the `UnstructuredParcelable` trait.
  * `src/`
    * `lib.rs`: Implementation of the service. We only need to impl
        it because Soong will not let us make the AIDL directly. Note
        that this is not actually used at all.

2. Use Soong to build. You need to put the above folder under
   `<aosp-root>/external/rust/<your-aidl-lib-folder>`
   * As per normal when building out of the AOSP, you'll need to
     choose your target architecture, so do the following steps first
     from the aosp root:
     * `source build/envsetup.sh`
     * `lunch <your-target-choice>`
       * Recall you can type `lunch` by itself to get a listing
         of available targets

You may then proceed with building:

```bash
m libmysimpleparcelableservice
```

Expected output
```
============================================
PLATFORM_VERSION_CODENAME=VanillaIceCream
PLATFORM_VERSION=VanillaIceCream
PRODUCT_INCLUDE_TAGS=com.android.mainline mainline_module_prebuilt_nightly
TARGET_PRODUCT=aosp_x86_64
TARGET_BUILD_VARIANT=eng
TARGET_ARCH=x86_64
TARGET_ARCH_VARIANT=x86_64
TARGET_2ND_ARCH=x86
TARGET_2ND_ARCH_VARIANT=x86_64
HOST_OS=linux
HOST_OS_EXTRA=Linux-5.15.0-79-generic-x86_64-Ubuntu-20.04.6-LTS
HOST_CROSS_OS=windows
BUILD_ID=MAIN
OUT_DIR=out
============================================
[ 22% 8/36 1m15s remaining] AIDL Rust external/rust/my_simple_parcelable_service/aidl/com/example/mysimp
Compiling IDL...
GenerateRust: out/soong/.intermediates/external/rust/my_simple_parcelable_service/aidl/com.example.mysim
pleparcelableservice-rust-source/gen/com/example/mysimpleparcelableservice/IMySimpleParcelableService.rs
[100% 36/36 1m14s remaining] Install: out/target/product/generic_x86_64/system/lib64/libmysimpleparcelab

#### build completed successfully (02:19 (mm:ss)) ####
```

3. Navigate to Soong intermediate build folder and scoop up 
   generated Rust code.
   * In this case it was located at:
     * `<aosp-root>/out/soong/.intermediates/external/rust/my_simple_parcelable_service/aidl/com.example.mysimpleparcelableservice-rust-source/gen/com/example/mysimpleparcelableservice/IMySimpleParcelableService.rs`

4. Assemble the pieces in your Rust project
   * Copy in your generated Rust source for the interface from step 3
   * Copy in the Rust source of your `UnstructuredParcelable`s from step 1

5. Build

```bash
cargo ndk -t x86_64 --platform=29 --bindgen build
```

6. Remaining steps to copy / run example same as above