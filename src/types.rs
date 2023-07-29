use std::fmt;
use std::rc::Rc;

pub struct LetType {
     let_type: Rc<TypeTag>,
}

impl LetType {
    pub fn new_int() -> Self {
        let let_type = Rc::new(TypeTag::Int);
        Self { let_type }
    }

    pub fn new_bool() -> Self {
        let let_type = Rc::new(TypeTag::Bool);
        Self { let_type }
    }

    pub fn new_proc(var_type: LetType, result_type: LetType) -> Self {
        let let_type = Rc::new(TypeTag::Proc(var_type, result_type));
        Self { let_type }
    }

    pub fn as_proc(&self) -> Option<(&LetType, &LetType)> {
    }

    pub fn is_int(&self) -> bool {
        match self.let_type.as_ref() {
            TypeTag::Int => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self.let_type.as_ref() {
            TypeTag::Bool => true,
            _ => false,
        }
    }
}

impl PartialEq for LetType {
    fn eq(&self, other: &Self) -> bool {
        self.let_type.as_ref() == other.let_type.as_ref()
    }
}

impl Clone for LetType {
    fn clone(&self) -> Self {
        let let_type = Rc::clone(&self.let_type);
        Self { let_type }
    }
}

impl fmt::Display for LetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.let_type)
    }
}

enum TypeTag {
    Int,
    Bool,
    Proc(LetType, LetType),
}

impl PartialEq for TypeTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeTag::Int, TypeTag::Int) => true,
            (TypeTag::Bool, TypeTag::Bool) => true,
            (TypeTag::Proc(v1, r1), TypeTag::Proc(v2, r2)) => v1 == v2 && r1 == r2,
            _ => false,
        }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Proc(var, result) => write!(f, "({} -> {})", var, result),
        }
    }
}

