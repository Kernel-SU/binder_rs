/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Rust Binder crate integration tests

use binder::{declare_binder_enum, declare_binder_interface};
use binder::{BinderFeatures, Interface, StatusCode, ThreadState};
// Import from internal API for testing only, do not use this module in
// production.
use binder::binder_impl::{
    Binder, BorrowedParcel, IBinderInternal, TransactionCode, FIRST_CALL_TRANSACTION,
};

use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::io::Write;
use std::sync::Mutex;

/// Name of service runner.
///
/// Must match the binary name in Android.bp
/// FYI: There is a delta here in the stand-alone crate. We name it what we called in the Cargo.toml
const RUST_SERVICE_BINARY: &str = "binder-tests";

/// Binary to run a test service.
///
/// This needs to be in a separate process from the tests, so we spawn this
/// binary as a child, providing the service name as an argument.
fn main() -> Result<(), &'static str> {
    // Ensure that we can handle all transactions on the main thread.
    binder::ProcessState::set_thread_pool_max_thread_count(0);
    binder::ProcessState::start_thread_pool();

    let mut args = std::env::args().skip(1);
    if args.len() < 1 || args.len() > 2 {
        print_usage();
        return Err("");
    }
    let service_name = args.next().ok_or_else(|| {
        print_usage();
        "Missing SERVICE_NAME argument"
    })?;
    let extension_name = args.next();

    {
        let mut service = Binder::new(BnTest(Box::new(TestService::new(&service_name))));
        service.set_requesting_sid(true);
        if let Some(extension_name) = extension_name {
            let extension =
                BnTest::new_binder(TestService::new(&extension_name), BinderFeatures::default());
            service.set_extension(&mut extension.as_binder()).expect("Could not add extension");
        }
        binder::add_service(&service_name, service.as_binder())
            .expect("Could not register service");
    }

    binder::ProcessState::join_thread_pool();
    Err("Unexpected exit after join_thread_pool")
}

fn print_usage() {
    eprintln!("Usage: {} SERVICE_NAME [EXTENSION_NAME]", RUST_SERVICE_BINARY);
    eprintln!(concat!(
        "Spawn a Binder test service identified by SERVICE_NAME,",
        " optionally with an extesion named EXTENSION_NAME",
    ));
}

struct TestService {
    s: String,
    dump_args: Mutex<Vec<String>>,
}

impl TestService {
    fn new(s: &str) -> Self {
        Self { s: s.to_string(), dump_args: Mutex::new(Vec::new()) }
    }
}

#[repr(u32)]
enum TestTransactionCode {
    Test = FIRST_CALL_TRANSACTION,
    GetDumpArgs,
    GetSelinuxContext,
    GetIsHandlingTransaction,
}

impl TryFrom<u32> for TestTransactionCode {
    type Error = StatusCode;

    fn try_from(c: u32) -> Result<Self, Self::Error> {
        match c {
            _ if c == TestTransactionCode::Test as u32 => Ok(TestTransactionCode::Test),
            _ if c == TestTransactionCode::GetDumpArgs as u32 => {
                Ok(TestTransactionCode::GetDumpArgs)
            }
            _ if c == TestTransactionCode::GetSelinuxContext as u32 => {
                Ok(TestTransactionCode::GetSelinuxContext)
            }
            _ if c == TestTransactionCode::GetIsHandlingTransaction as u32 => {
                Ok(TestTransactionCode::GetIsHandlingTransaction)
            }
            _ => Err(StatusCode::UNKNOWN_TRANSACTION),
        }
    }
}

impl Interface for TestService {
    fn dump(&self, _writer: &mut dyn Write, args: &[&CStr]) -> Result<(), StatusCode> {
        let mut dump_args = self.dump_args.lock().unwrap();
        dump_args.extend(args.iter().map(|s| s.to_str().unwrap().to_owned()));
        Ok(())
    }
}

impl ITest for TestService {
    fn test(&self) -> Result<String, StatusCode> {
        Ok(self.s.clone())
    }

    fn get_dump_args(&self) -> Result<Vec<String>, StatusCode> {
        let args = self.dump_args.lock().unwrap().clone();
        Ok(args)
    }

    fn get_selinux_context(&self) -> Result<String, StatusCode> {
        let sid =
            ThreadState::with_calling_sid(|sid| sid.map(|s| s.to_string_lossy().into_owned()));
        sid.ok_or(StatusCode::UNEXPECTED_NULL)
    }

    fn get_is_handling_transaction(&self) -> Result<bool, StatusCode> {
        Ok(binder::is_handling_transaction())
    }
}

/// Trivial testing binder interface
pub trait ITest: Interface {
    /// Returns a test string
    fn test(&self) -> Result<String, StatusCode>;

    /// Return the arguments sent via dump
    fn get_dump_args(&self) -> Result<Vec<String>, StatusCode>;

    /// Returns the caller's SELinux context
    fn get_selinux_context(&self) -> Result<String, StatusCode>;

    /// Returns the value of calling `is_handling_transaction`.
    fn get_is_handling_transaction(&self) -> Result<bool, StatusCode>;
}

/// Async trivial testing binder interface
pub trait IATest<P>: Interface {
    /// Returns a test string
    fn test(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>>;

    /// Return the arguments sent via dump
    fn get_dump_args(&self) -> binder::BoxFuture<'static, Result<Vec<String>, StatusCode>>;

    /// Returns the caller's SELinux context
    fn get_selinux_context(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>>;

    /// Returns the value of calling `is_handling_transaction`.
    fn get_is_handling_transaction(&self) -> binder::BoxFuture<'static, Result<bool, StatusCode>>;
}

declare_binder_interface! {
    ITest["android.os.ITest"] {
        native: BnTest(on_transact),
        proxy: BpTest {
            x: i32 = 100
        },
        async: IATest,
    }
}

fn on_transact(
    service: &dyn ITest,
    code: TransactionCode,
    _data: &BorrowedParcel<'_>,
    reply: &mut BorrowedParcel<'_>,
) -> Result<(), StatusCode> {
    match code.try_into()? {
        TestTransactionCode::Test => reply.write(&service.test()?),
        TestTransactionCode::GetDumpArgs => reply.write(&service.get_dump_args()?),
        TestTransactionCode::GetSelinuxContext => reply.write(&service.get_selinux_context()?),
        TestTransactionCode::GetIsHandlingTransaction => {
            reply.write(&service.get_is_handling_transaction()?)
        }
    }
}

impl ITest for BpTest {
    fn test(&self) -> Result<String, StatusCode> {
        let reply =
            self.binder.transact(TestTransactionCode::Test as TransactionCode, 0, |_| Ok(()))?;
        reply.read()
    }

    fn get_dump_args(&self) -> Result<Vec<String>, StatusCode> {
        let reply =
            self.binder
                .transact(TestTransactionCode::GetDumpArgs as TransactionCode, 0, |_| Ok(()))?;
        reply.read()
    }

    fn get_selinux_context(&self) -> Result<String, StatusCode> {
        let reply = self.binder.transact(
            TestTransactionCode::GetSelinuxContext as TransactionCode,
            0,
            |_| Ok(()),
        )?;
        reply.read()
    }

    fn get_is_handling_transaction(&self) -> Result<bool, StatusCode> {
        let reply = self.binder.transact(
            TestTransactionCode::GetIsHandlingTransaction as TransactionCode,
            0,
            |_| Ok(()),
        )?;
        reply.read()
    }
}

impl<P: binder::BinderAsyncPool> IATest<P> for BpTest {
    fn test(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>> {
        let binder = self.binder.clone();
        P::spawn(
            move || binder.transact(TestTransactionCode::Test as TransactionCode, 0, |_| Ok(())),
            |reply| async move { reply?.read() },
        )
    }

    fn get_dump_args(&self) -> binder::BoxFuture<'static, Result<Vec<String>, StatusCode>> {
        let binder = self.binder.clone();
        P::spawn(
            move || {
                binder.transact(TestTransactionCode::GetDumpArgs as TransactionCode, 0, |_| Ok(()))
            },
            |reply| async move { reply?.read() },
        )
    }

    fn get_selinux_context(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>> {
        let binder = self.binder.clone();
        P::spawn(
            move || {
                binder.transact(
                    TestTransactionCode::GetSelinuxContext as TransactionCode,
                    0,
                    |_| Ok(()),
                )
            },
            |reply| async move { reply?.read() },
        )
    }

    fn get_is_handling_transaction(&self) -> binder::BoxFuture<'static, Result<bool, StatusCode>> {
        let binder = self.binder.clone();
        P::spawn(
            move || {
                binder.transact(
                    TestTransactionCode::GetIsHandlingTransaction as TransactionCode,
                    0,
                    |_| Ok(()),
                )
            },
            |reply| async move { reply?.read() },
        )
    }
}

impl ITest for Binder<BnTest> {
    fn test(&self) -> Result<String, StatusCode> {
        self.0.test()
    }

    fn get_dump_args(&self) -> Result<Vec<String>, StatusCode> {
        self.0.get_dump_args()
    }

    fn get_selinux_context(&self) -> Result<String, StatusCode> {
        self.0.get_selinux_context()
    }

    fn get_is_handling_transaction(&self) -> Result<bool, StatusCode> {
        self.0.get_is_handling_transaction()
    }
}

impl<P: binder::BinderAsyncPool> IATest<P> for Binder<BnTest> {
    fn test(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>> {
        let res = self.0.test();
        Box::pin(async move { res })
    }

    fn get_dump_args(&self) -> binder::BoxFuture<'static, Result<Vec<String>, StatusCode>> {
        let res = self.0.get_dump_args();
        Box::pin(async move { res })
    }

    fn get_selinux_context(&self) -> binder::BoxFuture<'static, Result<String, StatusCode>> {
        let res = self.0.get_selinux_context();
        Box::pin(async move { res })
    }

    fn get_is_handling_transaction(&self) -> binder::BoxFuture<'static, Result<bool, StatusCode>> {
        let res = self.0.get_is_handling_transaction();
        Box::pin(async move { res })
    }
}

/// Trivial testing binder interface
pub trait ITestSameDescriptor: Interface {}

declare_binder_interface! {
    ITestSameDescriptor["android.os.ITest"] {
        native: BnTestSameDescriptor(on_transact_same_descriptor),
        proxy: BpTestSameDescriptor,
    }
}

fn on_transact_same_descriptor(
    _service: &dyn ITestSameDescriptor,
    _code: TransactionCode,
    _data: &BorrowedParcel<'_>,
    _reply: &mut BorrowedParcel<'_>,
) -> Result<(), StatusCode> {
    Ok(())
}

impl ITestSameDescriptor for BpTestSameDescriptor {}

impl ITestSameDescriptor for Binder<BnTestSameDescriptor> {}

declare_binder_enum! {
    TestEnum : [i32; 3] {
        FOO = 1,
        BAR = 2,
        BAZ = 3,
    }
}

declare_binder_enum! {
    #[deprecated(since = "1.0.0")]
    TestDeprecatedEnum : [i32; 3] {
        FOO = 1,
        BAR = 2,
        BAZ = 3,
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use std::fs::File;
    use std::process::{Child, Command};
    use std::ptr;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use binder::{
        BinderFeatures, DeathRecipient, FromIBinder, IBinder, Interface, SpIBinder, StatusCode,
        Strong,
    };
    // Import from impl API for testing only, should not be necessary as long as
    // you are using AIDL.
    use binder::binder_impl::{Binder, IBinderInternal, TransactionCode};

    use binder_tokio::Tokio;

    use super::{BnTest, IATest, ITest, ITestSameDescriptor, TestService, RUST_SERVICE_BINARY};

    pub struct ScopedServiceProcess(Child);

    impl ScopedServiceProcess {
        pub fn new(identifier: &str) -> Self {
            Self::new_internal(identifier, None)
        }

        pub fn new_with_extension(identifier: &str, extension: &str) -> Self {
            Self::new_internal(identifier, Some(extension))
        }

        fn new_internal(identifier: &str, extension: Option<&str>) -> Self {
            let mut binary_path =
                std::env::current_exe().expect("Could not retrieve current executable path");
            binary_path.pop();
            binary_path.push(RUST_SERVICE_BINARY);
            let mut command = Command::new(&binary_path);
            command.arg(identifier);
            if let Some(ext) = extension {
                command.arg(ext);
            }
            println!("Starting service: {:?}", command);
            let child = command.spawn().expect("Could not start service");
            Self(child)
        }
    }

    impl Drop for ScopedServiceProcess {
        fn drop(&mut self) {
            self.0.kill().expect("Could not kill child process");
            self.0.wait().expect("Could not wait for child process to die");
        }
    }

    #[test]
    fn check_services() {
        let mut sm = binder::get_service("manager").expect("Did not get manager binder service");
        assert!(sm.is_binder_alive());
        assert!(sm.ping_binder().is_ok());

        assert!(binder::get_service("this_service_does_not_exist").is_none());
        assert_eq!(
            binder::get_interface::<dyn ITest>("this_service_does_not_exist").err(),
            Some(StatusCode::NAME_NOT_FOUND)
        );
        assert_eq!(
            binder::get_interface::<dyn IATest<Tokio>>("this_service_does_not_exist").err(),
            Some(StatusCode::NAME_NOT_FOUND)
        );

        // The service manager service isn't an ITest, so this must fail.
        assert_eq!(binder::get_interface::<dyn ITest>("manager").err(), Some(StatusCode::BAD_TYPE));
        assert_eq!(
            binder::get_interface::<dyn IATest<Tokio>>("manager").err(),
            Some(StatusCode::BAD_TYPE)
        );
    }

    #[tokio::test]
    async fn check_services_async() {
        let mut sm = binder::get_service("manager").expect("Did not get manager binder service");
        assert!(sm.is_binder_alive());
        assert!(sm.ping_binder().is_ok());

        assert!(binder::get_service("this_service_does_not_exist").is_none());
        assert_eq!(
            binder_tokio::get_interface::<dyn ITest>("this_service_does_not_exist").await.err(),
            Some(StatusCode::NAME_NOT_FOUND)
        );
        assert_eq!(
            binder_tokio::get_interface::<dyn IATest<Tokio>>("this_service_does_not_exist")
                .await
                .err(),
            Some(StatusCode::NAME_NOT_FOUND)
        );

        // The service manager service isn't an ITest, so this must fail.
        assert_eq!(
            binder_tokio::get_interface::<dyn ITest>("manager").await.err(),
            Some(StatusCode::BAD_TYPE)
        );
        assert_eq!(
            binder_tokio::get_interface::<dyn IATest<Tokio>>("manager").await.err(),
            Some(StatusCode::BAD_TYPE)
        );
    }

    #[test]
    fn check_wait_for_service() {
        let mut sm =
            binder::wait_for_service("manager").expect("Did not get manager binder service");
        assert!(sm.is_binder_alive());
        assert!(sm.ping_binder().is_ok());

        // The service manager service isn't an ITest, so this must fail.
        assert_eq!(
            binder::wait_for_interface::<dyn ITest>("manager").err(),
            Some(StatusCode::BAD_TYPE)
        );
        assert_eq!(
            binder::wait_for_interface::<dyn IATest<Tokio>>("manager").err(),
            Some(StatusCode::BAD_TYPE)
        );
    }

    #[test]
    fn get_declared_instances() {
        // At the time of writing this test, there is no good VINTF interface
        // guaranteed to be on all devices. Cuttlefish has light, so this will
        // generally test things.
        let has_lights = binder::is_declared("android.hardware.light.ILights/default")
            .expect("Could not check for declared interface");

        let instances = binder::get_declared_instances("android.hardware.light.ILights")
            .expect("Could not get declared instances");

        let expected_defaults = usize::from(has_lights);
        assert_eq!(expected_defaults, instances.iter().filter(|i| i.as_str() == "default").count());
    }

    #[test]
    fn trivial_client() {
        let service_name = "trivial_client_test";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn ITest> =
            binder::get_interface(service_name).expect("Did not get manager binder service");
        assert_eq!(test_client.test().unwrap(), "trivial_client_test");
    }

    #[tokio::test]
    async fn trivial_client_async() {
        let service_name = "trivial_client_test";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn IATest<Tokio>> = binder_tokio::get_interface(service_name)
            .await
            .expect("Did not get manager binder service");
        assert_eq!(test_client.test().await.unwrap(), "trivial_client_test");
    }

    #[test]
    fn wait_for_trivial_client() {
        let service_name = "wait_for_trivial_client_test";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn ITest> =
            binder::wait_for_interface(service_name).expect("Did not get manager binder service");
        assert_eq!(test_client.test().unwrap(), "wait_for_trivial_client_test");
    }

    #[tokio::test]
    async fn wait_for_trivial_client_async() {
        let service_name = "wait_for_trivial_client_test";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn IATest<Tokio>> = binder_tokio::wait_for_interface(service_name)
            .await
            .expect("Did not get manager binder service");
        assert_eq!(test_client.test().await.unwrap(), "wait_for_trivial_client_test");
    }

    fn get_expected_selinux_context() -> &'static str {
        // SAFETY: The pointer we pass to `getcon` is valid because it comes from a reference, and
        // `getcon` doesn't retain it after it returns. If `getcon` succeeds then `out_ptr` will
        // point to a valid C string, otherwise it will remain null. We check for null, so the
        // pointer we pass to `CStr::from_ptr` must be a valid pointer to a C string. There is a
        // memory leak as we don't call `freecon`, but that's fine because this is just a test.
        // unsafe {
        //     let mut out_ptr = ptr::null_mut();
        //     assert_eq!(selinux_sys::getcon(&mut out_ptr), 0);
        //     assert!(!out_ptr.is_null());
        //     CStr::from_ptr(out_ptr).to_str().expect("context was invalid UTF-8")
        // }
        "u:r:su:s0"
    }

    #[test]
    fn get_selinux_context() {
        let service_name = "get_selinux_context";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn ITest> =
            binder::get_interface(service_name).expect("Did not get manager binder service");
        assert_eq!(test_client.get_selinux_context().unwrap(), get_expected_selinux_context());
    }

    #[tokio::test]
    async fn get_selinux_context_async() {
        let service_name = "get_selinux_context_async";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn IATest<Tokio>> = binder_tokio::get_interface(service_name)
            .await
            .expect("Did not get manager binder service");
        assert_eq!(
            test_client.get_selinux_context().await.unwrap(),
            get_expected_selinux_context()
        );
    }

    #[tokio::test]
    async fn get_selinux_context_sync_to_async() {
        let service_name = "get_selinux_context";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn ITest> =
            binder::get_interface(service_name).expect("Did not get manager binder service");
        let test_client = test_client.into_async::<Tokio>();
        assert_eq!(
            test_client.get_selinux_context().await.unwrap(),
            get_expected_selinux_context()
        );
    }

    #[tokio::test]
    async fn get_selinux_context_async_to_sync() {
        let service_name = "get_selinux_context";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn IATest<Tokio>> = binder_tokio::get_interface(service_name)
            .await
            .expect("Did not get manager binder service");
        let test_client = test_client.into_sync();
        assert_eq!(test_client.get_selinux_context().unwrap(), get_expected_selinux_context());
    }

    struct Bools {
        binder_died: Arc<AtomicBool>,
        binder_dealloc: Arc<AtomicBool>,
    }

    impl Bools {
        fn is_dead(&self) -> bool {
            self.binder_died.load(Ordering::Relaxed)
        }
        fn assert_died(&self) {
            assert!(self.is_dead(), "Did not receive death notification");
        }
        fn assert_dropped(&self) {
            assert!(
                self.binder_dealloc.load(Ordering::Relaxed),
                "Did not dealloc death notification"
            );
        }
        fn assert_not_dropped(&self) {
            assert!(
                !self.binder_dealloc.load(Ordering::Relaxed),
                "Dealloc death notification too early"
            );
        }
    }

    fn register_death_notification(binder: &mut SpIBinder) -> (Bools, DeathRecipient) {
        let binder_died = Arc::new(AtomicBool::new(false));
        let binder_dealloc = Arc::new(AtomicBool::new(false));

        struct SetOnDrop {
            binder_dealloc: Arc<AtomicBool>,
        }
        impl Drop for SetOnDrop {
            fn drop(&mut self) {
                self.binder_dealloc.store(true, Ordering::Relaxed);
            }
        }

        let mut death_recipient = {
            let flag = binder_died.clone();
            let set_on_drop = SetOnDrop { binder_dealloc: binder_dealloc.clone() };
            DeathRecipient::new(move || {
                flag.store(true, Ordering::Relaxed);
                // Force the closure to take ownership of set_on_drop. When the closure is
                // dropped, the destructor of `set_on_drop` will run.
                let _ = &set_on_drop;
            })
        };

        binder.link_to_death(&mut death_recipient).expect("link_to_death failed");

        let bools = Bools { binder_died, binder_dealloc };

        (bools, death_recipient)
    }

    /// Killing a remote service should unregister the service and trigger
    /// death notifications.
    #[test]
    fn test_death_notifications() {
        binder::ProcessState::start_thread_pool();

        let service_name = "test_death_notifications";
        let service_process = ScopedServiceProcess::new(service_name);
        let mut remote = binder::get_service(service_name).expect("Could not retrieve service");

        let (bools, recipient) = register_death_notification(&mut remote);

        drop(service_process);
        remote.ping_binder().expect_err("Service should have died already");

        // Pause to ensure any death notifications get delivered
        thread::sleep(Duration::from_secs(1));

        bools.assert_died();
        bools.assert_not_dropped();

        drop(recipient);

        bools.assert_dropped();
    }

    /// Test unregistering death notifications.
    #[test]
    fn test_unregister_death_notifications() {
        binder::ProcessState::start_thread_pool();

        let service_name = "test_unregister_death_notifications";
        let service_process = ScopedServiceProcess::new(service_name);
        let mut remote = binder::get_service(service_name).expect("Could not retrieve service");

        let (bools, mut recipient) = register_death_notification(&mut remote);

        remote.unlink_to_death(&mut recipient).expect("Could not unlink death notifications");

        drop(service_process);
        remote.ping_binder().expect_err("Service should have died already");

        // Pause to ensure any death notifications get delivered
        thread::sleep(Duration::from_secs(1));

        assert!(!bools.is_dead(), "Received unexpected death notification after unlinking",);

        bools.assert_not_dropped();
        drop(recipient);
        bools.assert_dropped();
    }

    /// Dropping a remote handle should unregister any death notifications.
    #[test]
    fn test_death_notification_registration_lifetime() {
        binder::ProcessState::start_thread_pool();

        let service_name = "test_death_notification_registration_lifetime";
        let service_process = ScopedServiceProcess::new(service_name);
        let mut remote = binder::get_service(service_name).expect("Could not retrieve service");

        let (bools, recipient) = register_death_notification(&mut remote);

        // This should automatically unregister our death notification.
        drop(remote);

        drop(service_process);

        // Pause to ensure any death notifications get delivered
        thread::sleep(Duration::from_secs(1));

        // We dropped the remote handle, so we should not receive the death
        // notification when the remote process dies here.
        assert!(
            !bools.is_dead(),
            "Received unexpected death notification after dropping remote handle"
        );

        bools.assert_not_dropped();
        drop(recipient);
        bools.assert_dropped();
    }

    /// Test IBinder interface methods not exercised elsewhere.
    #[test]
    fn test_misc_ibinder() {
        let service_name = "rust_test_ibinder";

        {
            let _process = ScopedServiceProcess::new(service_name);

            let test_client: Strong<dyn ITest> =
                binder::get_interface(service_name).expect("Did not get test binder service");
            let mut remote = test_client.as_binder();
            assert!(remote.is_binder_alive());
            remote.ping_binder().expect("Could not ping remote service");

            let dump_args = ["dump", "args", "for", "testing"];

            let null_out = File::open("/dev/null").expect("Could not open /dev/null");
            remote.dump(&null_out, &dump_args).expect("Could not dump remote service");

            let remote_args = test_client.get_dump_args().expect("Could not fetched dumped args");
            assert_eq!(dump_args, remote_args[..], "Remote args don't match call to dump");
        }

        // get/set_extensions is tested in test_extensions()

        // transact is tested everywhere else, and we can't make raw
        // transactions outside the [FIRST_CALL_TRANSACTION,
        // LAST_CALL_TRANSACTION] range from the NDK anyway.

        // link_to_death is tested in test_*_death_notification* tests.
    }

    #[test]
    fn test_extensions() {
        let service_name = "rust_test_extensions";
        let extension_name = "rust_test_extensions_ext";

        {
            let _process = ScopedServiceProcess::new(service_name);

            let mut remote = binder::get_service(service_name);
            assert!(remote.is_binder_alive());

            let extension = remote.get_extension().expect("Could not check for an extension");
            assert!(extension.is_none());
        }

        {
            let _process = ScopedServiceProcess::new_with_extension(service_name, extension_name);

            let mut remote = binder::get_service(service_name);
            assert!(remote.is_binder_alive());

            let maybe_extension = remote.get_extension().expect("Could not check for an extension");

            let extension = maybe_extension.expect("Remote binder did not have an extension");

            let extension: Strong<dyn ITest> = FromIBinder::try_from(extension)
                .expect("Extension could not be converted to the expected interface");

            assert_eq!(extension.test().unwrap(), extension_name);
        }
    }

    /// Test re-associating a local binder object with a different class.
    ///
    /// This is needed because different binder service (e.g. NDK vs Rust)
    /// implementations are incompatible and must not be interchanged. A local
    /// service with the same descriptor string but a different class pointer
    /// may have been created by an NDK service and is therefore incompatible
    /// with the Rust service implementation. It must be treated as remote and
    /// all API calls parceled and sent through transactions.
    ///
    /// Further tests of this behavior with the C NDK and Rust API are in
    /// rust_ndk_interop.rs
    #[test]
    fn associate_existing_class() {
        let service = Binder::new(BnTest(Box::new(TestService::new("testing_service"))));

        // This should succeed although we will have to treat the service as
        // remote.
        let _interface: Strong<dyn ITestSameDescriptor> =
            FromIBinder::try_from(service.as_binder())
                .expect("Could not re-interpret service as the ITestSameDescriptor interface");
    }

    /// Test that we can round-trip a rust service through a generic IBinder
    #[test]
    fn reassociate_rust_binder() {
        let service_name = "testing_service";
        let service_ibinder =
            BnTest::new_binder(TestService::new(service_name), BinderFeatures::default())
                .as_binder();

        let service: Strong<dyn ITest> =
            service_ibinder.into_interface().expect("Could not reassociate the generic ibinder");

        assert_eq!(service.test().unwrap(), service_name);
    }

    #[test]
    fn weak_binder_upgrade() {
        let service_name = "testing_service";
        let service = BnTest::new_binder(TestService::new(service_name), BinderFeatures::default());

        let weak = Strong::downgrade(&service);

        let upgraded = weak.upgrade().expect("Could not upgrade weak binder");

        assert_eq!(service, upgraded);
    }

    #[test]
    fn weak_binder_upgrade_dead() {
        let service_name = "testing_service";
        let weak = {
            let service =
                BnTest::new_binder(TestService::new(service_name), BinderFeatures::default());

            Strong::downgrade(&service)
        };

        assert_eq!(weak.upgrade(), Err(StatusCode::DEAD_OBJECT));
    }

    #[test]
    fn weak_binder_clone() {
        let service_name = "testing_service";
        let service = BnTest::new_binder(TestService::new(service_name), BinderFeatures::default());

        let weak = Strong::downgrade(&service);
        let cloned = weak.clone();
        assert_eq!(weak, cloned);

        let upgraded = weak.upgrade().expect("Could not upgrade weak binder");
        let clone_upgraded = cloned.upgrade().expect("Could not upgrade weak binder");

        assert_eq!(service, upgraded);
        assert_eq!(service, clone_upgraded);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn binder_ord() {
        let service1 =
            BnTest::new_binder(TestService::new("testing_service1"), BinderFeatures::default());
        let service2 =
            BnTest::new_binder(TestService::new("testing_service2"), BinderFeatures::default());

        assert!((service1 >= service1));
        assert!((service1 <= service1));
        assert_eq!(service1 < service2, (service2 >= service1));
    }

    #[test]
    fn binder_parcel_mixup() {
        let service1 =
            BnTest::new_binder(TestService::new("testing_service1"), BinderFeatures::default());
        let service2 =
            BnTest::new_binder(TestService::new("testing_service2"), BinderFeatures::default());

        let service1 = service1.as_binder();
        let service2 = service2.as_binder();

        let parcel = service1.prepare_transact().unwrap();
        let res = service2.submit_transact(
            super::TestTransactionCode::Test as TransactionCode,
            parcel,
            0,
        );

        match res {
            Ok(_) => panic!("submit_transact should fail"),
            Err(err) => assert_eq!(err, binder::StatusCode::BAD_VALUE),
        }
    }

    #[test]
    fn get_is_handling_transaction() {
        let service_name = "get_is_handling_transaction";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn ITest> =
            binder::get_interface(service_name).expect("Did not get manager binder service");
        // Should be true externally.
        assert!(test_client.get_is_handling_transaction().unwrap());

        // Should be false locally.
        assert!(!binder::is_handling_transaction());

        // Should also be false in spawned thread.
        std::thread::spawn(|| {
            assert!(!binder::is_handling_transaction());
        })
        .join()
        .unwrap();
    }

    #[tokio::test]
    async fn get_is_handling_transaction_async() {
        let service_name = "get_is_handling_transaction_async";
        let _process = ScopedServiceProcess::new(service_name);
        let test_client: Strong<dyn IATest<Tokio>> = binder_tokio::get_interface(service_name)
            .await
            .expect("Did not get manager binder service");
        // Should be true externally.
        assert!(test_client.get_is_handling_transaction().await.unwrap());

        // Should be false locally.
        assert!(!binder::is_handling_transaction());

        // Should also be false in spawned task.
        tokio::spawn(async {
            assert!(!binder::is_handling_transaction());
        })
        .await
        .unwrap();

        // And in spawn_blocking task.
        tokio::task::spawn_blocking(|| {
            assert!(!binder::is_handling_transaction());
        })
        .await
        .unwrap();
    }
}
