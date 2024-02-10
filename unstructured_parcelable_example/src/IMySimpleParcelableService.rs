/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/my_simple_parcelable_service/aidl/com.example.mysimpleparcelableservice-rust-source/gen/com/example/mysimpleparcelableservice/IMySimpleParcelableService.rs.d -o out/soong/.intermediates/external/rust/my_simple_parcelable_service/aidl/com.example.mysimpleparcelableservice-rust-source/gen -Nexternal/rust/my_simple_parcelable_service/aidl external/rust/my_simple_parcelable_service/aidl/com/example/mysimpleparcelableservice/IMySimpleParcelableService.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
use crate::my_simple_parcelable;
declare_binder_interface! {
  IMySimpleParcelableService["com.example.mysimpleparcelableservice.IMySimpleParcelableService"] {
    native: BnMySimpleParcelableService(on_transact),
    proxy: BpMySimpleParcelableService {
    },
    async: IMySimpleParcelableServiceAsync,
  }
}
pub trait IMySimpleParcelableService: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "com.example.mysimpleparcelableservice.IMySimpleParcelableService" }
  fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable>;
  fn getDefaultImpl() -> IMySimpleParcelableServiceDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IMySimpleParcelableServiceDefaultRef) -> IMySimpleParcelableServiceDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IMySimpleParcelableServiceAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "com.example.mysimpleparcelableservice.IMySimpleParcelableService" }
  fn r#sendMySimpleParcelable<'a>(&'a self, _arg_name: &'a str, _arg_years: i32) -> binder::BoxFuture<'a, binder::Result<my_simple_parcelable::MySimpleParcelable>>;
}
#[::async_trait::async_trait]
pub trait IMySimpleParcelableServiceAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "com.example.mysimpleparcelableservice.IMySimpleParcelableService" }
  async fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable>;
}
impl BnMySimpleParcelableService {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IMySimpleParcelableService>
  where
    T: IMySimpleParcelableServiceAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IMySimpleParcelableService for Wrapper<T, R>
    where
      T: IMySimpleParcelableServiceAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable> {
        self._rt.block_on(self._inner.r#sendMySimpleParcelable(_arg_name, _arg_years))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IMySimpleParcelableServiceDefault: Send + Sync {
  fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#sendMySimpleParcelable: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IMySimpleParcelableServiceDefaultRef = Option<std::sync::Arc<dyn IMySimpleParcelableServiceDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IMySimpleParcelableServiceDefaultRef> = std::sync::Mutex::new(None);
impl BpMySimpleParcelableService {
  fn build_parcel_sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_name)?;
    aidl_data.write(&_arg_years)?;
    Ok(aidl_data)
  }
  fn read_response_sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<my_simple_parcelable::MySimpleParcelable> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMySimpleParcelableService>::getDefaultImpl() {
        return _aidl_default_impl.r#sendMySimpleParcelable(_arg_name, _arg_years);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: my_simple_parcelable::MySimpleParcelable = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IMySimpleParcelableService for BpMySimpleParcelableService {
  fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable> {
    let _aidl_data = self.build_parcel_sendMySimpleParcelable(_arg_name, _arg_years)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#sendMySimpleParcelable, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_sendMySimpleParcelable(_arg_name, _arg_years, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IMySimpleParcelableServiceAsync<P> for BpMySimpleParcelableService {
  fn r#sendMySimpleParcelable<'a>(&'a self, _arg_name: &'a str, _arg_years: i32) -> binder::BoxFuture<'a, binder::Result<my_simple_parcelable::MySimpleParcelable>> {
    let _aidl_data = match self.build_parcel_sendMySimpleParcelable(_arg_name, _arg_years) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#sendMySimpleParcelable, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_sendMySimpleParcelable(_arg_name, _arg_years, _aidl_reply)
      }
    )
  }
}
impl IMySimpleParcelableService for binder::binder_impl::Binder<BnMySimpleParcelableService> {
  fn r#sendMySimpleParcelable(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<my_simple_parcelable::MySimpleParcelable> { self.0.r#sendMySimpleParcelable(_arg_name, _arg_years) }
}
fn on_transact(_aidl_service: &dyn IMySimpleParcelableService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#sendMySimpleParcelable => {
      let _arg_name: String = _aidl_data.read()?;
      let _arg_years: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#sendMySimpleParcelable(&_arg_name, _arg_years);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IMySimpleParcelableService as _3_com_7_example_25_mysimpleparcelableservice_26_IMySimpleParcelableService;
}
