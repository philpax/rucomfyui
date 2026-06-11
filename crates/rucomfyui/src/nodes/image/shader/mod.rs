//!`shader` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
/// Output types for nodes.
pub mod out {
    ///Output for [`GLSLShader`](super::GLSLShader).
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct GLSLShaderOutput {
        ///Available via layout(location = 0) out vec4 fragColor0 in the shader code
        pub image_0: crate::nodes::types::ImageOut,
        ///Available via layout(location = 1) out vec4 fragColor1 in the shader code
        pub image_1: crate::nodes::types::ImageOut,
        ///Available via layout(location = 2) out vec4 fragColor2 in the shader code
        pub image_2: crate::nodes::types::ImageOut,
        ///Available via layout(location = 3) out vec4 fragColor3 in the shader code
        pub image_3: crate::nodes::types::ImageOut,
    }
}
///**GLSL Shader**: Apply GLSL ES fragment shaders to images. u_resolution (vec2) is always available.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GLSLShader<FragmentShaderParam: crate::nodes::types::String> {
    /**GLSL fragment shader source code (GLSL ES 3.00 / WebGL 2.0 compatible)

**Metadata**:
  - Multiline: true
  - Default: #version 300 es
precision highp float;

uniform sampler2D u_image0;
uniform vec2 u_resolution;

in vec2 v_texCoord;
layout(location = 0) out vec4 fragColor0;

void main() {
    fragColor0 = texture(u_image0, v_texCoord);
}

*/
    pub fragment_shader: FragmentShaderParam,
}
impl<FragmentShaderParam: crate::nodes::types::String> GLSLShader<FragmentShaderParam> {
    /// Create a new node.
    pub fn new(fragment_shader: FragmentShaderParam) -> Self {
        Self { fragment_shader }
    }
}
impl<FragmentShaderParam: crate::nodes::types::String> crate::nodes::TypedNode
for GLSLShader<FragmentShaderParam> {
    type Output = out::GLSLShaderOutput;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output {
            image_0: crate::nodes::types::ImageOut::from_dynamic(node_id, 0u32),
            image_1: crate::nodes::types::ImageOut::from_dynamic(node_id, 1u32),
            image_2: crate::nodes::types::ImageOut::from_dynamic(node_id, 2u32),
            image_3: crate::nodes::types::ImageOut::from_dynamic(node_id, 3u32),
        }
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output
            .insert("fragment_shader".to_string(), self.fragment_shader.clone().into());
        output
    }
    const NAME: &'static str = "GLSLShader";
    const DISPLAY_NAME: &'static str = "GLSL Shader";
    const DESCRIPTION: &'static str = "Apply GLSL ES fragment shaders to images. u_resolution (vec2) is always available.";
    const CATEGORY: &'static str = "image/shader";
}
