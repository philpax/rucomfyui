//! Lua code generation from workflows using full_moon AST.

use crate::workflow_analyzer::{AnalyzedInput, AnalyzedNode, AnalyzedWorkflow};
use crate::Result;
use convert_case::{Case, Casing};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Block, Call, Expression, Field, FunctionArgs, FunctionCall, Index, LocalAssignment, Prefix,
    Stmt, Suffix, TableConstructor, Var, VarExpression,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::ShortString;
use rucomfyui::object_info::{Object, ObjectInfo};
use std::collections::HashMap;

// =============================================================================
// Token creation helpers
// =============================================================================

fn ident(name: &str) -> TokenReference {
    TokenReference::new(
        vec![],
        Token::new(TokenType::Identifier {
            identifier: ShortString::new(name),
        }),
        vec![],
    )
}

fn ident_with_leading_space(name: &str) -> TokenReference {
    TokenReference::new(
        vec![Token::new(TokenType::Whitespace {
            characters: ShortString::new(" "),
        })],
        Token::new(TokenType::Identifier {
            identifier: ShortString::new(name),
        }),
        vec![],
    )
}

fn ident_with_leading_newline_indent(name: &str) -> TokenReference {
    TokenReference::new(
        vec![Token::new(TokenType::Whitespace {
            characters: ShortString::new("\n    "),
        })],
        Token::new(TokenType::Identifier {
            identifier: ShortString::new(name),
        }),
        vec![],
    )
}

fn symbol(s: &str) -> TokenReference {
    TokenReference::symbol(s).expect("valid symbol")
}

fn symbol_with_leading_space(s: &str) -> TokenReference {
    TokenReference::symbol(&format!(" {}", s)).expect("valid symbol")
}

fn symbol_with_trailing_space(s: &str) -> TokenReference {
    TokenReference::symbol(&format!("{} ", s)).expect("valid symbol")
}

fn number_token(value: &str) -> TokenReference {
    TokenReference::new(
        vec![],
        Token::new(TokenType::Number {
            text: ShortString::new(value),
        }),
        vec![],
    )
}

fn string_token(value: &str) -> TokenReference {
    TokenReference::new(
        vec![],
        Token::new(TokenType::StringLiteral {
            literal: ShortString::new(value),
            multi_line_depth: 0,
            quote_type: full_moon::tokenizer::StringLiteralQuoteType::Double,
        }),
        vec![],
    )
}

fn comment_token(text: &str) -> Token {
    Token::new(TokenType::SingleLineComment {
        comment: ShortString::new(text),
    })
}

fn local_token() -> TokenReference {
    TokenReference::new(
        vec![],
        Token::new(TokenType::Symbol {
            symbol: full_moon::tokenizer::Symbol::Local,
        }),
        vec![],
    )
}

fn local_token_with_leading_newline() -> TokenReference {
    TokenReference::new(
        vec![Token::new(TokenType::Whitespace {
            characters: ShortString::new("\n"),
        })],
        Token::new(TokenType::Symbol {
            symbol: full_moon::tokenizer::Symbol::Local,
        }),
        vec![],
    )
}

fn local_token_with_comment(comment: &str) -> TokenReference {
    TokenReference::new(
        vec![
            Token::new(TokenType::Whitespace {
                characters: ShortString::new("\n"),
            }),
            comment_token(&format!(" {}", comment)),
            Token::new(TokenType::Whitespace {
                characters: ShortString::new("\n"),
            }),
        ],
        Token::new(TokenType::Symbol {
            symbol: full_moon::tokenizer::Symbol::Local,
        }),
        vec![],
    )
}

fn local_token_first_with_comment(comment: &str) -> TokenReference {
    TokenReference::new(
        vec![
            comment_token(&format!(" {}", comment)),
            Token::new(TokenType::Whitespace {
                characters: ShortString::new("\n"),
            }),
        ],
        Token::new(TokenType::Symbol {
            symbol: full_moon::tokenizer::Symbol::Local,
        }),
        vec![],
    )
}

// =============================================================================
// Expression builders
// =============================================================================

fn string_expr(value: &str) -> Expression {
    Expression::String(string_token(&escape_lua_string(value)))
}

fn number_expr(value: &str) -> Expression {
    Expression::Number(number_token(value))
}

fn var_expr(name: &str) -> Expression {
    Expression::Var(Var::Name(ident(name)))
}

fn field_access_expr(var_name: &str, field_name: &str) -> Expression {
    Expression::Var(Var::Expression(Box::new(
        VarExpression::new(Prefix::Name(ident(var_name))).with_suffixes(vec![Suffix::Index(
            Index::Dot {
                dot: symbol("."),
                name: ident(field_name),
            },
        )]),
    )))
}

fn index_access_expr(var_name: &str, index: u32) -> Expression {
    Expression::Var(Var::Expression(Box::new(
        VarExpression::new(Prefix::Name(ident(var_name))).with_suffixes(vec![Suffix::Index(
            Index::Brackets {
                brackets: ContainedSpan::new(symbol("["), symbol("]")),
                expression: number_expr(&index.to_string()),
            },
        )]),
    )))
}

fn bool_expr(value: bool) -> Expression {
    let token = if value {
        TokenReference::symbol("true").unwrap()
    } else {
        TokenReference::symbol("false").unwrap()
    };
    Expression::Symbol(token)
}

// =============================================================================
// Statement builders
// =============================================================================

/// Build a local assignment: `local name = expr`
fn local_assignment(local_tok: TokenReference, name: &str, expr: Expression) -> Stmt {
    let mut names = Punctuated::new();
    names.push(Pair::End(ident_with_leading_space(name)));

    let mut exprs = Punctuated::new();
    exprs.push(Pair::End(expr));

    Stmt::LocalAssignment(
        LocalAssignment::new(names)
            .with_local_token(local_tok)
            .with_equal_token(Some(symbol_with_leading_space("=")))
            .with_expressions(exprs),
    )
}

/// Build a method call: `prefix:method(args)` or `prefix:method { table }`
fn method_call(prefix_name: &str, method_name: &str, args: FunctionArgs) -> FunctionCall {
    FunctionCall::new(Prefix::Name(ident(prefix_name))).with_suffixes(vec![Suffix::Call(
        Call::MethodCall(
            full_moon::ast::MethodCall::new(ident(method_name), args).with_colon_token(symbol(":")),
        ),
    )])
}

/// Build parenthesized arguments: `(expr)`
fn paren_args(expr: Expression) -> FunctionArgs {
    let mut exprs = Punctuated::new();
    exprs.push(Pair::End(expr));
    FunctionArgs::Parentheses {
        parentheses: ContainedSpan::new(symbol("("), symbol(")")),
        arguments: exprs,
    }
}

/// Build empty parenthesized arguments: `()`
fn empty_paren_args() -> FunctionArgs {
    FunctionArgs::Parentheses {
        parentheses: ContainedSpan::new(symbol("("), symbol(")")),
        arguments: Punctuated::new(),
    }
}

/// Build table constructor arguments: `{ fields }`
fn table_args(fields: Vec<(String, Expression)>) -> FunctionArgs {
    let mut punctuated_fields: Punctuated<Field> = Punctuated::new();

    for (name, expr) in fields.into_iter() {
        let field = Field::NameKey {
            key: ident_with_leading_newline_indent(&name),
            equal: symbol_with_leading_space("="),
            value: expr,
        };
        punctuated_fields.push(Pair::Punctuated(field, symbol(",")));
    }

    FunctionArgs::TableConstructor(
        TableConstructor::new()
            .with_braces(ContainedSpan::new(
                symbol_with_trailing_space("{"),
                symbol("\n}"),
            ))
            .with_fields(punctuated_fields),
    )
}

// =============================================================================
// Main conversion functions
// =============================================================================

/// Convert a workflow JSON to Lua code using ObjectInfo for type information.
pub fn convert_to_lua(json: &str, object_info: &ObjectInfo) -> Result<String> {
    let ast = convert_to_lua_ast(json, object_info)?;
    Ok(ast.to_string())
}

/// Convert a workflow JSON to a full_moon AST.
pub fn convert_to_lua_ast(json: &str, object_info: &ObjectInfo) -> Result<full_moon::ast::Ast> {
    let analyzed = AnalyzedWorkflow::from_json(json)?;
    generate_lua_ast(&analyzed, object_info)
}

fn generate_lua_ast(
    analyzed: &AnalyzedWorkflow,
    object_info: &ObjectInfo,
) -> Result<full_moon::ast::Ast> {
    let mut stmts: Vec<(Stmt, Option<TokenReference>)> = Vec::new();

    // Track generated variables with their ObjectInfo
    let mut generated_vars: HashMap<String, (&AnalyzedNode, Option<&Object>)> = HashMap::new();

    // Generate code for each node
    for (i, node) in analyzed.nodes.iter().enumerate() {
        let obj = object_info.get(&node.class_type);

        // Build the function call expression
        let func_call_expr = build_node_call(node, &generated_vars)?;

        // Determine the local token (with or without comment, first or not)
        let local_tok = match (&node.display_name, i == 0) {
            (Some(display_name), true) => {
                let comment = format!("{} ({})", display_name, node.class_type);
                local_token_first_with_comment(&comment)
            }
            (Some(display_name), false) => {
                let comment = format!("{} ({})", display_name, node.class_type);
                local_token_with_comment(&comment)
            }
            (None, true) => local_token(),
            (None, false) => local_token_with_leading_newline(),
        };

        let stmt = local_assignment(
            local_tok,
            &node.var_name,
            Expression::FunctionCall(func_call_expr),
        );

        stmts.push((stmt, None));
        generated_vars.insert(node.var_name.clone(), (node, obj));
    }

    let block = Block::new().with_stmts(stmts);
    let empty_ast = full_moon::parse("").expect("empty string is valid lua");
    Ok(empty_ast.with_nodes(block))
}

fn build_node_call(
    node: &AnalyzedNode,
    generated_vars: &HashMap<String, (&AnalyzedNode, Option<&Object>)>,
) -> Result<FunctionCall> {
    // Check if we can use positional or need named arguments
    let use_table = node.inputs.len() > 1
        || node
            .inputs
            .iter()
            .any(|(_, v)| matches!(v, AnalyzedInput::NodeRef { .. }));

    let args = if node.inputs.is_empty() {
        empty_paren_args()
    } else if !use_table && node.inputs.len() == 1 {
        // Single simple argument - can use positional
        let (_, input) = node.inputs.iter().next().unwrap();
        let expr = build_input_expr(input, generated_vars)?;
        paren_args(expr)
    } else {
        // Multiple arguments or node references - use table syntax
        let fields: Vec<(String, Expression)> = node
            .inputs
            .iter()
            .map(|(name, input)| {
                let expr = build_input_expr(input, generated_vars)?;
                Ok((name.clone(), expr))
            })
            .collect::<Result<Vec<_>>>()?;
        table_args(fields)
    };

    Ok(method_call("g", &node.class_type, args))
}

fn build_input_expr(
    input: &AnalyzedInput,
    generated_vars: &HashMap<String, (&AnalyzedNode, Option<&Object>)>,
) -> Result<Expression> {
    match input {
        AnalyzedInput::String(s) => Ok(string_expr(s)),
        AnalyzedInput::Integer(i) => Ok(number_expr(&i.to_string())),
        AnalyzedInput::Float(f) => {
            let s = f.to_string();
            if s.contains('.') || s.contains('e') || s.contains('E') {
                Ok(number_expr(&s))
            } else {
                Ok(number_expr(&format!("{}.0", s)))
            }
        }
        AnalyzedInput::Boolean(b) => Ok(bool_expr(*b)),
        AnalyzedInput::NodeRef { var_name, slot } => {
            // Look up the referenced node to get its output field name from ObjectInfo
            if let Some((_ref_node, Some(ref_obj))) = generated_vars.get(var_name) {
                let outputs: Vec<_> = ref_obj.processed_output().collect();
                if outputs.len() > 1 {
                    // Multi-output node - use field name
                    if let Some(output) = outputs.get(*slot as usize) {
                        let field_name = output.name.to_case(Case::Snake);
                        return Ok(field_access_expr(var_name, &field_name));
                    }
                }
            }
            // Single-output or unknown - use variable directly
            if *slot == 0 {
                Ok(var_expr(var_name))
            } else {
                // For unknown multi-output nodes, use array-style access
                // Lua is 1-indexed, so we add 1
                Ok(index_access_expr(var_name, *slot + 1))
            }
        }
    }
}

fn escape_lua_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use full_moon::node::Node;

    fn load_test_object_info() -> ObjectInfo {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let object_info_path = manifest_dir
            .parent()
            .unwrap()
            .join("rucomfyui")
            .join("generate_nodes")
            .join("object_info.json");

        let json = std::fs::read_to_string(&object_info_path)
            .unwrap_or_else(|e| panic!("Failed to read object_info.json: {}", e));

        let objects: Vec<rucomfyui::object_info::Object> =
            serde_json::from_str(&json).expect("Failed to parse object_info.json");

        objects.into_iter().map(|o| (o.name.clone(), o)).collect()
    }

    /// Dedent a string by removing common leading whitespace from all lines.
    fn dedent(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let non_empty_lines: Vec<&str> = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .copied()
            .collect();

        if non_empty_lines.is_empty() {
            return String::new();
        }

        let min_indent = non_empty_lines
            .iter()
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line.trim()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    /// Parse Lua code and return the AST.
    fn parse_lua(code: &str) -> full_moon::ast::Ast {
        let dedented = dedent(code);
        full_moon::parse(&dedented).expect("Failed to parse Lua code")
    }

    /// Assert that a generated AST is semantically similar to expected code.
    fn assert_ast_eq(actual: &full_moon::ast::Ast, expected: &str) {
        let expected_ast = parse_lua(expected);

        assert!(
            actual.nodes().similar(expected_ast.nodes()),
            "\n\nActual:\n{}\n\nExpected:\n{}",
            actual,
            expected_ast
        );
    }

    #[test]
    fn test_convert_simple_workflow() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "sd_xl_base_1.0.safetensors" },
                "class_type": "CheckpointLoaderSimple",
                "_meta": { "title": "Load Checkpoint" }
            }
        }"#;

        // Test AST generation directly
        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            -- Load Checkpoint (CheckpointLoaderSimple)
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("sd_xl_base_1.0.safetensors")
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_convert_workflow_with_dependencies() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": {
                    "text": "a cat",
                    "clip": ["1", 1]
                },
                "class_type": "CLIPTextEncode"
            }
        }"#;

        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "a cat",
            }
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_multi_output_node_references() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "samples": ["1", 0], "vae": ["1", 2] },
                "class_type": "VAEDecode"
            }
        }"#;

        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local vae_decode = g:VAEDecode {
                samples = checkpoint_loader_simple.model,
                vae = checkpoint_loader_simple.vae,
            }
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_numeric_inputs() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": {
                    "width": 1024,
                    "height": 768,
                    "batch_size": 1
                },
                "class_type": "EmptyLatentImage"
            }
        }"#;

        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            local empty_latent_image = g:EmptyLatentImage {
                batch_size = 1.0,
                height = 768.0,
                width = 1024.0,
            }
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_variable_naming_dedup() {
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": { "text": "first", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            },
            "3": {
                "inputs": { "text": "second", "clip": ["1", 1] },
                "class_type": "CLIPTextEncode"
            }
        }"#;

        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "first",
            }
            local clip_text_encode_1 = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "second",
            }
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_example_workflow() {
        let object_info = load_test_object_info();
        let json = include_str!("../testdata/example_workflow.json");

        let ast = convert_to_lua_ast(json, &object_info).unwrap();

        let expected = r#"
            -- Load Checkpoint (CheckpointLoaderSimple)
            local checkpoint_loader_simple = g:CheckpointLoaderSimple("model.safetensors")
            -- Empty Latent (EmptyLatentImage)
            local empty_latent_image = g:EmptyLatentImage {
                batch_size = 1.0,
                height = 1024.0,
                width = 1024.0,
            }
            -- Positive Prompt (CLIPTextEncode)
            local clip_text_encode = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "a beautiful landscape",
            }
            -- Negative Prompt (CLIPTextEncode)
            local clip_text_encode_1 = g:CLIPTextEncode {
                clip = checkpoint_loader_simple.clip,
                text = "ugly, blurry",
            }
            -- Sampler (KSampler)
            local k_sampler = g:KSampler {
                cfg = 7.5,
                denoise = 1.0,
                latent_image = empty_latent_image,
                model = checkpoint_loader_simple.model,
                negative = clip_text_encode_1,
                positive = clip_text_encode,
                sampler_name = "euler",
                scheduler = "normal",
                seed = 42.0,
                steps = 20.0,
            }
            -- VAE Decode (VAEDecode)
            local vae_decode = g:VAEDecode {
                samples = k_sampler,
                vae = checkpoint_loader_simple.vae,
            }
            -- Preview (PreviewImage)
            local preview_image = g:PreviewImage {
                images = vae_decode,
            }
        "#;

        assert_ast_eq(&ast, expected);
    }

    #[test]
    fn test_string_to_ast_roundtrip() {
        // Verify that convert_to_lua produces valid parseable Lua
        let object_info = load_test_object_info();
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            }
        }"#;

        let result = convert_to_lua(json, &object_info).unwrap();
        // This should not panic - the output should be valid Lua
        let _parsed = full_moon::parse(&result).expect("Generated Lua should be valid");
    }
}
