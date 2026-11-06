use crate::utils::math::Mat4;

pub struct MatrixStack {
    stack: Vec<Mat4>,
}

impl MatrixStack {
    pub fn new() -> Self {
        Self { stack: vec![Mat4::identity()] }
    }

    pub fn get_top(&mut self) -> &mat4 {
        self.stack.last().expect("Stack is empty")
    }

    pub fn push(&mut self, matrix: mat4) {
        self.stack.push(*self.get_top() * matrix);
    }

    pub fn pop(&mut self) -> mat4 {
        if self.stack.len() > 1 {
            self.stack.pop().unwrap()
        } else {
            *self.get_top()
        }
    }
}
