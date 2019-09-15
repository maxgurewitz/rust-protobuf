use std::any::Any;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
#[cfg(feature = "bytes")]
use bytes::Bytes;

use super::*;

use crate::reflect::transmute_eq::transmute_eq;

/// Type implemented by all protobuf elementary types
/// (ints, floats, bool, string, bytes, enums, messages).
pub trait ProtobufValue: Any + 'static + Send + Sync {
    /// As ref
    fn as_ref(&self) -> ReflectValueRef;

    /// Is value non-zero?
    fn is_non_zero(&self) -> bool {
        self.as_ref().is_non_zero()
    }

    /// Return `ProtobufValueRef` if self is `Copy`.
    ///
    /// # Panics
    ///
    /// if `Self` is not `Copy`.
    fn as_ref_copy(&self) -> ReflectValueRef<'static>
//where Self : Copy // TODO
    {
        match self.as_ref() {
            ReflectValueRef::Bool(v) => ReflectValueRef::Bool(v),
            ReflectValueRef::U32(v) => ReflectValueRef::U32(v),
            ReflectValueRef::U64(v) => ReflectValueRef::U64(v),
            ReflectValueRef::I32(v) => ReflectValueRef::I32(v),
            ReflectValueRef::I64(v) => ReflectValueRef::I64(v),
            ReflectValueRef::F32(v) => ReflectValueRef::F32(v),
            ReflectValueRef::F64(v) => ReflectValueRef::F64(v),
            ReflectValueRef::Enum(v) => ReflectValueRef::Enum(v),
            ReflectValueRef::String(..)
            | ReflectValueRef::Bytes(..)
            | ReflectValueRef::Message(..) => unreachable!(),
        }
    }
}

impl ProtobufValue for u32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::U32(*self)
    }
}

impl ProtobufValue for u64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::U64(*self)
    }
}

impl ProtobufValue for i32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::I32(*self)
    }
}

impl ProtobufValue for i64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::I64(*self)
    }
}

impl ProtobufValue for f32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::F32(*self)
    }
}

impl ProtobufValue for f64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::F64(*self)
    }
}

impl ProtobufValue for bool {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bool(*self)
    }
}

impl ProtobufValue for String {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(*&self)
    }
}

impl ProtobufValue for str {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(self)
    }
}

impl ProtobufValue for Vec<u8> {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bytes(*&self)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bytes(&*self)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(&*self)
    }
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Enum(self.descriptor())
    }
}

impl<M : Message> ProtobufValue for M {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Message(self)
    }
}
*/

/// A reference to a value
#[derive(Debug, Clone)]
pub enum ReflectValueRef<'a> {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(&'a str),
    /// `bytes`
    Bytes(&'a [u8]),
    /// `enum`
    // TODO: change to (i32, EnumDescriptor)
    Enum(&'static EnumValueDescriptor),
    /// `message`
    Message(&'a dyn Message),
}

impl<'a> ReflectValueRef<'a> {
    /// Value is "non-zero"?
    #[doc(hidden)]
    pub fn is_non_zero(&self) -> bool {
        match *self {
            ReflectValueRef::U32(v) => v != 0,
            ReflectValueRef::U64(v) => v != 0,
            ReflectValueRef::I32(v) => v != 0,
            ReflectValueRef::I64(v) => v != 0,
            ReflectValueRef::F32(v) => v != 0.,
            ReflectValueRef::F64(v) => v != 0.,
            ReflectValueRef::Bool(v) => v,
            ReflectValueRef::String(v) => !v.is_empty(),
            ReflectValueRef::Bytes(v) => !v.is_empty(),
            ReflectValueRef::Enum(v) => v.value() != 0,
            ReflectValueRef::Message(_) => true,
        }
    }

    /// Take `i32` value.
    pub fn to_i32(&self) -> Option<i32> {
        match *self {
            ReflectValueRef::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `i64` value.
    pub fn to_i64(&self) -> Option<i64> {
        match *self {
            ReflectValueRef::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u32` value.
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            ReflectValueRef::U32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u64` value.
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            ReflectValueRef::U64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f32` value.
    pub fn to_f32(&self) -> Option<f32> {
        match *self {
            ReflectValueRef::F32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f64` value.
    pub fn to_f64(&self) -> Option<f64> {
        match *self {
            ReflectValueRef::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `bool` value.
    pub fn to_bool(&self) -> Option<bool> {
        match *self {
            ReflectValueRef::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Take `str` value.
    pub fn to_str(&self) -> Option<&str> {
        match *self {
            ReflectValueRef::String(v) => Some(v),
            _ => None,
        }
    }

    /// Take `[u8]` value.
    pub fn to_bytes(&self) -> Option<&[u8]> {
        match *self {
            ReflectValueRef::Bytes(v) => Some(v),
            _ => None,
        }
    }

    /// Take message value.
    pub fn to_message(&self) -> Option<&'a dyn Message> {
        match *self {
            ReflectValueRef::Message(m) => Some(m),
            _ => None,
        }
    }

    /// Clone to a box
    pub(crate) fn to_box(&self) -> ReflectValueBox {
        match *self {
            ReflectValueRef::U32(v) => ReflectValueBox::U32(v),
            ReflectValueRef::U64(v) => ReflectValueBox::U64(v),
            ReflectValueRef::I32(v) => ReflectValueBox::I32(v),
            ReflectValueRef::I64(v) => ReflectValueBox::I64(v),
            ReflectValueRef::F32(v) => ReflectValueBox::F32(v),
            ReflectValueRef::F64(v) => ReflectValueBox::F64(v),
            ReflectValueRef::Bool(v) => ReflectValueBox::Bool(v),
            ReflectValueRef::String(v) => ReflectValueBox::String(v.to_owned()),
            ReflectValueRef::Bytes(v) => ReflectValueBox::Bytes(v.to_owned()),
            ReflectValueRef::Enum(v) => ReflectValueBox::Enum(v),
            ReflectValueRef::Message(v) => ReflectValueBox::Message(v.descriptor().clone(v)),
        }
    }

    /// Convert a value to arbitrary value.
    pub fn downcast_clone<V: ProtobufValue>(&self) -> Result<V, Self> {
        self.to_box().downcast().map_err(|_| self.clone())
    }
}

#[doc(hidden)]
pub enum ReflectValueMut<'a> {
    Message(&'a mut dyn Message),
}

/// Owner value of any elementary type
#[derive(Debug, Clone)]
pub enum ReflectValueBox {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(String),
    /// `bytes`
    Bytes(Vec<u8>),
    /// `enum`
    // TODO: change to (i32, EnumDescriptor)
    Enum(&'static EnumValueDescriptor),
    /// `message`
    Message(Box<dyn Message>),
}

impl From<u32> for ReflectValueBox {
    fn from(v: u32) -> Self {
        ReflectValueBox::U32(v)
    }
}

impl From<u64> for ReflectValueBox {
    fn from(v: u64) -> Self {
        ReflectValueBox::U64(v)
    }
}

impl From<i32> for ReflectValueBox {
    fn from(v: i32) -> Self {
        ReflectValueBox::I32(v)
    }
}

impl From<i64> for ReflectValueBox {
    fn from(v: i64) -> Self {
        ReflectValueBox::I64(v)
    }
}

impl From<f32> for ReflectValueBox {
    fn from(v: f32) -> Self {
        ReflectValueBox::F32(v)
    }
}

impl From<f64> for ReflectValueBox {
    fn from(v: f64) -> Self {
        ReflectValueBox::F64(v)
    }
}

impl From<bool> for ReflectValueBox {
    fn from(v: bool) -> Self {
        ReflectValueBox::Bool(v)
    }
}

impl From<String> for ReflectValueBox {
    fn from(v: String) -> Self {
        ReflectValueBox::String(v)
    }
}

impl From<Vec<u8>> for ReflectValueBox {
    fn from(v: Vec<u8>) -> Self {
        ReflectValueBox::Bytes(v)
    }
}

impl From<&'static EnumValueDescriptor> for ReflectValueBox {
    fn from(v: &'static EnumValueDescriptor) -> Self {
        ReflectValueBox::Enum(v)
    }
}

impl From<Box<dyn Message>> for ReflectValueBox {
    fn from(v: Box<dyn Message>) -> Self {
        ReflectValueBox::Message(v)
    }
}

fn _assert_value_box_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<ReflectValueBox>();
}

#[cfg(not(feature = "bytes"))]
type VecU8OrBytes = Vec<u8>;
#[cfg(feature = "bytes")]
type VecU8OrBytes = Vec<u8>;
#[cfg(not(feature = "bytes"))]
type StringOrChars = String;
#[cfg(feature = "bytes")]
type StringOrChars = Chars;

impl ReflectValueBox {
    /// Downcast to real typed value.
    ///
    /// For `enum` `V` can be either `V: ProtobufEnum` or `V: ProtobufEnumOrUnknown<E>`.
    pub fn downcast<V: ProtobufValue>(self) -> Result<V, Self> {
        match self {
            ReflectValueBox::U32(v) => transmute_eq(v).map_err(ReflectValueBox::U32),
            ReflectValueBox::U64(v) => transmute_eq(v).map_err(ReflectValueBox::U64),
            ReflectValueBox::I32(v) => transmute_eq(v).map_err(ReflectValueBox::I32),
            ReflectValueBox::I64(v) => transmute_eq(v).map_err(ReflectValueBox::I64),
            ReflectValueBox::F32(v) => transmute_eq(v).map_err(ReflectValueBox::F32),
            ReflectValueBox::F64(v) => transmute_eq(v).map_err(ReflectValueBox::F64),
            ReflectValueBox::Bool(v) => transmute_eq(v).map_err(ReflectValueBox::Bool),
            ReflectValueBox::String(v) => transmute_eq::<String, _>(v)
                .or_else(|v: String| transmute_eq::<StringOrChars, _>(v.into()))
                .map_err(|v: StringOrChars| ReflectValueBox::String(v.into())),
            ReflectValueBox::Bytes(v) => transmute_eq::<Vec<u8>, _>(v)
                .or_else(|v: Vec<u8>| transmute_eq::<VecU8OrBytes, _>(v.into()))
                .map_err(|v: VecU8OrBytes| ReflectValueBox::Bytes(v.into())),
            ReflectValueBox::Enum(v) => v
                .enum_descriptor()
                .cast_to_protobuf_enum(v.value())
                .ok_or(ReflectValueBox::Enum(v)),
            ReflectValueBox::Message(m) => m
                .downcast_box::<V>()
                .map(|m| *m)
                .map_err(ReflectValueBox::Message),
        }
    }
}
