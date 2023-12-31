/// An offset on a procedure's captures vector.
#[derive(Clone, Copy, Debug)]
pub struct CaptureOffset(pub usize);

/// An offset on the stack.
#[derive(Clone, Copy, Debug)]
pub struct StackOffset(pub usize);

impl std::ops::Add for StackOffset {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        StackOffset(self.0 + other.0)
    }
}

impl std::ops::AddAssign for StackOffset {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl std::ops::Sub for StackOffset {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        StackOffset(self.0 - other.0)
    }
}

impl std::ops::SubAssign for StackOffset {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Capture {
    Local(StackOffset),
    Capture(CaptureOffset),
}
