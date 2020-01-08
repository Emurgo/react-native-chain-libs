use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Address, Fee, Input, InputOutputBuilder, OutputPolicy, Payload, Value};

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_empty(
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| InputOutputBuilder::empty().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_add_input(
  io_builder: RPtr, input: RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(input.typed_ref::<Input>())
      .and_then(|(io_builder, input)| io_builder.add_input(input).into_result())
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_add_output(
  io_builder: RPtr, address: RPtr, value: RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(address.typed_ref::<Address>())
      .zip(value.typed_ref::<Value>())
      .and_then(|((io_builder, address), value)| {
        io_builder.add_output(address, value).into_result()
      })
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_get_balance(
  io_builder: RPtr, payload: RPtr, fee: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(payload.typed_ref::<Payload>())
      .zip(fee.typed_ref::<Fee>())
      .map(|((io_builder, payload), fee)| io_builder.get_balance(payload, fee).into_result())
  })
  .map(|balance| balance.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_estimate_fee(
  io_builder: RPtr, fee: RPtr, payload: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(fee.typed_ref::<Fee>())
      .zip(payload.typed_ref::<Payload>())
      .map(|((io_builder, fee), payload)| io_builder.estimate_fee(fee, payload))
  })
  .map(|value| value.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_build(
  io_builder: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder.owned::<InputOutputBuilder>().map(|io_builder| io_builder.build())
  })
  .map(|input_output| input_output.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_builder_seal_with_output_policy(
  io_builder: &mut RPtr, payload: RPtr, fee_algorithm: RPtr, policy: RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    io_builder
      .owned::<InputOutputBuilder>()
      .zip(payload.typed_ref::<Payload>())
      .zip(fee_algorithm.typed_ref::<Fee>())
      .zip(policy.typed_ref::<OutputPolicy>())
      .and_then(|(((io_builder, payload), fee_algorithm), policy)| {
        io_builder.seal_with_output_policy(payload, fee_algorithm, policy).into_result()
      })
  })
  .map(|input_output| input_output.rptr())
  .response(result, error)
}
