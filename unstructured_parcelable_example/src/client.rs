use binder::Strong;
use crate::IMySimpleParcelableService::{IMySimpleParcelableService};
use crate::my_simple_parcelable::MySimpleParcelable;

pub fn run() -> anyhow::Result<()> {
    let my_service: Strong<dyn IMySimpleParcelableService> = binder::get_interface("myservice").unwrap();
    println!("Do sendMySimpleParcelable()");
    let name = "Franklin";
    let my_simplest_parcelable = my_service.sendMySimpleParcelable(name, 99 as i32).expect("Failed to retrieve simple parcelable");
    println!("Got simple parcelable: {:?}", my_simplest_parcelable);
    println!("Done!");
    Ok(())
}
