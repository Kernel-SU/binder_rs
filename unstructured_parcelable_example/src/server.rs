use binder::BinderFeatures;
use crate::IMySimpleParcelableService::{BnMySimpleParcelableService, IMySimpleParcelableService};
use crate::my_simple_parcelable::MySimpleParcelable;
use binder::{Interface, Result as BinderResult,
             binder_impl::{BorrowedParcel, UnstructuredParcelable},
};

pub struct MyService;

impl Interface for MyService {}

impl IMySimpleParcelableService for MyService {
    fn sendMySimpleParcelable(&self, name: &str, years: i32) -> BinderResult<MySimpleParcelable> {
        println!("sending simple parcelable for name: {} and years: {}", name, years);

        let my_simplest_parcelable = MySimpleParcelable {
            name: name.to_owned(),
            number: years
        };

        Ok(my_simplest_parcelable)
    }
}

pub fn run() -> anyhow::Result<()> {
    let my_service = MyService;
    let my_service_binder = BnMySimpleParcelableService::new_binder(my_service, BinderFeatures::default());
    binder::add_service("myservice", my_service_binder.as_binder())
        .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
