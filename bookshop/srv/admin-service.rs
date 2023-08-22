mod utils;

use std::future::Future;
use std::result::Result::*;

use js_sys::Array;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
  // Import CQL to create CQN
  fn CQL(s: &str) -> JsValue;
  // Import SELECT to convert CQN to an Query instance
  async fn SELECT(s: JsValue) -> JsValue;
}

// Export phase function to determine execution phase
#[wasm_bindgen]
pub fn phase() -> JsValue {
  return "before".into();
}

// Export event function to determine trigger event
#[wasm_bindgen]
pub fn event() -> JsValue {
  return "CREATE".into();
}

// Export entity function to determine target entity name
#[wasm_bindgen]
pub fn entity() -> JsValue {
  return "Books".into();
}

// Export exec function to attach to the event
#[wasm_bindgen]
pub async fn exec(req: JsValue) -> Result<(), JsValue> {

  // Create CQN object
  let cqn = CQL("SELECT MAX(ID) AS ID FROM AdminService.Books");
  // Convert CQN to Query and call then
  let result = SELECT(cqn).await;

  // ID = "ID"
  let ID = JsValue::from_str("ID");

  // result[0]
  let first = Reflect::get(&result,&0f64.into())?;

  // (id = first[ID]) && typeof id === "number" ? id : throw
  let id = Reflect::get(&first, &ID)?.as_f64().ok_or(JsValue::from_str("ID must be a Number"))?;

  // Compute newId just as original implementation
  let newId = id - id % 100.0 + 100.0 + 1.0;

  // req.data
  let data = Reflect::get(&req, &JsValue::from_str("data"))?;
  // data.ID = newId
  let _ = Reflect::set(&data, &ID, &newId.into())?;

  // return undefined
  return Ok(());
}
