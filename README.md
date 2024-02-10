# binder_rs

The stub `libbinder_ndk.so` in ndk's sysroot hides some apis, so that we cannot link our `libbiner_rs` to it. However, we can rebuild a stub so to make linker happy. The stub source is generated from symbols.txt, whose contents are extracted from prebuilt `libbinder_ndk.so` from aosp-mainline.


The libbinder_ndk.so is introduced in api-29, you need to compile it with specified target api level. The compilation won't failed if api is lower than 29, because we build the dynamic library on ourselves.


We use ndk-build to build the stub so, `ANDROID_NDK_HOME` must be set in your env !


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