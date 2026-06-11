//!`text` definitions/categories.
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
///**Add Text Prefix (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Add Text Prefix (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Add Text Suffix (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Add Text Suffix (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Convert Text Case**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CaseConverter<StringParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
}
impl<StringParam: crate::nodes::types::String> CaseConverter<StringParam> {
    /// Create a new node.
    pub fn new(string: StringParam) -> Self {
        Self { string }
    }
}
impl<StringParam: crate::nodes::types::String> crate::nodes::TypedNode
for CaseConverter<StringParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output
    }
    const NAME: &'static str = "CaseConverter";
    const DISPLAY_NAME: &'static str = "Convert Text Case";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Extract Text from JSON**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JsonExtractString<
    JsonStringParam: crate::nodes::types::String,
    KeyParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub json_string: JsonStringParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
*/
    pub key: KeyParam,
}
impl<
    JsonStringParam: crate::nodes::types::String,
    KeyParam: crate::nodes::types::String,
> JsonExtractString<JsonStringParam, KeyParam> {
    /// Create a new node.
    pub fn new(json_string: JsonStringParam, key: KeyParam) -> Self {
        Self { json_string, key }
    }
}
impl<
    JsonStringParam: crate::nodes::types::String,
    KeyParam: crate::nodes::types::String,
> crate::nodes::TypedNode for JsonExtractString<JsonStringParam, KeyParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("json_string".to_string(), self.json_string.clone().into());
        output.insert("key".to_string(), self.key.clone().into());
        output
    }
    const NAME: &'static str = "JsonExtractString";
    const DISPLAY_NAME: &'static str = "Extract Text from JSON";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Merge Text Lists (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Merge Text Lists (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Extract Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RegexExtract<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
    GroupIndexParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub regex_pattern: RegexPatternParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub case_insensitive: CaseInsensitiveParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub multiline: MultilineParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub dotall: DotallParam,
    /**No documentation.

**Metadata**:
  - Default: 1
  - Max: 100
  - Min: 0
*/
    pub group_index: GroupIndexParam,
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
    GroupIndexParam: crate::nodes::types::Int,
> RegexExtract<
    StringParam,
    RegexPatternParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
    GroupIndexParam,
> {
    /// Create a new node.
    pub fn new(
        string: StringParam,
        regex_pattern: RegexPatternParam,
        case_insensitive: CaseInsensitiveParam,
        multiline: MultilineParam,
        dotall: DotallParam,
        group_index: GroupIndexParam,
    ) -> Self {
        Self {
            string,
            regex_pattern,
            case_insensitive,
            multiline,
            dotall,
            group_index,
        }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
    GroupIndexParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RegexExtract<
    StringParam,
    RegexPatternParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
    GroupIndexParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("regex_pattern".to_string(), self.regex_pattern.clone().into());
        output
            .insert(
                "case_insensitive".to_string(),
                self.case_insensitive.clone().into(),
            );
        output.insert("multiline".to_string(), self.multiline.clone().into());
        output.insert("dotall".to_string(), self.dotall.clone().into());
        output.insert("group_index".to_string(), self.group_index.clone().into());
        output
    }
    const NAME: &'static str = "RegexExtract";
    const DISPLAY_NAME: &'static str = "Extract Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Match Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RegexMatch<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub regex_pattern: RegexPatternParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub case_insensitive: CaseInsensitiveParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub multiline: MultilineParam,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub dotall: DotallParam,
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
> RegexMatch<
    StringParam,
    RegexPatternParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
> {
    /// Create a new node.
    pub fn new(
        string: StringParam,
        regex_pattern: RegexPatternParam,
        case_insensitive: CaseInsensitiveParam,
        multiline: MultilineParam,
        dotall: DotallParam,
    ) -> Self {
        Self {
            string,
            regex_pattern,
            case_insensitive,
            multiline,
            dotall,
        }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for RegexMatch<
    StringParam,
    RegexPatternParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
> {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("regex_pattern".to_string(), self.regex_pattern.clone().into());
        output
            .insert(
                "case_insensitive".to_string(),
                self.case_insensitive.clone().into(),
            );
        output.insert("multiline".to_string(), self.multiline.clone().into());
        output.insert("dotall".to_string(), self.dotall.clone().into());
        output
    }
    const NAME: &'static str = "RegexMatch";
    const DISPLAY_NAME: &'static str = "Match Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Replace Text (Regex)**: Find and replace text using regex patterns.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RegexReplace<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    MultilineParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    DotallParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    CountParam: crate::nodes::types::Int = crate::nodes::types::IntOut,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub regex_pattern: RegexPatternParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub replace: ReplaceParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub case_insensitive: Option<CaseInsensitiveParam>,
    /**No documentation.

**Metadata**:
  - Default: false
*/
    pub multiline: Option<MultilineParam>,
    /**When enabled, the dot (.) character will match any character including newline characters. When disabled, dots won't match newlines.

**Metadata**:
  - Default: false
*/
    pub dotall: Option<DotallParam>,
    /**Maximum number of replacements to make. Set to 0 to replace all occurrences (default). Set to 1 to replace only the first match, 2 for the first two matches, etc.

**Metadata**:
  - Default: 0
  - Max: 100
  - Min: 0
*/
    pub count: Option<CountParam>,
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
    CountParam: crate::nodes::types::Int,
> RegexReplace<
    StringParam,
    RegexPatternParam,
    ReplaceParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
    CountParam,
> {
    /// Create a new node.
    pub fn new(
        string: StringParam,
        regex_pattern: RegexPatternParam,
        replace: ReplaceParam,
        case_insensitive: Option<CaseInsensitiveParam>,
        multiline: Option<MultilineParam>,
        dotall: Option<DotallParam>,
        count: Option<CountParam>,
    ) -> Self {
        Self {
            string,
            regex_pattern,
            replace,
            case_insensitive,
            multiline,
            dotall,
            count,
        }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    RegexPatternParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
    CaseInsensitiveParam: crate::nodes::types::Boolean,
    MultilineParam: crate::nodes::types::Boolean,
    DotallParam: crate::nodes::types::Boolean,
    CountParam: crate::nodes::types::Int,
> crate::nodes::TypedNode
for RegexReplace<
    StringParam,
    RegexPatternParam,
    ReplaceParam,
    CaseInsensitiveParam,
    MultilineParam,
    DotallParam,
    CountParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("regex_pattern".to_string(), self.regex_pattern.clone().into());
        output.insert("replace".to_string(), self.replace.clone().into());
        if let Some(v) = &self.case_insensitive {
            output.insert("case_insensitive".to_string(), v.clone().into());
        }
        if let Some(v) = &self.multiline {
            output.insert("multiline".to_string(), v.clone().into());
        }
        if let Some(v) = &self.dotall {
            output.insert("dotall".to_string(), v.clone().into());
        }
        if let Some(v) = &self.count {
            output.insert("count".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "RegexReplace";
    const DISPLAY_NAME: &'static str = "Replace Text (Regex)";
    const DESCRIPTION: &'static str = "Find and replace text using regex patterns.";
    const CATEGORY: &'static str = "text";
}
///**Replace Text (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Replace Text (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Compare Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringCompare<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string_a: StringAParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string_b: StringBParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub case_sensitive: CaseSensitiveParam,
}
impl<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> StringCompare<StringAParam, StringBParam, CaseSensitiveParam> {
    /// Create a new node.
    pub fn new(
        string_a: StringAParam,
        string_b: StringBParam,
        case_sensitive: CaseSensitiveParam,
    ) -> Self {
        Self {
            string_a,
            string_b,
            case_sensitive,
        }
    }
}
impl<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for StringCompare<StringAParam, StringBParam, CaseSensitiveParam> {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string_a".to_string(), self.string_a.clone().into());
        output.insert("string_b".to_string(), self.string_b.clone().into());
        output.insert("case_sensitive".to_string(), self.case_sensitive.clone().into());
        output
    }
    const NAME: &'static str = "StringCompare";
    const DISPLAY_NAME: &'static str = "Compare Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Concatenate Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringConcatenate<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    DelimiterParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string_a: StringAParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string_b: StringBParam,
    /**No documentation.

**Metadata**:
  - Multiline: false
  - Default:
*/
    pub delimiter: DelimiterParam,
}
impl<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    DelimiterParam: crate::nodes::types::String,
> StringConcatenate<StringAParam, StringBParam, DelimiterParam> {
    /// Create a new node.
    pub fn new(
        string_a: StringAParam,
        string_b: StringBParam,
        delimiter: DelimiterParam,
    ) -> Self {
        Self {
            string_a,
            string_b,
            delimiter,
        }
    }
}
impl<
    StringAParam: crate::nodes::types::String,
    StringBParam: crate::nodes::types::String,
    DelimiterParam: crate::nodes::types::String,
> crate::nodes::TypedNode
for StringConcatenate<StringAParam, StringBParam, DelimiterParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string_a".to_string(), self.string_a.clone().into());
        output.insert("string_b".to_string(), self.string_b.clone().into());
        output.insert("delimiter".to_string(), self.delimiter.clone().into());
        output
    }
    const NAME: &'static str = "StringConcatenate";
    const DISPLAY_NAME: &'static str = "Concatenate Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Contains Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringContains<
    StringParam: crate::nodes::types::String,
    SubstringParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub substring: SubstringParam,
    /**No documentation.

**Metadata**:
  - Default: true
*/
    pub case_sensitive: CaseSensitiveParam,
}
impl<
    StringParam: crate::nodes::types::String,
    SubstringParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> StringContains<StringParam, SubstringParam, CaseSensitiveParam> {
    /// Create a new node.
    pub fn new(
        string: StringParam,
        substring: SubstringParam,
        case_sensitive: CaseSensitiveParam,
    ) -> Self {
        Self {
            string,
            substring,
            case_sensitive,
        }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    SubstringParam: crate::nodes::types::String,
    CaseSensitiveParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for StringContains<StringParam, SubstringParam, CaseSensitiveParam> {
    type Output = crate::nodes::types::BooleanOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("substring".to_string(), self.substring.clone().into());
        output.insert("case_sensitive".to_string(), self.case_sensitive.clone().into());
        output
    }
    const NAME: &'static str = "StringContains";
    const DISPLAY_NAME: &'static str = "Contains Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Format Text**: Same as Python's string format method. Supports all of Python's format options and features.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringFormat<FStringParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: true
  - Default: {a}
*/
    pub f_string: FStringParam,
}
impl<FStringParam: crate::nodes::types::String> StringFormat<FStringParam> {
    /// Create a new node.
    pub fn new(f_string: FStringParam) -> Self {
        Self { f_string }
    }
}
impl<FStringParam: crate::nodes::types::String> crate::nodes::TypedNode
for StringFormat<FStringParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("f_string".to_string(), self.f_string.clone().into());
        output
    }
    const NAME: &'static str = "StringFormat";
    const DISPLAY_NAME: &'static str = "Format Text";
    const DESCRIPTION: &'static str = "Same as Python's string format method. Supports all of Python's format options and features.";
    const CATEGORY: &'static str = "text";
}
///**Text Length**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringLength<StringParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
}
impl<StringParam: crate::nodes::types::String> StringLength<StringParam> {
    /// Create a new node.
    pub fn new(string: StringParam) -> Self {
        Self { string }
    }
}
impl<StringParam: crate::nodes::types::String> crate::nodes::TypedNode
for StringLength<StringParam> {
    type Output = crate::nodes::types::IntOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output
    }
    const NAME: &'static str = "StringLength";
    const DISPLAY_NAME: &'static str = "Text Length";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Replace Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringReplace<
    StringParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub find: FindParam,
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub replace: ReplaceParam,
}
impl<
    StringParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> StringReplace<StringParam, FindParam, ReplaceParam> {
    /// Create a new node.
    pub fn new(string: StringParam, find: FindParam, replace: ReplaceParam) -> Self {
        Self { string, find, replace }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    FindParam: crate::nodes::types::String,
    ReplaceParam: crate::nodes::types::String,
> crate::nodes::TypedNode for StringReplace<StringParam, FindParam, ReplaceParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("find".to_string(), self.find.clone().into());
        output.insert("replace".to_string(), self.replace.clone().into());
        output
    }
    const NAME: &'static str = "StringReplace";
    const DISPLAY_NAME: &'static str = "Replace Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Substring**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringSubstring<
    StringParam: crate::nodes::types::String,
    StartParam: crate::nodes::types::Int,
    EndParam: crate::nodes::types::Int,
> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
    ///No documentation.
    pub start: StartParam,
    ///No documentation.
    pub end: EndParam,
}
impl<
    StringParam: crate::nodes::types::String,
    StartParam: crate::nodes::types::Int,
    EndParam: crate::nodes::types::Int,
> StringSubstring<StringParam, StartParam, EndParam> {
    /// Create a new node.
    pub fn new(string: StringParam, start: StartParam, end: EndParam) -> Self {
        Self { string, start, end }
    }
}
impl<
    StringParam: crate::nodes::types::String,
    StartParam: crate::nodes::types::Int,
    EndParam: crate::nodes::types::Int,
> crate::nodes::TypedNode for StringSubstring<StringParam, StartParam, EndParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output.insert("start".to_string(), self.start.clone().into());
        output.insert("end".to_string(), self.end.clone().into());
        output
    }
    const NAME: &'static str = "StringSubstring";
    const DISPLAY_NAME: &'static str = "Substring";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Trim Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct StringTrim<StringParam: crate::nodes::types::String> {
    /**No documentation.

**Metadata**:
  - Multiline: true
*/
    pub string: StringParam,
}
impl<StringParam: crate::nodes::types::String> StringTrim<StringParam> {
    /// Create a new node.
    pub fn new(string: StringParam) -> Self {
        Self { string }
    }
}
impl<StringParam: crate::nodes::types::String> crate::nodes::TypedNode
for StringTrim<StringParam> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("string".to_string(), self.string.clone().into());
        output
    }
    const NAME: &'static str = "StringTrim";
    const DISPLAY_NAME: &'static str = "Trim Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Strip Whitespace (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Strip Whitespace (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Generate Text**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TextGenerate<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    VideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
    ThinkingParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    UseDefaultTemplateParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 1
*/
    pub max_length: MaxLengthParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    ///Video frames as image batch. Assumed to be 24 FPS; subsampled to 1 FPS internally.
    pub video: Option<VideoParam>,
    ///No documentation.
    pub audio: Option<AudioParam>,
    /**Operate in thinking mode if the model supports it.

**Metadata**:
  - Default: false
*/
    pub thinking: Option<ThinkingParam>,
    /**Use the built in system prompt/template if the model has one.

**Metadata**:
  - Default: true
*/
    pub use_default_template: Option<UseDefaultTemplateParam>,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    VideoParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    ThinkingParam: crate::nodes::types::Boolean,
    UseDefaultTemplateParam: crate::nodes::types::Boolean,
> TextGenerate<
    ClipParam,
    PromptParam,
    MaxLengthParam,
    ImageParam,
    VideoParam,
    AudioParam,
    ThinkingParam,
    UseDefaultTemplateParam,
> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        prompt: PromptParam,
        max_length: MaxLengthParam,
        image: Option<ImageParam>,
        video: Option<VideoParam>,
        audio: Option<AudioParam>,
        thinking: Option<ThinkingParam>,
        use_default_template: Option<UseDefaultTemplateParam>,
    ) -> Self {
        Self {
            clip,
            prompt,
            max_length,
            image,
            video,
            audio,
            thinking,
            use_default_template,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    VideoParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    ThinkingParam: crate::nodes::types::Boolean,
    UseDefaultTemplateParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TextGenerate<
    ClipParam,
    PromptParam,
    MaxLengthParam,
    ImageParam,
    VideoParam,
    AudioParam,
    ThinkingParam,
    UseDefaultTemplateParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("max_length".to_string(), self.max_length.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.video {
            output.insert("video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.thinking {
            output.insert("thinking".to_string(), v.clone().into());
        }
        if let Some(v) = &self.use_default_template {
            output.insert("use_default_template".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TextGenerate";
    const DISPLAY_NAME: &'static str = "Generate Text";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Generate LTX2 Prompt**: No description.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TextGenerateLTX2Prompt<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    VideoParam: crate::nodes::types::Image = crate::nodes::types::ImageOut,
    AudioParam: crate::nodes::types::Audio = crate::nodes::types::AudioOut,
    ThinkingParam: crate::nodes::types::Boolean = crate::nodes::types::BooleanOut,
    UseDefaultTemplateParam: crate::nodes::types::Boolean
        = crate::nodes::types::BooleanOut,
> {
    ///No documentation.
    pub clip: ClipParam,
    /**No documentation.

**Metadata**:
  - Dynamic prompts: true
  - Multiline: true
  - Default:
*/
    pub prompt: PromptParam,
    /**No documentation.

**Metadata**:
  - Default: 256
  - Max: 2048
  - Min: 1
*/
    pub max_length: MaxLengthParam,
    ///No documentation.
    pub image: Option<ImageParam>,
    ///Video frames as image batch. Assumed to be 24 FPS; subsampled to 1 FPS internally.
    pub video: Option<VideoParam>,
    ///No documentation.
    pub audio: Option<AudioParam>,
    /**Operate in thinking mode if the model supports it.

**Metadata**:
  - Default: false
*/
    pub thinking: Option<ThinkingParam>,
    /**Use the built in system prompt/template if the model has one.

**Metadata**:
  - Default: true
*/
    pub use_default_template: Option<UseDefaultTemplateParam>,
}
impl<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    VideoParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    ThinkingParam: crate::nodes::types::Boolean,
    UseDefaultTemplateParam: crate::nodes::types::Boolean,
> TextGenerateLTX2Prompt<
    ClipParam,
    PromptParam,
    MaxLengthParam,
    ImageParam,
    VideoParam,
    AudioParam,
    ThinkingParam,
    UseDefaultTemplateParam,
> {
    /// Create a new node.
    pub fn new(
        clip: ClipParam,
        prompt: PromptParam,
        max_length: MaxLengthParam,
        image: Option<ImageParam>,
        video: Option<VideoParam>,
        audio: Option<AudioParam>,
        thinking: Option<ThinkingParam>,
        use_default_template: Option<UseDefaultTemplateParam>,
    ) -> Self {
        Self {
            clip,
            prompt,
            max_length,
            image,
            video,
            audio,
            thinking,
            use_default_template,
        }
    }
}
impl<
    ClipParam: crate::nodes::types::Clip,
    PromptParam: crate::nodes::types::String,
    MaxLengthParam: crate::nodes::types::Int,
    ImageParam: crate::nodes::types::Image,
    VideoParam: crate::nodes::types::Image,
    AudioParam: crate::nodes::types::Audio,
    ThinkingParam: crate::nodes::types::Boolean,
    UseDefaultTemplateParam: crate::nodes::types::Boolean,
> crate::nodes::TypedNode
for TextGenerateLTX2Prompt<
    ClipParam,
    PromptParam,
    MaxLengthParam,
    ImageParam,
    VideoParam,
    AudioParam,
    ThinkingParam,
    UseDefaultTemplateParam,
> {
    type Output = crate::nodes::types::StringOut;
    fn output(&self, node_id: WorkflowNodeId) -> Self::Output {
        Self::Output::from_dynamic(node_id, 0)
    }
    fn inputs(&self) -> HashMap<String, WorkflowInput> {
        let mut output = HashMap::default();
        output.insert("clip".to_string(), self.clip.clone().into());
        output.insert("prompt".to_string(), self.prompt.clone().into());
        output.insert("max_length".to_string(), self.max_length.clone().into());
        if let Some(v) = &self.image {
            output.insert("image".to_string(), v.clone().into());
        }
        if let Some(v) = &self.video {
            output.insert("video".to_string(), v.clone().into());
        }
        if let Some(v) = &self.audio {
            output.insert("audio".to_string(), v.clone().into());
        }
        if let Some(v) = &self.thinking {
            output.insert("thinking".to_string(), v.clone().into());
        }
        if let Some(v) = &self.use_default_template {
            output.insert("use_default_template".to_string(), v.clone().into());
        }
        output
    }
    const NAME: &'static str = "TextGenerateLTX2Prompt";
    const DISPLAY_NAME: &'static str = "Generate LTX2 Prompt";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Convert Text to Lowercase (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Convert Text to Lowercase (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
}
///**Convert Text to Uppercase (DEPRECATED)**: No description.
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
    const DISPLAY_NAME: &'static str = "Convert Text to Uppercase (DEPRECATED)";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "text";
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
    const CATEGORY: &'static str = "text";
}
