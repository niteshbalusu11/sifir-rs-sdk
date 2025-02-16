#[allow(dead_code)]
mod internal_aliases {
    use super::*;
    pub type JStringOptStr = jstring;
    pub type JOptionalInt = jobject;
    pub type JInteger = jobject;
    pub type JByte = jobject;
    pub type JShort = jobject;
    pub type JFloat = jobject;
    pub type JDouble = jobject;
    pub type JOptionalDouble = jobject;
    pub type JLong = jobject;
    pub type JOptionalLong = jobject;
    #[repr(transparent)]
    pub struct JForeignObjectsArray<T: SwigForeignClass> {
        pub(crate) inner: jobjectArray,
        pub(crate) _marker: ::std::marker::PhantomData<T>,
    }
    pub type JStringPath = jstring;
    pub type JStringObjectsArray = jobjectArray;
}
#[doc = " Default JNI_VERSION"]
const SWIG_JNI_VERSION: jint = JNI_VERSION_1_6 as jint;
#[doc = " Marker for what to cache in JNI_OnLoad"]
#[allow(unused_macros)]
macro_rules! swig_jni_find_class {
    ($ id : ident , $ path : expr) => {
        unsafe { $id }
    };
    ($ id : ident , $ path : expr ,) => {
        unsafe { $id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_method_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_method_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_field_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_field_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(dead_code)]
#[doc = ""]
trait SwigInto<T> {
    fn swig_into(self, env: *mut JNIEnv) -> T;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigFrom<T> {
    fn swig_from(_: T, env: *mut JNIEnv) -> Self;
}
#[allow(unused_macros)]
macro_rules! swig_c_str {
    ($ lit : expr) => {
        concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
#[allow(unused_macros)]
macro_rules ! swig_assert_eq_size { ($ x : ty , $ ($ xs : ty) ,+ $ (,) *) => { $ (let _ = :: std :: mem :: transmute ::<$ x , $ xs >;) + } ; }
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}
#[allow(dead_code)]
pub trait SwigForeignClass {
    type PointedType;
    fn jni_class() -> jclass;
    fn jni_class_pointer_field() -> jfieldID;
    fn box_object(x: Self) -> jlong;
    fn unbox_object(x: jlong) -> Self;
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType>;
}
#[allow(dead_code)]
pub trait SwigForeignCLikeEnum {
    fn as_jint(&self) -> jint;
    #[doc = " # Panics"]
    #[doc = " Panics on error"]
    fn from_jint(_: jint) -> Self;
}
#[allow(dead_code)]
struct JavaCallback {
    java_vm: *mut JavaVM,
    this: jobject,
    methods: Vec<jmethodID>,
}
#[doc = " According to JNI spec it should be safe to"]
#[doc = " pass pointer to JavaVm and jobject (global) across threads"]
unsafe impl Send for JavaCallback {}
#[allow(dead_code)]
struct JniEnvHolder<'a> {
    env: Option<*mut JNIEnv>,
    callback: &'a JavaCallback,
    need_detach: bool,
}
#[allow(dead_code)]
impl<'a> Drop for JniEnvHolder<'a> {
    fn drop(&mut self) {
        if self.need_detach {
            let res = unsafe {
                (**self.callback.java_vm).DetachCurrentThread.unwrap()(self.callback.java_vm)
            };
            if res != 0 {
                log::error!("JniEnvHolder: DetachCurrentThread failed: {}", res);
            }
        }
    }
}
#[allow(dead_code)]
impl JavaCallback {
    fn new(obj: jobject, env: *mut JNIEnv) -> JavaCallback {
        let mut java_vm: *mut JavaVM = ::std::ptr::null_mut();
        let ret = unsafe { (**env).GetJavaVM.unwrap()(env, &mut java_vm) };
        assert_eq!(0, ret, "GetJavaVm failed");
        let global_obj = unsafe { (**env).NewGlobalRef.unwrap()(env, obj) };
        assert!(!global_obj.is_null());
        JavaCallback {
            java_vm,
            this: global_obj,
            methods: Vec::new(),
        }
    }
    fn get_jni_env(&self) -> JniEnvHolder {
        assert!(!self.java_vm.is_null());
        let mut env: *mut JNIEnv = ::std::ptr::null_mut();
        let res = unsafe {
            (**self.java_vm).GetEnv.unwrap()(
                self.java_vm,
                (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
                SWIG_JNI_VERSION,
            )
        };
        if res == (JNI_OK as jint) {
            return JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: false,
            };
        }
        if res != (JNI_EDETACHED as jint) {
            panic!("get_jni_env: GetEnv return error `{}`", res);
        }
        trait ConvertPtr<T> {
            fn convert_ptr(self) -> T;
        }
        impl ConvertPtr<*mut *mut ::std::os::raw::c_void> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut ::std::os::raw::c_void {
                self as *mut *mut ::std::os::raw::c_void
            }
        }
        impl ConvertPtr<*mut *mut JNIEnv> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut JNIEnv {
                self
            }
        }
        let res = unsafe {
            (**self.java_vm).AttachCurrentThread.unwrap()(
                self.java_vm,
                (&mut env as *mut *mut JNIEnv).convert_ptr(),
                ::std::ptr::null_mut(),
            )
        };
        if res != 0 {
            log::error!(
                "JavaCallback::get_jnienv: AttachCurrentThread failed: {}",
                res
            );
            JniEnvHolder {
                env: None,
                callback: self,
                need_detach: false,
            }
        } else {
            assert!(!env.is_null());
            JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: true,
            }
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaCallback {
    fn drop(&mut self) {
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            assert!(!env.is_null());
            unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.this) };
        } else {
            log::error!("JavaCallback::drop failed, can not get JNIEnv");
        }
    }
}
#[allow(dead_code)]
fn jni_throw(env: *mut JNIEnv, ex_class: jclass, message: &str) {
    let c_message = ::std::ffi::CString::new(message).unwrap();
    let res = unsafe { (**env).ThrowNew.unwrap()(env, ex_class, c_message.as_ptr()) };
    if res != 0 {
        log::error!(
            "JNI ThrowNew({}) failed for class {:?} failed",
            message,
            ex_class
        );
    }
}
#[allow(dead_code)]
fn jni_throw_exception(env: *mut JNIEnv, message: &str) {
    let exception_class = swig_jni_find_class!(JAVA_LANG_EXCEPTION, "java/lang/Exception");
    jni_throw(env, exception_class, message)
}
#[allow(dead_code)]
fn object_to_jobject<T: SwigForeignClass>(env: *mut JNIEnv, obj: T) -> jobject {
    let jcls = <T>::jni_class();
    assert!(!jcls.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");
    let ret: jlong = <T>::box_object(obj);
    unsafe {
        (**env).SetLongField.unwrap()(env, jobj, field_id, ret);
        if (**env).ExceptionCheck.unwrap()(env) != 0 {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }
    jobj
}
#[allow(dead_code)]
fn jobject_array_to_vec_of_objects<T: SwigForeignClass + Clone>(
    env: *mut JNIEnv,
    arr: internal_aliases::JForeignObjectsArray<T>,
) -> Vec<T> {
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let length = unsafe { (**env).GetArrayLength.unwrap()(env, arr.inner) };
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversion");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: &mut T = unsafe {
            let obj = (**env).GetObjectArrayElement.unwrap()(env, arr.inner, i);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let ptr = (**env).GetLongField.unwrap()(env, obj, field_id);
            let native = (jlong_to_pointer(ptr) as *mut T).as_mut().unwrap();
            (**env).DeleteLocalRef.unwrap()(env, obj);
            native
        };
        result.push(native.clone());
    }
    result
}
#[allow(dead_code)]
fn vec_of_objects_to_jobject_array<T: SwigForeignClass>(
    env: *mut JNIEnv,
    mut arr: Vec<T>,
) -> internal_aliases::JForeignObjectsArray<T> {
    let jcls: jclass = <T>::jni_class();
    assert!(!jcls.is_null());
    let arr_len = <jsize as ::std::convert::TryFrom<usize>>::try_from(arr.len())
        .expect("invalid usize, in usize => to jsize conversion");
    let obj_arr: jobjectArray =
        unsafe { (**env).NewObjectArray.unwrap()(env, arr_len, jcls, ::std::ptr::null_mut()) };
    assert!(!obj_arr.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    for (i, r_obj) in arr.drain(..).enumerate() {
        let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
        assert!(!jobj.is_null());
        let r_obj: jlong = <T>::box_object(r_obj);
        unsafe {
            (**env).SetLongField.unwrap()(env, jobj, field_id, r_obj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Can not mNativeObj field: catch exception");
            }
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jobj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jobj);
        }
    }
    internal_aliases::JForeignObjectsArray {
        inner: obj_arr,
        _marker: ::std::marker::PhantomData,
    }
}
#[allow(dead_code)]
trait JniInvalidValue {
    fn jni_invalid_value() -> Self;
}
impl<T> JniInvalidValue for *const T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null()
    }
}
impl<T> JniInvalidValue for *mut T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null_mut()
    }
}
impl JniInvalidValue for () {
    fn jni_invalid_value() {}
}
impl<T: SwigForeignClass> JniInvalidValue for internal_aliases::JForeignObjectsArray<T> {
    fn jni_invalid_value() -> Self {
        Self {
            inner: ::std::ptr::null_mut(),
            _marker: ::std::marker::PhantomData,
        }
    }
}
macro_rules ! impl_jni_jni_invalid_value { ($ ($ type : ty) *) => ($ (impl JniInvalidValue for $ type { fn jni_invalid_value () -> Self { <$ type >:: default () } }) *) }
impl_jni_jni_invalid_value! { jbyte jshort jint jlong jfloat jdouble jboolean }
#[allow(dead_code)]
pub fn u64_to_jlong_checked(x: u64) -> jlong {
    <jlong as ::std::convert::TryFrom<u64>>::try_from(x)
        .expect("invalid u64, in u64 => jlong conversion")
}
#[allow(dead_code)]
struct JavaUTF16Slice {
    string: jstring,
    chars: *const ::std::os::raw::c_ushort,
    len: usize,
    env: *mut JNIEnv,
}
#[allow(dead_code)]
impl JavaUTF16Slice {
    pub fn new(env: *mut JNIEnv, js: jstring) -> JavaUTF16Slice {
        let len = unsafe { (**env).GetStringLength.unwrap()(env, js) };
        let chars = unsafe { (**env).GetStringChars.unwrap()(env, js, ::std::ptr::null_mut()) };
        JavaUTF16Slice {
            string: js,
            chars,
            len: len as usize,
            env,
        }
    }
    pub fn as_slice(&self) -> &[u16] {
        unsafe { ::std::slice::from_raw_parts(self.chars, self.len) }
    }
    pub fn into_string(self) -> String {
        String::from_utf16(self.as_slice()).unwrap()
    }
}
#[allow(dead_code)]
impl Drop for JavaUTF16Slice {
    fn drop(&mut self) {
        unsafe {
            (**self.env).ReleaseStringChars.unwrap()(self.env, self.string, self.chars);
        }
    }
}
#[allow(dead_code)]
fn from_jstring_std_string(js: jstring, env: *mut JNIEnv) -> String {
    if !js.is_null() {
        JavaUTF16Slice::new(env, js).into_string()
    } else {
        "".to_string()
    }
}
#[allow(dead_code)]
fn from_std_str_jstring(x: &str, env: *mut JNIEnv) -> jstring {
    let x: Vec<::std::os::raw::c_ushort> = x.encode_utf16().collect();
    unsafe { (**env).NewString.unwrap()(env, x.as_ptr(), x.len() as i32) }
}
#[allow(dead_code)]
fn vec_string_to_jobject_array(mut arr: Vec<String>, env: *mut JNIEnv) -> jobjectArray {
    let jcls: jclass = swig_jni_find_class!(JAVA_LANG_STRING, "java/lang/String");
    assert!(!jcls.is_null());
    let obj_arr: jobjectArray = unsafe {
        (**env).NewObjectArray.unwrap()(env, arr.len() as jsize, jcls, ::std::ptr::null_mut())
    };
    assert!(!obj_arr.is_null());
    for (i, r_str) in arr.drain(..).enumerate() {
        let jstr: jstring = from_std_str_jstring(&r_str, env);
        assert!(!jstr.is_null());
        unsafe {
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jstr);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jstr);
        }
    }
    obj_arr
}
macro_rules ! define_array_handling_code { ($ ([jni_arr_type = $ jni_arr_type : ident , rust_arr_wrapper = $ rust_arr_wrapper : ident , jni_get_array_elements = $ jni_get_array_elements : ident , jni_elem_type = $ jni_elem_type : ident , rust_elem_type = $ rust_elem_type : ident , jni_release_array_elements = $ jni_release_array_elements : ident , jni_new_array = $ jni_new_array : ident , jni_set_array_region = $ jni_set_array_region : ident]) ,*) => { $ (# [allow (dead_code)] struct $ rust_arr_wrapper { array : $ jni_arr_type , data : * mut $ jni_elem_type , env : * mut JNIEnv , } # [allow (dead_code)] impl $ rust_arr_wrapper { fn new (env : * mut JNIEnv , array : $ jni_arr_type) -> $ rust_arr_wrapper { assert ! (! array . is_null ()) ; let data = unsafe { (** env) .$ jni_get_array_elements . unwrap () (env , array , :: std :: ptr :: null_mut ()) } ; $ rust_arr_wrapper { array , data , env } } fn to_slice (& self) -> & [$ rust_elem_type] { unsafe { let len : jsize = (** self . env) . GetArrayLength . unwrap () (self . env , self . array) ; assert ! ((len as u64) <= (usize :: max_value () as u64)) ; :: std :: slice :: from_raw_parts (self . data , len as usize) } } fn from_slice_to_raw (arr : & [$ rust_elem_type] , env : * mut JNIEnv) -> $ jni_arr_type { assert ! ((arr . len () as u64) <= (jsize :: max_value () as u64)) ; let jarr : $ jni_arr_type = unsafe { (** env) .$ jni_new_array . unwrap () (env , arr . len () as jsize) } ; assert ! (! jarr . is_null ()) ; unsafe { (** env) .$ jni_set_array_region . unwrap () (env , jarr , 0 , arr . len () as jsize , arr . as_ptr ()) ; if (** env) . ExceptionCheck . unwrap () (env) != 0 { panic ! ("{}:{} {} failed" , file ! () , line ! () , stringify ! ($ jni_set_array_region)) ; } } jarr } } # [allow (dead_code)] impl Drop for $ rust_arr_wrapper { fn drop (& mut self) { assert ! (! self . env . is_null ()) ; assert ! (! self . array . is_null ()) ; unsafe { (** self . env) .$ jni_release_array_elements . unwrap () (self . env , self . array , self . data , JNI_ABORT as jint ,) } ; } }) * } }
define_array_handling_code!(
    [
        jni_arr_type = jbyteArray,
        rust_arr_wrapper = JavaByteArray,
        jni_get_array_elements = GetByteArrayElements,
        jni_elem_type = jbyte,
        rust_elem_type = i8,
        jni_release_array_elements = ReleaseByteArrayElements,
        jni_new_array = NewByteArray,
        jni_set_array_region = SetByteArrayRegion
    ],
    [
        jni_arr_type = jshortArray,
        rust_arr_wrapper = JavaShortArray,
        jni_get_array_elements = GetShortArrayElements,
        jni_elem_type = jshort,
        rust_elem_type = i16,
        jni_release_array_elements = ReleaseShortArrayElements,
        jni_new_array = NewShortArray,
        jni_set_array_region = SetShortArrayRegion
    ],
    [
        jni_arr_type = jintArray,
        rust_arr_wrapper = JavaIntArray,
        jni_get_array_elements = GetIntArrayElements,
        jni_elem_type = jint,
        rust_elem_type = i32,
        jni_release_array_elements = ReleaseIntArrayElements,
        jni_new_array = NewIntArray,
        jni_set_array_region = SetIntArrayRegion
    ],
    [
        jni_arr_type = jlongArray,
        rust_arr_wrapper = JavaLongArray,
        jni_get_array_elements = GetLongArrayElements,
        jni_elem_type = jlong,
        rust_elem_type = i64,
        jni_release_array_elements = ReleaseLongArrayElements,
        jni_new_array = NewLongArray,
        jni_set_array_region = SetLongArrayRegion
    ],
    [
        jni_arr_type = jfloatArray,
        rust_arr_wrapper = JavaFloatArray,
        jni_get_array_elements = GetFloatArrayElements,
        jni_elem_type = jfloat,
        rust_elem_type = f32,
        jni_release_array_elements = ReleaseFloatArrayElements,
        jni_new_array = NewFloatArray,
        jni_set_array_region = SetFloatArrayRegion
    ],
    [
        jni_arr_type = jdoubleArray,
        rust_arr_wrapper = JavaDoubleArray,
        jni_get_array_elements = GetDoubleArrayElements,
        jni_elem_type = jdouble,
        rust_elem_type = f64,
        jni_release_array_elements = ReleaseDoubleArrayElements,
        jni_new_array = NewDoubleArray,
        jni_set_array_region = SetDoubleArrayRegion
    ]
);
#[allow(dead_code)]
fn to_java_util_optional_double(
    env: *mut JNIEnv,
    x: Option<f64>,
) -> internal_aliases::JOptionalDouble {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_DOUBLE, "java/util/OptionalDouble");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_OF,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "of",
                "(D)Ljava/util/OptionalDouble;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "empty",
                "()Ljava/util/OptionalDouble;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_double_to_rust(env: *mut JNIEnv, x: internal_aliases::JDouble) -> Option<f64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_DOUBLE, "java/lang/Double");
            assert!(!class.is_null());
            let double_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD,
                JAVA_LANG_DOUBLE,
                "doubleValue",
                "()D",
            );
            assert!(!double_value_m.is_null(),);
            let ret: f64 = unsafe {
                let ret = (**env).CallDoubleMethod.unwrap()(env, x, double_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Double.doubleValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_float_to_rust(env: *mut JNIEnv, x: internal_aliases::JFloat) -> Option<f32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_FLOAT, "java/lang/Float");
            assert!(!class.is_null());
            let float_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_FLOAT_FLOAT_VALUE,
                JAVA_LANG_FLOAT,
                "floatValue",
                "()F"
            );
            assert!(!float_value_m.is_null());
            let ret: f32 = unsafe {
                let ret = (**env).CallFloatMethod.unwrap()(env, x, float_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Float.floatValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_long(env: *mut JNIEnv, x: Option<i64>) -> internal_aliases::JOptionalLong {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_LONG, "java/util/OptionalLong");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_OF,
                JAVA_UTIL_OPTIONAL_LONG,
                "of",
                "(J)Ljava/util/OptionalLong;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_EMPTY,
                JAVA_UTIL_OPTIONAL_LONG,
                "empty",
                "()Ljava/util/OptionalLong;",
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_long_to_rust(env: *mut JNIEnv, x: internal_aliases::JLong) -> Option<i64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_LONG, "java/lang/Long");
            assert!(!class.is_null());
            let long_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_LONG_LONG_VALUE,
                JAVA_LANG_LONG,
                "longValue",
                "()J"
            );
            assert!(!long_value_m.is_null());
            let ret: i64 = unsafe {
                let ret = (**env).CallLongMethod.unwrap()(env, x, long_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Long.longValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_int_to_rust(env: *mut JNIEnv, x: internal_aliases::JInteger) -> Option<i32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_INTEGER, "java/lang/Integer");
            assert!(!class.is_null());
            let int_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_INTEGER_INT_VALUE,
                JAVA_LANG_INTEGER,
                "intValue",
                "()I"
            );
            assert!(!int_value_m.is_null(),);
            let ret: i32 = unsafe {
                let ret = (**env).CallIntMethod.unwrap()(env, x, int_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Integer.intValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_byte_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i8> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_BYTE, "java/lang/Byte");
            assert!(!class.is_null());
            let byte_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_BYTE_BYTE_VALUE,
                JAVA_LANG_BYTE,
                "byteValue",
                "()B"
            );
            assert!(!byte_value_m.is_null(),);
            let ret: i8 = unsafe {
                let ret = (**env).CallByteMethod.unwrap()(env, x, byte_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Byte.byteValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_short_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i16> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_SHORT, "java/lang/Short");
            assert!(!class.is_null());
            let short_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_SHORT_SHORT_VALUE,
                JAVA_LANG_SHORT,
                "shortValue",
                "()S"
            );
            assert!(!short_value_m.is_null());
            let ret: i16 = unsafe {
                let ret = (**env).CallShortMethod.unwrap()(env, x, short_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Short.shortValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_int(env: *mut JNIEnv, x: Option<i32>) -> jobject {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_INT, "java/util/OptionalInt");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_OF,
                JAVA_UTIL_OPTIONAL_INT,
                "of",
                "(I)Ljava/util/OptionalInt;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_EMPTY,
                JAVA_UTIL_OPTIONAL_INT,
                "empty",
                "()Ljava/util/OptionalInt;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
use jni_sys::*;
use logger::{log, Logger};
use std::time::Duration;
use tor::{
    hidden_service::HiddenServiceHandler,
    tcp_stream::{DataObserver, TcpSocksStream},
    OwnedTorService, TorHiddenService, TorHiddenServiceParam, TorServiceParam,
};
unsafe impl Send for Observer {}
unsafe impl Sync for Observer {}
struct Observer {
    cb: Box<dyn DataObserver>,
}
impl DataObserver for Observer {
    fn on_data(&self, data: String) {
        self.cb.on_data(data);
    }
    fn on_error(&self, data: String) {
        self.cb.on_error(data);
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence1(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence2(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence3(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence4(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence5(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence6(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence7(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_JNIReachabilityFence_reachabilityFence8(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
impl DataObserver for JavaCallback {
    #[allow(unused_mut)]
    fn on_data(&self, a0: String) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jstring = from_std_str_jstring(&a0, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[0usize], a0);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(stringify!(on_data), ": java throw exception"));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
    #[allow(unused_mut)]
    fn on_error(&self, a0: String) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jstring = from_std_str_jstring(&a0, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[1usize], a0);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(stringify!(on_error), ": java throw exception"));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
}
impl SwigForeignClass for HiddenServiceHandler {
    type PointedType = HiddenServiceHandler;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_HIDDENSERVICEHANDLER,
            "com/sifir/tor/HiddenServiceHandler"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_HIDDENSERVICEHANDLER_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_HIDDENSERVICEHANDLER,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<HiddenServiceHandler> = Box::new(this);
        let this: *mut HiddenServiceHandler = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut HiddenServiceHandler = unsafe {
            jlong_to_pointer::<HiddenServiceHandler>(x)
                .as_mut()
                .unwrap()
        };
        let x: Box<HiddenServiceHandler> = unsafe { Box::from_raw(x) };
        let x: HiddenServiceHandler = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut HiddenServiceHandler = unsafe {
            jlong_to_pointer::<HiddenServiceHandler>(x)
                .as_mut()
                .unwrap()
        };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[doc = ""]
impl SwigFrom<jobject> for Box<dyn DataObserver> {
    fn swig_from(this: jobject, env: *mut JNIEnv) -> Self {
        let mut cb = JavaCallback::new(this, env);
        cb.methods.reserve(2);
        let class = unsafe { (**env).GetObjectClass.unwrap()(env, cb.this) };
        assert!(
            !class.is_null(),
            "GetObjectClass return null class for DataObserver"
        );
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onData"),
                swig_c_str!("(Ljava/lang/String;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onData id");
        cb.methods.push(method_id);
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("onError"),
                swig_c_str!("(Ljava/lang/String;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find onError id");
        cb.methods.push(method_id);
        Box::new(cb)
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_HiddenServiceHandler_init(
    env: *mut JNIEnv,
    _: jclass,
    dst_port: jint,
    cb: jobject,
) -> jlong {
    let mut dst_port: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(dst_port)
        .expect("invalid jint, in jint => u16 conversion");
    let mut cb: Box<dyn DataObserver> = <Box<dyn DataObserver>>::swig_from(cb, env);
    let this: Result<HiddenServiceHandler, String> = {
        let mut lsnr = HiddenServiceHandler::new(dst_port)
            .map_err(|e| format!("{:#?}", e))
            .unwrap();
        lsnr.set_data_handler(Observer { cb })
            .map_err(|e| format!("{:#?}", e))
            .unwrap();
        let _ = lsnr.start_http_listener();
        Ok(lsnr)
    };
    let mut this: jlong = match this {
        Ok(x) => {
            let ret: jlong = <HiddenServiceHandler>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    this as jlong
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_HiddenServiceHandler_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut HiddenServiceHandler = unsafe {
        jlong_to_pointer::<HiddenServiceHandler>(this)
            .as_mut()
            .unwrap()
    };
    let this: Box<HiddenServiceHandler> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for TorHiddenService {
    type PointedType = TorHiddenService;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_TORHIDDENSERVICE,
            "com/sifir/tor/TorHiddenService"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TORHIDDENSERVICE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TORHIDDENSERVICE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<TorHiddenService> = Box::new(this);
        let this: *mut TorHiddenService = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut TorHiddenService =
            unsafe { jlong_to_pointer::<TorHiddenService>(x).as_mut().unwrap() };
        let x: Box<TorHiddenService> = unsafe { Box::from_raw(x) };
        let x: TorHiddenService = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut TorHiddenService =
            unsafe { jlong_to_pointer::<TorHiddenService>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TorHiddenService_do_1get_1onion_1url(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &TorHiddenService =
        unsafe { jlong_to_pointer::<TorHiddenService>(this).as_mut().unwrap() };
    let mut ret: String = { this.onion_url.to_string() };
    let mut ret: jstring = from_std_str_jstring(&ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TorHiddenService_do_1get_1secret_1b64(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &TorHiddenService =
        unsafe { jlong_to_pointer::<TorHiddenService>(this).as_mut().unwrap() };
    let mut ret: String = { base64::encode(this.secret_key).into() };
    let mut ret: jstring = from_std_str_jstring(&ret, env);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TorHiddenService_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut TorHiddenService =
        unsafe { jlong_to_pointer::<TorHiddenService>(this).as_mut().unwrap() };
    let this: Box<TorHiddenService> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for TorServiceParam {
    type PointedType = TorServiceParam;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_TORSERVICEPARAM,
            "com/sifir/tor/TorServiceParam"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TORSERVICEPARAM_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TORSERVICEPARAM,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<TorServiceParam> = Box::new(this);
        let this: *mut TorServiceParam = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut TorServiceParam =
            unsafe { jlong_to_pointer::<TorServiceParam>(x).as_mut().unwrap() };
        let x: Box<TorServiceParam> = unsafe { Box::from_raw(x) };
        let x: TorServiceParam = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut TorServiceParam =
            unsafe { jlong_to_pointer::<TorServiceParam>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TorServiceParam_init(
    env: *mut JNIEnv,
    _: jclass,
    data_dir: jstring,
    socks_port: jint,
    bootstap_timeout_ms: jlong,
) -> jlong {
    let mut data_dir: String = from_jstring_std_string(data_dir, env);
    let mut data_dir: &str = data_dir.as_str();
    let mut socks_port: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(socks_port)
        .expect("invalid jint, in jint => u16 conversion");
    let mut bootstap_timeout_ms: u64 =
        <u64 as ::std::convert::TryFrom<jlong>>::try_from(bootstap_timeout_ms)
            .expect("invalid jlong, in jlong => u64 conversion");
    let this: TorServiceParam = TorServiceParam::new(data_dir, socks_port, bootstap_timeout_ms);
    let this: Box<TorServiceParam> = Box::new(this);
    let this: *mut TorServiceParam = Box::into_raw(this);
    this as jlong
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TorServiceParam_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut TorServiceParam =
        unsafe { jlong_to_pointer::<TorServiceParam>(this).as_mut().unwrap() };
    let this: Box<TorServiceParam> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for OwnedTorService {
    type PointedType = OwnedTorService;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_OWNEDTORSERVICE,
            "com/sifir/tor/OwnedTorService"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_OWNEDTORSERVICE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_OWNEDTORSERVICE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<OwnedTorService> = Box::new(this);
        let this: *mut OwnedTorService = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut OwnedTorService =
            unsafe { jlong_to_pointer::<OwnedTorService>(x).as_mut().unwrap() };
        let x: Box<OwnedTorService> = unsafe { Box::from_raw(x) };
        let x: OwnedTorService = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut OwnedTorService =
            unsafe { jlong_to_pointer::<OwnedTorService>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_init(
    env: *mut JNIEnv,
    _: jclass,
    param: jlong,
) -> jlong {
    let param: *mut TorServiceParam =
        unsafe { jlong_to_pointer::<TorServiceParam>(param).as_mut().unwrap() };
    let param: Box<TorServiceParam> = unsafe { Box::from_raw(param) };
    let param: TorServiceParam = *param;
    let this: Result<OwnedTorService, String> = {
        Logger::new();
        OwnedTorService::new(param).map_err(|e| format!("{:#?}", e))
    };
    let mut this: jlong = match this {
        Ok(x) => {
            let ret: jlong = <OwnedTorService>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1getSocksPort(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jint {
    let this: &OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let mut ret: u16 = { this.socks_port };
    let mut ret: jint = jint::from(ret);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1shutdown(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> () {
    let this: &mut OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let mut ret: Result<(), String> = { this.shutdown().map_err(|e| format!("{:#?}", e)) };
    let mut ret: () = match ret {
        Ok(x) => {
            let mut ret = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <()>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1get_1status(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &mut OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let mut ret: String = {
        let node_status = this.get_status();
        match node_status {
            Ok(status) => {
                let status_string = serde_json::to_string(&status).unwrap();
                status_string
            }
            Err(e) => e.to_string(),
        }
    };
    let mut ret: jstring = from_std_str_jstring(&ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1create_1hidden_1service(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    dst_port: jint,
    hs_port: jint,
    secret_key: jstring,
) -> jlong {
    let mut dst_port: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(dst_port)
        .expect("invalid jint, in jint => u16 conversion");
    let mut hs_port: u16 = <u16 as ::std::convert::TryFrom<jint>>::try_from(hs_port)
        .expect("invalid jint, in jint => u16 conversion");
    let mut secret_key: String = from_jstring_std_string(secret_key, env);
    let this: &mut OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let mut ret: Result<TorHiddenService, String> = {
        let hs_key = match secret_key.len() {
            0 => Ok(None),
            _ => {
                let mut decoded_buff: [u8; 64] = [0; 64];
                base64::decode_config_slice(secret_key, base64::STANDARD, &mut decoded_buff)
                    .map(|_| Some(decoded_buff))
            }
        };
        match hs_key {
            Ok(key) => this
                .create_hidden_service(TorHiddenServiceParam {
                    to_port: dst_port,
                    hs_port,
                    secret_key: key,
                })
                .map_err(|e| format!("{:#?}", e)),
            Err(e) => Err(format!("{:#?}", e)),
        }
    };
    let mut ret: jlong = match ret {
        Ok(x) => {
            let ret: jlong = <TorHiddenService>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1delete_1hidden_1service(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    onion: jstring,
) -> () {
    let mut onion: String = from_jstring_std_string(onion, env);
    let this: &mut OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let mut ret: Result<(), String> = {
        this.delete_hidden_service(onion)
            .map_err(|e| format!("{:#?}", e))
    };
    let mut ret: () = match ret {
        Ok(x) => {
            let mut ret = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <()>::jni_invalid_value();
        }
    };
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_OwnedTorService_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut OwnedTorService =
        unsafe { jlong_to_pointer::<OwnedTorService>(this).as_mut().unwrap() };
    let this: Box<OwnedTorService> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for TcpSocksStream {
    type PointedType = TcpSocksStream;
    fn jni_class() -> jclass {
        swig_jni_find_class!(FOREIGN_CLASS_TCPSOCKSSTREAM, "com/sifir/tor/TcpSocksStream")
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_TCPSOCKSSTREAM_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_TCPSOCKSSTREAM,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<TcpSocksStream> = Box::new(this);
        let this: *mut TcpSocksStream = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut TcpSocksStream =
            unsafe { jlong_to_pointer::<TcpSocksStream>(x).as_mut().unwrap() };
        let x: Box<TcpSocksStream> = unsafe { Box::from_raw(x) };
        let x: TcpSocksStream = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut TcpSocksStream =
            unsafe { jlong_to_pointer::<TcpSocksStream>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TcpSocksStream_init(
    env: *mut JNIEnv,
    _: jclass,
    target: jstring,
    socks_proxy: jstring,
    timeout_ms: jlong,
) -> jlong {
    let mut target: String = from_jstring_std_string(target, env);
    let mut socks_proxy: String = from_jstring_std_string(socks_proxy, env);
    let mut timeout_ms: u64 = <u64 as ::std::convert::TryFrom<jlong>>::try_from(timeout_ms)
        .expect("invalid jlong, in jlong => u64 conversion");
    let this: Result<TcpSocksStream, String> = {
        TcpSocksStream::new_timeout(target, socks_proxy, timeout_ms)
            .map_err(|e| format!("{:#?}", e))
    };
    let mut this: jlong = match this {
        Ok(x) => {
            let ret: jlong = <TcpSocksStream>::box_object(x);
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <jlong>::jni_invalid_value();
        }
    };
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TcpSocksStream_do_1on_1data(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    cb: jobject,
) -> () {
    let mut cb: Box<dyn DataObserver> = <Box<dyn DataObserver>>::swig_from(cb, env);
    let this: &mut TcpSocksStream =
        unsafe { jlong_to_pointer::<TcpSocksStream>(this).as_mut().unwrap() };
    let mut ret: Result<(), String> = {
        this.set_data_handler(Observer { cb }).unwrap();
        this.read_line_async().map_err(|e| format!("{:#?}", e))
    };
    let mut ret: () = match ret {
        Ok(x) => {
            let mut ret = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <()>::jni_invalid_value();
        }
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TcpSocksStream_do_1send_1data(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
    msg: jstring,
    timeout: jlong,
) -> () {
    let mut msg: String = from_jstring_std_string(msg, env);
    let mut timeout: u64 = <u64 as ::std::convert::TryFrom<jlong>>::try_from(timeout)
        .expect("invalid jlong, in jlong => u64 conversion");
    let this: &mut TcpSocksStream =
        unsafe { jlong_to_pointer::<TcpSocksStream>(this).as_mut().unwrap() };
    let mut ret: Result<(), String> = {
        this.send_data(msg, Some(Duration::new(timeout, 0)))
            .map_err(|e| format!("{:#?}", e))
    };
    let mut ret: () = match ret {
        Ok(x) => {
            let mut ret = x;
            ret
        }
        Err(msg) => {
            jni_throw_exception(env, &msg);
            return <()>::jni_invalid_value();
        }
    };
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_sifir_tor_TcpSocksStream_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut TcpSocksStream =
        unsafe { jlong_to_pointer::<TcpSocksStream>(this).as_mut().unwrap() };
    let this: Box<TcpSocksStream> = unsafe { Box::from_raw(this) };
    drop(this);
}
static mut JAVA_LANG_EXCEPTION: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_STRING: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE_BYTE_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_OWNEDTORSERVICE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_OWNEDTORSERVICE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_HIDDENSERVICEHANDLER: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_HIDDENSERVICEHANDLER_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TORSERVICEPARAM: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TORSERVICEPARAM_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT_FLOAT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER_INT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG_LONG_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TORHIDDENSERVICE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TORHIDDENSERVICE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT_SHORT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TCPSOCKSSTREAM: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_TCPSOCKSSTREAM_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    java_vm: *mut JavaVM,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    log::debug!("JNI_OnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Exception"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_EXCEPTION = class;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalDouble"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_DOUBLE = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(D)Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "of",
                " sig ",
                "(D)Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/String"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/String")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/String")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_STRING = class;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalLong"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_LONG = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(J)Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "of",
                " sig ",
                "(J)Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Byte"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_BYTE = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("byteValue"), swig_c_str!("()B"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Byte",
                " method ",
                "byteValue",
                " sig ",
                "()B",
                " failed"
            )
        );
        JAVA_LANG_BYTE_BYTE_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/sifir/tor/OwnedTorService"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/OwnedTorService")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/OwnedTorService")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_OWNEDTORSERVICE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/sifir/tor/OwnedTorService",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_OWNEDTORSERVICE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/sifir/tor/HiddenServiceHandler"));
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/sifir/tor/HiddenServiceHandler"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/sifir/tor/HiddenServiceHandler"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_HIDDENSERVICEHANDLER = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/sifir/tor/HiddenServiceHandler",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_HIDDENSERVICEHANDLER_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/sifir/tor/TorServiceParam"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TorServiceParam")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TorServiceParam")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TORSERVICEPARAM = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/sifir/tor/TorServiceParam",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TORSERVICEPARAM_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Float"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_FLOAT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("floatValue"), swig_c_str!("()F"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Float",
                " method ",
                "floatValue",
                " sig ",
                "()F",
                " failed"
            )
        );
        JAVA_LANG_FLOAT_FLOAT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalInt"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_INT = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(I)Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "of",
                " sig ",
                "(I)Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Integer"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_INTEGER = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("intValue"), swig_c_str!("()I"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Integer",
                " method ",
                "intValue",
                " sig ",
                "()I",
                " failed"
            )
        );
        JAVA_LANG_INTEGER_INT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Long"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_LONG = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("longValue"), swig_c_str!("()J"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Long",
                " method ",
                "longValue",
                " sig ",
                "()J",
                " failed"
            )
        );
        JAVA_LANG_LONG_LONG_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/sifir/tor/TorHiddenService"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TorHiddenService")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TorHiddenService")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TORHIDDENSERVICE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/sifir/tor/TorHiddenService",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TORHIDDENSERVICE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Double"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_DOUBLE = class;
        let method_id: jmethodID = (**env).GetMethodID.unwrap()(
            env,
            class,
            swig_c_str!("doubleValue"),
            swig_c_str!("()D"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Double",
                " method ",
                "doubleValue",
                " sig ",
                "()D",
                " failed"
            )
        );
        JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Short"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_SHORT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("shortValue"), swig_c_str!("()S"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Short",
                " method ",
                "shortValue",
                " sig ",
                "()S",
                " failed"
            )
        );
        JAVA_LANG_SHORT_SHORT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/sifir/tor/TcpSocksStream"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TcpSocksStream")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "com/sifir/tor/TcpSocksStream")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_TCPSOCKSSTREAM = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/sifir/tor/TcpSocksStream",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_TCPSOCKSSTREAM_MNATIVEOBJ_FIELD = field_id;
    }
    SWIG_JNI_VERSION
}
#[no_mangle]
pub extern "system" fn JNI_OnUnload(java_vm: *mut JavaVM, _reserved: *mut ::std::os::raw::c_void) {
    log::debug!("JNI_OnUnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_EXCEPTION);
        JAVA_LANG_EXCEPTION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_DOUBLE);
        JAVA_UTIL_OPTIONAL_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_STRING);
        JAVA_LANG_STRING = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_LONG);
        JAVA_UTIL_OPTIONAL_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_BYTE);
        JAVA_LANG_BYTE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_OWNEDTORSERVICE);
        FOREIGN_CLASS_OWNEDTORSERVICE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_HIDDENSERVICEHANDLER);
        FOREIGN_CLASS_HIDDENSERVICEHANDLER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TORSERVICEPARAM);
        FOREIGN_CLASS_TORSERVICEPARAM = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_FLOAT);
        JAVA_LANG_FLOAT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_INT);
        JAVA_UTIL_OPTIONAL_INT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_INTEGER);
        JAVA_LANG_INTEGER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_LONG);
        JAVA_LANG_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TORHIDDENSERVICE);
        FOREIGN_CLASS_TORHIDDENSERVICE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_DOUBLE);
        JAVA_LANG_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_SHORT);
        JAVA_LANG_SHORT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_TCPSOCKSSTREAM);
        FOREIGN_CLASS_TCPSOCKSSTREAM = ::std::ptr::null_mut()
    }
}
