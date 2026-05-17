#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    Void,
    Never,
    Function {
        params: Vec<Type>,
        ret: Box<Type>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        variants: Vec<(String, Option<Type>)>,
    },
    Array(Box<Type>),
    Ref(Box<Type>),
    MutRef(Box<Type>),
    Tuple(Vec<Type>),
    Generic(&'static str),
    Unknown,
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::I32 | Type::I64 | Type::U32 | Type::U64 | Type::F32 | Type::F64)
    }

    pub fn name(&self) -> &str {
        match self {
            Type::I32 => "i32",
            Type::I64 => "i64",
            Type::U32 => "u32",
            Type::U64 => "u64",
            Type::F32 => "f32",
            Type::F64 => "f64",
            Type::Bool => "bool",
            Type::String => "string",
            Type::Void => "void",
            Type::Never => "never",
            _ => "?",
        }
    }
}
