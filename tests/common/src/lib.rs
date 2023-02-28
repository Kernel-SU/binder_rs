pub mod IRemoteService;

use binder::{Interface, Result as BinderResult};

pub struct MyService;

impl Interface for MyService {}

impl IRemoteService::IRemoteService for MyService {
    fn getPid(&self) -> BinderResult<i32> {
        println!("getPid -> 42");
        Ok(42)
    }

    fn basicTypes(&self, a: i32, b: i64, c: bool, d: f32, e: f64, f: &str) -> BinderResult<()> {
        println!("basicTypes called!");
        println!("{} {} {} {} {} {}", a, b, c, d, e, f);
        Ok(())
    }
}