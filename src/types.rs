//! The types representing the parts of a flatbuffer schema

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Schema<'a> {
    pub(crate) includes: Vec<Include<'a>>, // zero or more
    pub(crate) body: Vec<Element<'a>>,     // zero or more
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Include<'a>(pub(crate) &'a str);

#[derive(Debug, Clone, PartialEq)]
pub enum Element<'a> {
    Namespace(Namespace<'a>),
    ProductType(ProductType<'a>), // type_decl in the grammar
    Enum(Enum<'a>),
    Root(Root<'a>),
    FileExtension(FileExtension<'a>),
    FileIdentifier(FileIdentifier<'a>),
    Attribute(Attribute<'a>),
    Rpc(Rpc<'a>),
    Object(Object<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Root<'a>(pub(crate) Ident<'a>);

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct FileExtension<'a>(pub(crate) &'a str);

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct FileIdentifier<'a>(pub(crate) &'a str);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Namespace<'a>(pub(crate) Vec<Ident<'a>>);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Attribute<'a>(pub(crate) Ident<'a>);

#[derive(Debug, Clone, PartialEq)]
pub struct ProductType<'a> {
    pub(crate) kind: ProductKind,
    pub(crate) name: Ident<'a>,
    pub(crate) fields: Vec<Field<'a>>, // one or more
    pub(crate) metadata: Option<Metadata<'a>>,
}

impl<'a> ProductType<'a> {
    pub fn new(
        kind: ProductKind,
        name: Ident<'a>,
        fields: Vec<Field<'a>>,
    ) -> Self {
        Self {
            kind,
            name,
            fields,
            metadata: None,
        }
    }

    pub fn with_metadata(self, metadata: Option<Metadata<'a>>) -> Self {
        Self {
            kind: self.kind,
            name: self.name,
            fields: self.fields,
            metadata,
        }
    }
}

pub fn table<'a>(name: Ident<'a>, fields: Vec<Field<'a>>) -> ProductType<'a> {
    ProductType::new(ProductKind::Table, name, fields)
}

pub fn struct_<'a>(name: Ident<'a>, fields: Vec<Field<'a>>) -> ProductType<'a> {
    ProductType::new(ProductKind::Struct, name, fields)
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum ProductKind {
    Table,
    Struct,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    pub(crate) kind: EnumKind<'a>,
    pub(crate) metadata: Option<Metadata<'a>>,
    pub(crate) values: Vec<EnumVal<'a>>, // zero or more?
    pub(crate) ident: Ident<'a>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum EnumKind<'a> {
    Enum(Type<'a>),
    Union,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) ty: Type<'a>,
    pub(crate) scalar: Option<Scalar>,
    pub(crate) metadata: Option<Metadata<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(name: Ident<'a>, ty: Type<'a>) -> Self {
        Self {
            name,
            ty,
            scalar: None,
            metadata: None,
        }
    }

    pub fn with_scalar(self, scalar: Scalar) -> Self {
        Self {
            name: self.name,
            ty: self.ty,
            scalar: Some(scalar),
            metadata: self.metadata,
        }
    }

    pub fn with_metadata(self, metadata: Metadata<'a>) -> Self {
        Self {
            name: self.name,
            ty: self.ty,
            scalar: self.scalar,
            metadata: Some(metadata),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rpc<'a> {
    name: Ident<'a>,
    methods: Vec<RpcMethod<'a>>, // one or more
}

impl<'a> Rpc<'a> {
    pub fn new(name: Ident<'a>, methods: Vec<RpcMethod<'a>>) -> Self {
        Self { name, methods }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RpcMethod<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) request_type: Ident<'a>,
    pub(crate) response_type: Ident<'a>,
    pub(crate) metadata: Option<Metadata<'a>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type<'a> {
    Bool,
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Long,
    ULong,
    Double,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float32,
    Float64,
    String,
    Array(Box<Type<'a>>),
    Ident(Ident<'a>),
}

pub type IntegerConstant = i64;
pub type FloatingConstant = f64;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EnumVal<'a> {
    name: Ident<'a>,
    value: Option<IntegerConstant>,
}

impl<'a> EnumVal<'a> {
    pub fn new(name: Ident<'a>, value: Option<IntegerConstant>) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata<'a>(pub(crate) HashMap<Ident<'a>, Option<SingleValue<'a>>>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scalar {
    Integer(IntegerConstant),
    Float(FloatingConstant),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object<'a>(pub(crate) HashMap<Ident<'a>, Value<'a>>);

#[derive(Debug, Clone, PartialEq)]
pub enum SingleValue<'a> {
    Scalar(Scalar),
    StringConstant(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    SingleValue(SingleValue<'a>),
    Object(Object<'a>),
    List(Vec<Value<'a>>),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Ident<'a>(pub(crate) &'a str);
