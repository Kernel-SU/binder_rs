use binder::binder_impl::{Proxy, IBinderInternal};
use common::IRemoteService::{BpRemoteService, IRemoteService};

pub fn run() -> anyhow::Result<()> {
    let my_service_binder = binder::get_service("myservice").expect("Failed to get service");
    println!("my_service_binder alive = {}", my_service_binder.is_binder_alive());
    match BpRemoteService::from_binder(my_service_binder) {
        Ok(my_service) => {
            println!("Do getPid()");
            let pid = my_service.getPid().expect("Failed to get pid");
            println!("Got pid: {}", pid);
            println!("Do basicTypes()");
            my_service
                .basicTypes(
                    1 as i32, 2 as i64, false, 1.1 as f32, 2.2 as f64, "fuckyou!",
                )
                .expect("Failed to call basicTypes");
            println!("Done!");
        }
        Err(e) => {
            println!("Failed to get service: {}", e);
        }
    }

    Ok(())
}
