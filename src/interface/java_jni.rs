extern crate jni;

use crate::core::builder::petpet_builder::PetpetBuilder;
use crate::core::encoder::encoder::{EncodeFormat, IMAGE_ENCODER};
use crate::core::errors::Error;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::{AvatarDataURL, PetpetData};
use crate::core::template::petpet_template::PetpetTemplate;
use crate::core::template::text_template::TextData;

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject, JString, AsJArrayRaw, JObjectArray};
use once_cell::sync::Lazy;


static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(||
tokio::runtime::Runtime::new().unwrap()
);

#[no_mangle]
pub extern "C" fn Java_PetpetRsBuilder_createBuilder<'local>(
    mut env: JNIEnv<'local>, _class: JClass<'local>,
    template: JString<'local>, path: JString<'local>,
) -> *const PetpetBuilder {
    let template_raw: String = env.get_string(&template).expect("Couldn't get java string!").into();
    let path: String = env.get_string(&path).expect("Couldn't get java string!").into();

    match create_builder(template_raw, path) {
        Ok(builder) => {
            Box::into_raw(Box::new(builder))
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            std::ptr::null()
        }
    }
}


fn create_builder(template: String, path: String) -> Result<PetpetBuilder, Error> {
    let template: PetpetTemplate = serde_json::from_str(&template)?;
    PetpetBuilder::new(template, path)
}

macro_rules! jni_string_option_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        let $field: JString = $env.get_field(&$obj, stringify!($field), "Ljava/lang/String;").unwrap().l().unwrap().into();
        let $field: Option<String> = if $field.is_null() {
            None
        } else {
            Some($env.get_string(&$field).unwrap().into())
        };
    }
}

macro_rules! jni_string_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        jni_string_option_prop!($env, $obj, $field);
        let $field = if let Some(str) = $field { str } else { "".to_string() };
    }
}

macro_rules! jni_string_array_option_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        let jarr_raw: JObjectArray = JObjectArray::from($env.get_field(&$obj, stringify!($field), "[Ljava/lang/String;").unwrap().l().unwrap());
        let $field: Option<Vec<String>> = if !jarr_raw.is_null() {
            let jarr_len = $env.get_array_length(&jarr_raw).unwrap();
            let mut strs: Vec<String> = Vec::with_capacity(jarr_len as usize);
            for i in 0..jarr_len {
                let element: JString = $env.get_object_array_element(&jarr_raw, i).unwrap().into();
                let rust_string: String = $env.get_string(&element).unwrap().into();
                strs.push(rust_string);
            }
            Some(strs)
        } else {
            None
        };
    }
}

macro_rules! jni_string_array_prop {
    ($env:expr, $obj:expr, $field:ident) => {
        jni_string_array_option_prop!($env, $obj, $field);
        let $field = if let Some(strs) = $field { strs } else { Vec::new() };
    }
}


#[no_mangle]
pub unsafe extern "C" fn Java_PetpetRsBuilder_builderBuildByString<'local>(
    mut env: JNIEnv<'local>, class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    data: JString<'local>,
) -> JByteArray<'local> {
    if ptr.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "");
        return JByteArray::default();
    }

    match builder_build_by_string(&mut env, class, ptr, data) {
        Ok(arr) => arr,
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            JByteArray::default()
        }
    }
}


unsafe fn builder_build_by_string<'local>(
    env: &mut JNIEnv<'local>, _class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    data: JString<'local>,
) -> Result<JByteArray<'local>, Error> {
    let data_raw: String = env.get_string(&data).expect("Couldn't get java string!").into();
    let data: PetpetData = serde_json::from_str(&data_raw)?;
    let builder = Box::from_raw(ptr);
    let avatar_data = create_avatar_data(&data.avatar)?;
    let (images, delay) = RUNTIME.block_on(
        builder.build(avatar_data, data.text)
    )?;
    let (blob, _format) = IMAGE_ENCODER.encode(&images, delay)?;
    Ok(env.byte_array_from_slice(&blob).unwrap())
}


#[no_mangle]
pub unsafe extern "C" fn Java_PetpetRsBuilder_builderBuildByObjects<'local>(
    mut env: JNIEnv<'local>, class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    avatar_data: JObject<'local>, text_data: JObject<'local>,
) -> JByteArray<'local> {
    if ptr.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "");
        return JByteArray::default();
    }

    match builder_build_by_objects(&mut env, class, ptr, avatar_data, text_data) {
        Ok(arr) => arr,
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", e));
            JByteArray::default()
        }
    }
}


unsafe fn builder_build_by_objects<'local>(
    env: &mut JNIEnv<'local>, _class: JClass<'local>,
    ptr: *mut PetpetBuilder,
    avatar_data: JObject<'local>, text_data: JObject<'local>,
) -> Result<JByteArray<'local>, Error> {
    let avatar_data_url = {
        jni_string_option_prop!(env, avatar_data, from);
        jni_string_option_prop!(env, avatar_data, to);
        jni_string_option_prop!(env, avatar_data, group);
        jni_string_option_prop!(env, avatar_data, bot);
        jni_string_array_option_prop!(env, avatar_data, random);
        AvatarDataURL { from, to, bot, group, random }
    };
    let text_data = {
        jni_string_prop!(env, text_data, from);
        jni_string_prop!(env, text_data, to);
        jni_string_prop!(env, text_data, group);
        jni_string_array_prop!(env, text_data, textList);
        TextData { from, to, group, text_list: textList }
    };

    let builder = Box::from_raw(ptr);
    let avatar_data = create_avatar_data(&avatar_data_url)?;
    let (images, delay) = RUNTIME.block_on(
        builder.build(avatar_data, text_data)
    )?;
    let (blob, _format) = IMAGE_ENCODER.encode(&images, delay)?;
    Ok(env.byte_array_from_slice(&blob).unwrap())
}


#[no_mangle]
pub unsafe extern "C" fn Java_PetpetRsBuilder_closeBuilder(_env: JNIEnv, _class: JClass, ptr: *mut PetpetBuilder) {
    if ptr.is_null() {
        return;
    }
    let builder = Box::from_raw(ptr);
    std::mem::drop(builder)
}