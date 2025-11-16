//!`Rodin` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Rodin 3D Generate - Detail Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Detail<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Detail<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Detail<ImagesParam, SeedParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Detail";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Detail Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "api node/3d/Rodin";
}
///**Rodin 3D Generate - Gen-2 Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Gen2<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub ta_pose: TaPoseParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Gen2<ImagesParam, TaPoseParam, SeedParam> {
    /// Create a new node.
    pub fn new(
        images: ImagesParam,
        ta_pose: TaPoseParam,
        seed: Option<SeedParam>,
    ) -> Self {
        Self { images, ta_pose, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    TaPoseParam: crate::nodes::types::Boolean,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Gen2<ImagesParam, TaPoseParam, SeedParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        output.insert("TAPose".to_string(), self.ta_pose.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Gen2";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Gen-2 Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "api node/3d/Rodin";
}
///**Rodin 3D Generate - Regular Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Regular<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Regular<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Regular<ImagesParam, SeedParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Regular";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Regular Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "api node/3d/Rodin";
}
///**Rodin 3D Generate - Sketch Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Sketch<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Sketch<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Sketch<ImagesParam, SeedParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Sketch";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Sketch Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "api node/3d/Rodin";
}
///**Rodin 3D Generate - Smooth Generate**: Generate 3D Assets using Rodin API
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rodin3D_Smooth<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    ///No documentation.
    pub images: ImagesParam,
    /**No documentation.

**Metadata**:
  - Default: 0
  - Display: number
  - Max: 65535
  - Min: 0
*/
    pub seed: Option<SeedParam>,
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> Rodin3D_Smooth<ImagesParam, SeedParam> {
    /// Create a new node.
    pub fn new(images: ImagesParam, seed: Option<SeedParam>) -> Self {
        Self { images, seed }
    }
}
impl<
    ImagesParam: crate::nodes::types::Image,
    SeedParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for Rodin3D_Smooth<ImagesParam, SeedParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("Images".to_string(), self.images.clone().into());
        if let Some(v) = &self.seed {
            output.insert("Seed".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "Rodin3D_Smooth";
    const DISPLAY_NAME: &'static str = "Rodin 3D Generate - Smooth Generate";
    const DESCRIPTION: &'static str = "Generate 3D Assets using Rodin API";
    const CATEGORY: &'static str = "api node/3d/Rodin";
}
