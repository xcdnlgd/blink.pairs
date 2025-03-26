use crate::define_token_enum;

define_token_enum!(RubyToken, {
    delimiters: {
        "(" => ")",
        "[" => "]",
        "{" => "}"
    },
    line_comment: ["#"],
    block_comment: ["=begin" => "end"],
    string_regex: ["(?&dstring)", "(?&sstring)"],
    block_string: []
});
