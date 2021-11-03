// OLD TT-MUNCHER STYLE UNIFORM MACROS
/* macro_rules! uniform_data {
    ( () -> {struct UniformData $( ($name:ident : $type:ty) )*}) => {
        #[derive(Debug)]
        pub struct UniformData {
            $(pub $name : Vec<($type, String)>,)*
        }
        impl UniformData {

            pub fn new() -> UniformData {
                UniformData {
                    $($name : Vec::new(),)*
                }
            }

            pub fn get_uniform_locations(&self) -> Vec<&String> {
                let mut res = Vec::new();

                $(for entry in &self.$name {
                    res.push(&entry.1);
                })*

                res
            }

            $(pub fn $name<T : Into<String>>(&mut self, value : $type, location : T) {
                self.$name.push((value, location.into()));
            })*

            pub fn get_uniforms<'a>(&'a self) -> Vec<Uniform<'a>> {
                let mut res = Vec::new();

                $(for entry in &self.$name {
                    res.push(Uniform { value : UniformValue::$name(&entry.0), location : &entry.1});
                })*

                res
            }

        }

        pub struct Uniform<'a> {
            pub value : UniformValue<'a>,
            pub location : &'a String
        }

        #[allow(non_camel_case_types)]
        pub enum UniformValue<'a> {
            $($name (&'a $type),)*
        }
    };
    (($name:ident ($type:ty), $($rest:tt)*) -> {$($output:tt)*}) => {
        uniform_data!{($($rest)*) -> {$($output)* ($name : $type)}}
    };

    (($name:ident ($type:ty)) -> {$($output:tt)*}) => {
        uniform_data!{() -> {$($output)* ($name : $type)}}
    };
}
macro_rules! create_uniform_data {
    ($($rest:tt)*) => {
        uniform_data!{($($rest)*) -> {struct UniformData}}
    };
}*/

macro_rules! create_uniform_data {
    ( $($name:ident ($type:ty), )* ) => {
        #[derive(Debug)]
        pub struct UniformData {
            $(pub $name : Vec<($type, String)>,)*
        }
        impl UniformData {
            /*pub fn new(
                $($name : Vec<($type, String)>,)*
            ) -> UniformData {
                UniformData {
                    $($name,)*
                }
            }*/

            pub fn new() -> UniformData {
                UniformData {
                    $($name : Vec::new(),)*
                }
            }

            pub fn get_uniform_locations(&self) -> Vec<&String> {
                let mut res = Vec::new();

                $(for entry in &self.$name {
                    res.push(&entry.1);
                })*

                res
            }

            $(pub fn $name<T : Into<String>>(&mut self, value : $type, location : T) {
                self.$name.push((value, location.into()));
            })*

            pub fn get_uniforms<'a>(&'a self) -> Vec<Uniform<'a>> {
                let mut res = Vec::new();

                $(for entry in &self.$name {
                    res.push(Uniform { value : UniformValue::$name(&entry.0), location : &entry.1});
                })*

                res
            }

        }

        pub struct Uniform<'a> {
            pub value : UniformValue<'a>,
            pub location : &'a String
        }

        #[allow(non_camel_case_types)]
        pub enum UniformValue<'a> {
            $($name (&'a $type),)*
        }
    }
}

//This generates the UniformData struct
create_uniform_data! {
    float(f32), int(i32), uint(u32),
    vec4(glm::Vec4), vec3 (glm::Vec3), vec2(glm::Vec2),
    ivec4(glm::IVec4), ivec3(glm::IVec3), ivec2(glm::IVec2),
    uvec4(glm::UVec4), uvec3(glm::UVec3), uvec2(glm::UVec2),
    mat4 (glm::Mat4), mat3(glm::Mat3), mat2(glm::Mat2),
    texture (String),

}

//TODO: Add mat 3, 2, and vec 3, 2, and f32, u32, i32, and texture
/*pub struct UniformData {
    pub mat4s: Vec<(glm::Mat4, String)>,
    pub vec3s: Vec<(glm::Vec3, String)>,
    /// The first string is the texture name, second is the uniform location name
    pub textures: Vec<(String, String)>,
}

//  create_uniform_data! {
//      mat4(Mat4), vec3(Vec3), texture(String), vec2(Vec2), vec4(Vec4)
//  }

impl UniformData {
    ///
    /// textures = (texture name, uniform location name)
    pub fn new(
        mat4s: Vec<(glm::Mat4, String)>,
        vec3s: Vec<(glm::Vec3, String)>,
        textures: Vec<(String, String)>,
    ) -> UniformData {
        UniformData {
            mat4s,
            vec3s,
            textures,
        }
    }

    pub fn new_empty() -> UniformData {
        UniformData {
            mat4s : Vec::new(),
            vec3s : Vec::new(),
            textures : Vec::new(),
        }
    }

    /// Gets the list of all uniforms referred to by this set of uniform data.
    pub fn get_uniform_locations(&self) -> Vec<&String> {
        let mut res = Vec::new();

        for entry in &self.mat4s {
            res.push(&entry.1);
        }
        for entry in &self.vec3s {
            res.push(&entry.1);
        }
        for entry in &self.textures {
            res.push(&entry.1);
        }

        res
    }

    pub fn mat4(&mut self, value : glm::Mat4, location : String) {
        self.mat4s.push((value, location));
    }
    pub fn vec3(&mut self, value : glm::Vec3, location : String) {
        self.vec3s.push((value, location));
    }
    pub fn texture(&mut self, value : String, location : String) {
        self.textures.push((value, location));
    }
}

impl fmt::Debug for UniformData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UniformData")
    }
}
*/
