//!`string` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
///**Case Converter**: No description.
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
    const DISPLAY_NAME: &'static str = "Case Converter";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Regex Extract**: No description.
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
    const DISPLAY_NAME: &'static str = "Regex Extract";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Regex Match**: No description.
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
    const DISPLAY_NAME: &'static str = "Regex Match";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Regex Replace**: Find and replace text using regex patterns.
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
    const DISPLAY_NAME: &'static str = "Regex Replace";
    const DESCRIPTION: &'static str = "Find and replace text using regex patterns.";
    const CATEGORY: &'static str = "utils/string";
}
///**Compare**: No description.
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
    const DISPLAY_NAME: &'static str = "Compare";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Concatenate**: No description.
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
    const DISPLAY_NAME: &'static str = "Concatenate";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Contains**: No description.
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
    const DISPLAY_NAME: &'static str = "Contains";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Length**: No description.
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
    const DISPLAY_NAME: &'static str = "Length";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
///**Replace**: No description.
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
    const DISPLAY_NAME: &'static str = "Replace";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
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
    const CATEGORY: &'static str = "utils/string";
}
///**Trim**: No description.
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
    const DISPLAY_NAME: &'static str = "Trim";
    const DESCRIPTION: &'static str = "";
    const CATEGORY: &'static str = "utils/string";
}
