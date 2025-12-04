//!`text` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Add Text Prefix**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AddTextPrefix<
    TextsParam: crate::nodes::types::String,
    PrefixParam: crate::nodes::types::String,
> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Prefix to add.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub prefix: PrefixParam,
}
impl<
    TextsParam: crate::nodes::types::String,
    PrefixParam: crate::nodes::types::String,
> AddTextPrefix<TextsParam, PrefixParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam, prefix: PrefixParam) -> Self {
        Self { texts, prefix }
    }
}
impl<
    TextsParam: crate::nodes::types::String,
    PrefixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for AddTextPrefix<TextsParam, PrefixParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("prefix".to_string(), self.prefix.clone().into());
        output
    }
    const NAME: &'static str = "AddTextPrefix";
    const DISPLAY_NAME: &'static str = "Add Text Prefix";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Add Text Suffix**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AddTextSuffix<
    TextsParam: crate::nodes::types::String,
    SuffixParam: crate::nodes::types::String,
> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Suffix to add.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub suffix: SuffixParam,
}
impl<
    TextsParam: crate::nodes::types::String,
    SuffixParam: crate::nodes::types::String,
> AddTextSuffix<TextsParam, SuffixParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam, suffix: SuffixParam) -> Self {
        Self { texts, suffix }
    }
}
impl<
    TextsParam: crate::nodes::types::String,
    SuffixParam: crate::nodes::types::String,
> crate::nodes::TypedNode for AddTextSuffix<TextsParam, SuffixParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("suffix".to_string(), self.suffix.clone().into());
        output
    }
    const NAME: &'static str = "AddTextSuffix";
    const DISPLAY_NAME: &'static str = "Add Text Suffix";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Merge Text Lists**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MergeTextLists<TextsParam: crate::nodes::types::String> {
    /**List of texts to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
}
impl<TextsParam: crate::nodes::types::String> MergeTextLists<TextsParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam) -> Self {
        Self { texts }
    }
}
impl<TextsParam: crate::nodes::types::String> crate::nodes::TypedNode
for MergeTextLists<TextsParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output
    }
    const NAME: &'static str = "MergeTextLists";
    const DISPLAY_NAME: &'static str = "Merge Text Lists";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Replace Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ReplaceText<
    TextsParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Text to find.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub find: FindParam,
    /**Text to replace with.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub replace: ReplaceParam,
}
impl<
    TextsParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> ReplaceText<TextsParam, FindParam, ReplaceParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam, find: FindParam, replace: ReplaceParam) -> Self {
        Self { texts, find, replace }
    }
}
impl<
    TextsParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> crate::nodes::TypedNode for ReplaceText<TextsParam, FindParam, ReplaceParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("find".to_string(), self.find.clone().into());
        output.insert("replace".to_string(), self.replace.clone().into());
        output
    }
    const NAME: &'static str = "ReplaceText";
    const DISPLAY_NAME: &'static str = "Replace Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Strip Whitespace**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StripWhitespace<TextsParam: crate::nodes::types::String> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
}
impl<TextsParam: crate::nodes::types::String> StripWhitespace<TextsParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam) -> Self {
        Self { texts }
    }
}
impl<TextsParam: crate::nodes::types::String> crate::nodes::TypedNode
for StripWhitespace<TextsParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output
    }
    const NAME: &'static str = "StripWhitespace";
    const DISPLAY_NAME: &'static str = "Strip Whitespace";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Text to Lowercase**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TextToLowercase<TextsParam: crate::nodes::types::String> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
}
impl<TextsParam: crate::nodes::types::String> TextToLowercase<TextsParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam) -> Self {
        Self { texts }
    }
}
impl<TextsParam: crate::nodes::types::String> crate::nodes::TypedNode
for TextToLowercase<TextsParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output
    }
    const NAME: &'static str = "TextToLowercase";
    const DISPLAY_NAME: &'static str = "Text to Lowercase";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Text to Uppercase**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TextToUppercase<TextsParam: crate::nodes::types::String> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
}
impl<TextsParam: crate::nodes::types::String> TextToUppercase<TextsParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam) -> Self {
        Self { texts }
    }
}
impl<TextsParam: crate::nodes::types::String> crate::nodes::TypedNode
for TextToUppercase<TextsParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output
    }
    const NAME: &'static str = "TextToUppercase";
    const DISPLAY_NAME: &'static str = "Text to Uppercase";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
///**Truncate Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TruncateText<
    TextsParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
> {
    /**Text to process.

**Metadata**:
  - Multiline: false
*/
    pub texts: TextsParam,
    /**Maximum text length.

**Metadata**:
  - Default: 77
  - Max: 10000
  - Min: 1
*/
    pub max_length: MaxLengthParam,
}
impl<
    TextsParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
> TruncateText<TextsParam, MaxLengthParam> {
    /// Create a new node.
    pub fn new(texts: TextsParam, max_length: MaxLengthParam) -> Self {
        Self { texts, max_length }
    }
}
impl<
    TextsParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for TruncateText<TextsParam, MaxLengthParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("texts".to_string(), self.texts.clone().into());
        output.insert("max_length".to_string(), self.max_length.clone().into());
        output
    }
    const NAME: &'static str = "TruncateText";
    const DISPLAY_NAME: &'static str = "Truncate Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "dataset/text";
}
