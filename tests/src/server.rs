use anyhow::Ok;
use binder::BinderFeatures;
use common::{IRemoteService::BnRemoteService, MyService};

pub fn run() -> anyhow::Result<()> {
    let my_service = MyService;
    let my_service_binder = BnRemoteService::new_binder(my_service, BinderFeatures::default());
    binder::add_service("myservice", my_service_binder.as_binder())
        .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    Ok(())
}
