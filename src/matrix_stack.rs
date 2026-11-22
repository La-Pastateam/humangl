use crate::utils::math::Mat4;

pub struct MatrixStack {
    stack: Vec<Mat4>,
}

impl MatrixStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Mat4::identity()],
        }
    }

    pub fn top(&self) -> &Mat4 {
        self.stack.last().expect("Stack is empty")
    }

    pub fn push(&mut self, matrix: Mat4) {
        let top = *self.top();
        self.stack.push(top * matrix);
    }

    pub fn pop(&mut self) -> Mat4 {
        if self.stack.len() > 1 {
            self.stack.pop().unwrap()
        } else {
            *self.top()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() == 1 && self.stack[0].is_identity()
    }
}
