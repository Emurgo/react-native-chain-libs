use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jobject;

use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::*;

use crate::js_chain_libs::Value;

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_io_emurgo_chainlibs_Native_valueFromStr(
  env: JNIEnv, _: JObject, string: JString
) -> jobject {
  handle_exception_result(|| {
    let rstr = string.string(&env)?;
    let val = Value::from_str(&rstr).into_result()?;
    RPtr::new(val).jptr(&env)
  }).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_io_emurgo_chainlibs_Native_valueToStr(
  env: JNIEnv, _: JObject, ptr: JObject
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    let val = rptr.typed_ref::<Value>()?;
    val.to_str().jstring(&env)
  }).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_io_emurgo_chainlibs_Native_valueCheckedAdd(
  env: JNIEnv, _: JObject, ptr: JObject, other: JObject
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    let val = rptr.typed_ref::<Value>()?;
    let rother = other.rptr(&env)?;
    let otherval = rother.typed_ref::<Value>()?;
    let res = val.checked_add(otherval).into_result()?;
    RPtr::new(res).jptr(&env)
  }).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_io_emurgo_chainlibs_Native_valueCheckedSub(
  env: JNIEnv, _: JObject, ptr: JObject, other: JObject
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    let val = rptr.typed_ref::<Value>()?;
    let rother = other.rptr(&env)?;
    let otherval = rother.typed_ref::<Value>()?;
    let res = val.checked_sub(otherval).into_result()?;
    RPtr::new(res).jptr(&env)
  }).jresult(&env)
}