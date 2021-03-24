use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "writeUserDataSetor")]
    pub fn write_user_data_setor(userId: JsValue, setor: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataId")]
    pub fn write_user_data_id(userId: JsValue, id: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataHdd")]
    pub fn write_user_data_hdd(userId: JsValue, hdd: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataCpu")]
    pub fn write_user_data_cpu(userId: JsValue, cpu: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataOs")]
    pub fn write_user_data_os(userId: JsValue, os: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataUser")]
    pub fn write_user_data_user(userId: JsValue, user: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataMarca")]
    pub fn write_user_data_marca(userId: JsValue, marca: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataMonitor")]
    pub fn write_user_data_monitor(userId: JsValue, monitor: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataTamMonitor")]
    pub fn write_user_data_tam_monitor(userId: JsValue, tamMonitor: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataRam")]
    pub fn write_user_data_ram(userId: JsValue, ram: JsValue);
    #[wasm_bindgen(js_name = "writeUserDataStatus")]
    pub fn write_user_data_status(userId: JsValue, status: JsValue);
}