use crate::define_token_enum;

// Includes comments from jsonc and json5
define_token_enum!(JsonToken, {
    delimiters: {
        "[" => "]",
        "{" => "}"
    },
    line_comment: ["//"],
    block_comment: ["/*" => "*/"],
    string: ["\""],
    block_string: []
});
