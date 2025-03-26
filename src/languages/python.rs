use crate::define_token_enum;

define_token_enum!(PythonToken, {
    delimiters: {
        "(" => ")",
        "[" => "]",
        "{" => "}"
    },
    line_comment: ["#"],
    block_comment: [],
    string_regex: ["(?&dstring)", "(?&sstring)"],
    block_string: [symmetric "\"\"\"", symmetric "'''"]
});
