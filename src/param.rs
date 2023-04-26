pub trait Param: Send {
    fn as_param(&self) -> ParamType;
}

pub enum ParamType {
    Float(f32),
    Str(&'static str),
}

impl Param for f32 {
    fn as_param(&self) -> ParamType {
        ParamType::Float(*self)
    }
}

impl Param for &'static str {
    fn as_param(&self) -> ParamType {
        ParamType::Str(*self)
    }
}
