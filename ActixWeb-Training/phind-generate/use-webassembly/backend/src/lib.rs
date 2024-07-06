use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, ImageBuffer, Rgba};
use base64::encode;
use std::io::Cursor;

#[no_mangle]
#[wasm_bindgen]
pub fn grayscale_image(image_path: &str) -> Result<String, JsValue> {
    // 画像を読み込む
    let img = image::open(image_path).map_err(|e| JsValue::from_str(&e.to_string()))?;
    // 画像を灰色に変換
    let grayscale_img = img.grayscale();
    // 画像をBase64形式に変換
    let mut buffer = Cursor::new(Vec::new());
    grayscale_img.write_to(&mut buffer, ImageFormat::Png).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let base64_img = encode(buffer.into_inner());
    // Base64形式の画像を返す
    Ok(format!("data:image/png;base64,{}", base64_img))
}