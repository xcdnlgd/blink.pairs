mod c;
mod clojure;
mod cpp;
mod csharp;
mod dart;
mod elixir;
mod erlang;
mod fsharp;
mod go;
mod haskell;
mod haxe;
mod java;
mod javascript;
mod json;
mod kotlin;
mod lean;
mod lua;
mod objc;
mod ocaml;
mod perl;
mod php;
mod python;
mod r;
mod ruby;
mod rust;
mod scala;
mod shell;
mod swift;
mod toml;
mod typescript;
mod typst;
mod zig;

pub use c::CToken;
pub use clojure::ClojureToken;
pub use cpp::CppToken;
pub use csharp::CSharpToken;
pub use dart::DartToken;
pub use elixir::ElixirToken;
pub use erlang::ErlangToken;
pub use fsharp::FSharpToken;
pub use go::GoToken;
pub use haskell::HaskellToken;
pub use haxe::HaxeToken;
pub use java::JavaToken;
pub use javascript::JavaScriptToken;
pub use json::JsonToken;
pub use kotlin::KotlinToken;
pub use lean::LeanToken;
pub use lua::LuaToken;
pub use objc::ObjCToken;
pub use ocaml::OCamlToken;
pub use perl::PerlToken;
pub use php::PhpToken;
pub use python::PythonToken;
pub use r::RToken;
pub use ruby::RubyToken;
pub use rust::RustToken;
pub use scala::ScalaToken;
pub use shell::ShellToken;
pub use swift::SwiftToken;
pub use toml::TomlToken;
pub use typescript::TypeScriptToken;
pub use typst::TypstToken;
pub use zig::ZigToken;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    DelimiterOpen(&'static str),
    DelimiterClose(&'static str),
    LineComment,
    BlockCommentOpen,
    BlockCommentClose,
    String,
    BlockStringSymmetric(&'static str),
    BlockStringOpen,
    BlockStringClose,
    Escape,
}

// static string newtype - needed because logos callbacks can't directly return `&'static str`
struct SStr(&'static str);

#[macro_export]
macro_rules! define_token_enum {
    ($name:ident, {
        delimiters: { $($open:literal => $close:literal),* $(,)? },
        line_comment: [ $($line_comment:literal),* $(,)? ],
        block_comment: [ $($block_comment_open:literal => $block_comment_close:literal),* $(,)? ],
        string_regex: [ $($string_regex:literal),* $(,)? ],
        block_string: [
            $(symmetric $block_string_symmetric:literal),*
            $($block_string_open:literal => $block_string_close:literal),* $(,)?
        ]
    }) => {
        #[allow(unused, private_interfaces)] // Ignore warnings about unused variants and SStr
        #[derive(logos::Logos)]
        #[logos(skip r"[ \t\f]+")] // Skip whitespace
        #[logos(subpattern dstring = r#""([^"\\]|\\.)*""#)] // " string
        #[logos(subpattern sstring = r#"'([^'\\]|\\.)*'"#)] // ' string
        #[logos(subpattern schar = r#"'([^'\\]|\\.)'"#)] // ' char (single-character)
        pub enum $name {
            $(#[token($open, |_|  $crate::languages::SStr($open) )])*
            DelimiterOpen($crate::languages::SStr),

            $(#[token($close, |_| $crate::languages::SStr($close) )])*
            DelimiterClose($crate::languages::SStr),

            $(#[token($line_comment)])*
            LineComment,

            $(#[token($block_comment_open)])*
            BlockCommentOpen,
            $(#[token($block_comment_close)])*
            BlockCommentClose,

            $(#[regex($string_regex, priority = 15)])*
            String,

            $(#[token($block_string_symmetric, |_| $crate::languages::SStr($block_string_symmetric) )])*
            BlockStringSymmetric($crate::languages::SStr),

            $(#[token($block_string_open)])*
            BlockStringOpen,

            $(#[token($block_string_close, priority = 10)])*
            BlockStringClose,

            #[token("\\")]
            Escape,
        }

        impl From<$name> for $crate::languages::Token {
            fn from(value: $name) -> Self {
                match value {
                    $name::DelimiterOpen(s) => Self::DelimiterOpen(s.0),
                    $name::DelimiterClose(s) => Self::DelimiterClose(s.0),
                    $name::LineComment => Self::LineComment,
                    $name::BlockStringSymmetric(s) => Self::BlockStringSymmetric(s.0),
                    $name::BlockCommentOpen => Self::BlockCommentOpen,
                    $name::BlockCommentClose => Self::BlockCommentClose,
                    $name::String => Self::String,
                    $name::BlockStringOpen => Self::BlockStringOpen,
                    $name::BlockStringClose => Self::BlockStringClose,
                    $name::Escape => Self::Escape,
                }
            }
        }
    };
}
