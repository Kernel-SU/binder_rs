use binder::Strong;
use crate::IRemoteService::{IRemoteService};

pub fn run() -> anyhow::Result<()> {
    let my_service: Strong<dyn IRemoteService> = binder::get_interface("myservice").unwrap();
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
    Ok(())
}
