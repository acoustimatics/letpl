//! Types the represent types in letpl.

use std::fmt;
use std::rc::Rc;

enum TypeTag {
    Int,
    Bool,
    Proc { t_param: Type, t_result: Type },
}

impl TypeTag {
    pub fn is_int(&self) -> bool {
        match self {
            TypeTag::Int => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            TypeTag::Bool => true,
            _ => false,
        }
    }

    pub fn as_proc(&self) -> Option<(&Type, &Type)> {
        match self {
            TypeTag::Proc { t_param, t_result } => Some((t_param, t_result)),
            _ => None,
        }
    }
}

impl PartialEq for TypeTag {
    fn eq(&self, other: &Self) -> bool {
        if self.is_int() && other.is_int() {
            return true;
        }

        if self.is_bool() && other.is_bool() {
            return true;
        }

        if let Some(left_proc) = self.as_proc() {
            if let Some(right_proc) = other.as_proc() {
                return left_proc == right_proc;
            }
        }

        return false;
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Proc { t_param, t_result } => write!(f, "({t_param} -> {t_result})"),
        }
    }
}

pub struct Type {
    tag: Rc<TypeTag>,
}

impl Type {
    pub fn new_int() -> Self {
        let tag = Rc::new(TypeTag::Int);
        Self { tag }
    }

    pub fn new_bool() -> Self {
        let tag = Rc::new(TypeTag::Bool);
        Self { tag }
    }

    pub fn new_proc(t_param: Type, t_result: Type) -> Self {
        let tag = Rc::new(TypeTag::Proc { t_param, t_result });
        Self { tag }
    }

    pub fn is_int(&self) -> bool {
        self.tag.is_int()
    }

    pub fn is_bool(&self) -> bool {
        self.tag.is_bool()
    }

    pub fn as_proc(&self) -> Option<(&Type, &Type)> {
        self.tag.as_proc()
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.tag.as_ref() == other.tag.as_ref()
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        let tag = Rc::clone(&self.tag);
        Self { tag }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}
