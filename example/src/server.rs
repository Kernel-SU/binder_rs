use binder::BinderFeatures;
use crate::IRemoteService::{BnRemoteService, IRemoteService};
use binder::{Interface, Result as BinderResult};

pub struct MyService;

impl Interface for MyService {}

impl IRemoteService for MyService {
    fn getPid(&self) -> BinderResult<i32> {
        println!("getPid -> 42");
        Ok(42)
    }

    fn basicTypes(&self, a: i32, b: i64, c: bool, d: f32, e: f64, f: &str) -> BinderResult<()> {
        println!("basicTypes -> {} {} {} {} {} {}", a, b, c, d, e, f);
        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let my_service = MyService;
    let my_service_binder = BnRemoteService::new_binder(my_service, BinderFeatures::default());
    binder::add_service("myservice", my_service_binder.as_binder())
        .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
