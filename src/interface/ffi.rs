use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::encoder::encoder::IMAGE_ENCODER;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::AvatarDataURL;
use crate::core::template::petpet_template::PetpetTemplate;
use crate::core::template::text_template::TextData;

#[repr(C)]
pub struct FFIResult {
    data: *const u8,
    length: usize,
    format: *const c_char,
}

#[repr(C)]
pub struct StringStringPair {
    key: *const c_char,
    value: *const c_char,
}

#[repr(C)]
pub struct BuildParams {
    map: *const StringStringPair,
    map_len: usize,
    list: *const *const c_char,
    list_len: usize,
}

#[no_mangle]
pub extern "C" fn create_builder(
    template: *const c_char,
    path: *const c_char,
) -> *const PetpetBuilder {
    let template_cstr = unsafe { CStr::from_ptr(template) };
    let template_raw: &str = template_cstr.to_str().unwrap();
    let path_cstr = unsafe { CStr::from_ptr(path) };
    let path: &str = path_cstr.to_str().unwrap();

    let template: PetpetTemplate = match serde_json::from_str(template_raw) {
        Ok(template) => template,
        Err(e) => {
            println!("{}", e);
            return std::ptr::null();
        }
    };
    match PetpetBuilder::new(template, path.parse().unwrap()) {
        Ok(builder) => Box::into_raw(Box::new(builder)),
        Err(e) => {
            println!("{}", e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn builder_build(
    builder: *const PetpetBuilder,
    avatar_params: *const BuildParams,
    text_params: *const BuildParams
) -> FFIResult {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    //TODO
    let (avatar_map, avatar_list) = params_to_map_list(avatar_params);
    let (text_map, text_list) = params_to_map_list(text_params);

    let form_key = "from";
    let to_key = "to";
    let bot_key = "bot";
    let group_key = "group";

    let builder = unsafe { &*builder };

    let (images, delay) = runtime.block_on(builder.build(
        create_avatar_data(
            &AvatarDataURL {
                from: avatar_map.get(form_key).cloned(),
                to: avatar_map.get(to_key).cloned(),
                bot: avatar_map.get(bot_key).cloned(),
                group: avatar_map.get(group_key).cloned(),
                random: Some(avatar_list),
            }
        ).unwrap(),
        TextData {
            from: text_map.get(form_key).unwrap_or(&form_key.to_owned()).to_owned(),
            to: text_map.get(to_key).unwrap_or(&to_key.to_owned()).to_owned(),
            group: text_map.get(group_key).unwrap_or(&group_key.to_owned()).to_owned(),
            text_list,
        },
    )).unwrap();
    let (blob, format) = IMAGE_ENCODER.encode(&images, delay).unwrap();
    let ptr = blob.as_ptr();
    let length = blob.len();
    let format = CString::new(format.to_format()).unwrap();
    std::mem::forget(blob);
    std::mem::forget(format);
    FFIResult {
        data: ptr,
        length,
        format: format.as_ptr() as *const c_char,
    }
}

pub fn params_to_map_list(params: *const BuildParams) -> (HashMap<String, String>, Vec<String>) {
    if params.is_null() {
        return (HashMap::with_capacity(0), Vec::with_capacity(0));
    }

    let params = unsafe { &*params };

    let mut hashmap = HashMap::new();
    for i in 0..params.map_len {
        let pair = unsafe { &*params.map.add(i) };
        let key = unsafe { CStr::from_ptr(pair.key) }.to_string_lossy().into_owned();
        let value = unsafe { CStr::from_ptr(pair.value) }.to_string_lossy().into_owned();
        hashmap.insert(key, value);
    }

    let mut vec_list = Vec::with_capacity(params.list_len);
    for i in 0..params.list_len {
        let c_str = unsafe { CStr::from_ptr(*params.list.add(i)) };
        let value = c_str.to_string_lossy().into_owned();
        vec_list.push(value);
    }

    (hashmap, vec_list)
}

#[no_mangle]
pub unsafe extern "C" fn free_builder(
    ptr: *mut PetpetBuilder
) {
    if ptr.is_null() {
        return;
    }
    let builder = Box::from_raw(ptr);
    std::mem::drop(builder)
}

