use my_simple_parcelable::MySimpleParcelable;

/// Implementation of the `IMySimpleParcelableService` AIDL interface.
use com_example_mysimpleparcelableservice::aidl::com::example::mysimpleparcelableservice::IMySimpleParcelableService::IMySimpleParcelableService;
use com_example_mysimpleparcelableservice::binder;

/// The `IMySimpleParcelableService` implementation.
pub struct MySimpleParcelableService;

impl binder::Interface for MySimpleParcelableService {}

impl IMySimpleParcelableService for MySimpleParcelableService {
    fn sendMySimpleParcelable(&self, name: &str, years: i32) -> binder::Result<MySimpleParcelable> {
        Ok(MySimpleParcelable {
            name: name.to_string(),
            number: years
        })
    }
}
