#![forbid(unsafe_code)]
// NOTE: use cfg_attr(rustfmt, rustfmt_skip) instead of rustfmt::skip 
// #![rustfmt::skip]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IRemoteService["IRemoteService"] {
    native: BnRemoteService(on_transact),
    proxy: BpRemoteService {
    },
    async: IRemoteServiceAsync,
  }
}
pub trait IRemoteService: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "IRemoteService" }
  fn getPid(&self) -> binder::Result<i32>;
  fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()>;
  fn getDefaultImpl() -> IRemoteServiceDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IRemoteServiceDefaultRef) -> IRemoteServiceDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IRemoteServiceAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "IRemoteService" }
  fn getPid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn basicTypes<'a>(&'a self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IRemoteServiceAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "IRemoteService" }
  async fn getPid(&self) -> binder::Result<i32>;
  async fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()>;
}
impl BnRemoteService {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IRemoteService>
  where
    T: IRemoteServiceAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
    }
    impl<T, R> IRemoteService for Wrapper<T, R>
    where
      T: IRemoteServiceAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn getPid(&self) -> binder::Result<i32> {
        self._rt.block_on(self._inner.getPid())
      }
      fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IRemoteServiceDefault: Send + Sync {
  fn getPid(&self) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const getPid: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const basicTypes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
}
pub type IRemoteServiceDefaultRef = Option<std::sync::Arc<dyn IRemoteServiceDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IRemoteServiceDefaultRef> = std::sync::Mutex::new(None);
}
impl BpRemoteService {
  fn build_parcel_getPid(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getPid(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IRemoteService>::getDefaultImpl() {
        return _aidl_default_impl.getPid();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_anInt)?;
    aidl_data.write(&_arg_aLong)?;
    aidl_data.write(&_arg_aBoolean)?;
    aidl_data.write(&_arg_aFloat)?;
    aidl_data.write(&_arg_aDouble)?;
    aidl_data.write(_arg_aString)?;
    Ok(aidl_data)
  }
  fn read_response_basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IRemoteService>::getDefaultImpl() {
        return _aidl_default_impl.basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
}
impl IRemoteService for BpRemoteService {
  fn getPid(&self) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getPid()?;
    let _aidl_reply = self.binder.submit_transact(transactions::getPid, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getPid(_aidl_reply)
  }
  fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString)?;
    let _aidl_reply = self.binder.submit_transact(transactions::basicTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IRemoteServiceAsync<P> for BpRemoteService {
  fn getPid<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getPid() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getPid, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getPid(_aidl_reply)
      }
    )
  }
  fn basicTypes<'a>(&'a self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::basicTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString, _aidl_reply)
      }
    )
  }
}
impl IRemoteService for binder::binder_impl::Binder<BnRemoteService> {
  fn getPid(&self) -> binder::Result<i32> { self.0.getPid() }
  fn basicTypes(&self, _arg_anInt: i32, _arg_aLong: i64, _arg_aBoolean: bool, _arg_aFloat: f32, _arg_aDouble: f64, _arg_aString: &str) -> binder::Result<()> { self.0.basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, _arg_aString) }
}
fn on_transact(_aidl_service: &dyn IRemoteService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::getPid => {
      let _aidl_return = _aidl_service.getPid();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::basicTypes => {
      let _arg_anInt: i32 = _aidl_data.read()?;
      let _arg_aLong: i64 = _aidl_data.read()?;
      let _arg_aBoolean: bool = _aidl_data.read()?;
      let _arg_aFloat: f32 = _aidl_data.read()?;
      let _arg_aDouble: f64 = _aidl_data.read()?;
      let _arg_aString: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.basicTypes(_arg_anInt, _arg_aLong, _arg_aBoolean, _arg_aFloat, _arg_aDouble, &_arg_aString);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::IRemoteService as _14_IRemoteService;
}
