use crate::define_token_enum;

define_token_enum!(KotlinToken, {
    delimiters: {
        "(" => ")",
        "[" => "]",
        "{" => "}"
    },
    line_comment: ["//"],
    block_comment: ["/*" => "*/"],
    string_regex: ["(?&dstring)"],
    block_string: [symmetric "\"\"\""]
});
