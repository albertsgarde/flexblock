use std::fmt::Display;

use crate::utils::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub enum BufferTarget {
    //TODO: RENAME TO VERTEXBUFFERTARGET
    GuiBuffer,
    NormalBuffer(usize),
}

impl BufferTarget {
    pub fn get_target_id(&self) -> usize {
        match &self {
            BufferTarget::GuiBuffer => 0,
            BufferTarget::NormalBuffer(i) => i + 1,
        }
    }
}

impl Display for BufferTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BufferTarget::GuiBuffer => f.write_fmt(format_args!("BufferTarget::GuiBuffer")),
            BufferTarget::NormalBuffer(i) => {
                f.write_fmt(format_args!("BufferTarget::NormalBuffer({})", i))
            }
        }
    }
}

///
/// Has size*components members.
pub struct ArrayBuffer<T: Vertex> {
    id: u32,
    size: usize,
    stride: usize,
    dummy: T,
}

///TODO: MAKE TYPE SAFE
impl<T: Vertex> ArrayBuffer<T> {
    pub unsafe fn new(dummy: T) -> Result<ArrayBuffer<T>, String> {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);

        Ok(ArrayBuffer {
            id: vbo,
            size: 0,
            stride: std::mem::size_of::<T>(),
            dummy,
        })
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    pub unsafe fn fill(&mut self, data: &[T]) {
        self.bind();
        //TODO: Maybe shouldn't be static draw
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (self.stride * data.len()) as isize,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        self.size = data.len();
    }

    pub fn get_stride(&self) -> usize {
        self.stride
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl<T: Vertex> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct ElementBuffer {
    id: u32,
    size: usize,
}

impl ElementBuffer {
    pub unsafe fn new() -> Result<ElementBuffer, String> {
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);

        Ok(ElementBuffer { id: ebo, size: 0 })
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
    }

    pub unsafe fn fill(&mut self, data: &[u32]) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (std::mem::size_of::<u32>() * data.len()) as isize,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        self.size = data.len();
    }
}
