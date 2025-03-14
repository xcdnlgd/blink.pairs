use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use nvim_oxi::api::Buffer;
use nvim_oxi::{Dictionary, Function, Object, Result};
use parser::{parse_lines, recalculate_stack_heights, Match};

pub mod languages;
pub mod parser;

static PARSED_BUFFERS: LazyLock<Mutex<HashMap<i32, Vec<Vec<Match>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[nvim_oxi::plugin]
pub fn blink_pairs() -> Result<Dictionary> {
    let parse_buffer: Function<(i32, Option<i32>, Option<i32>, Option<i32>), Result<bool>> =
        Function::from_fn(
            |(bufnr, start_line, old_end_line, new_end_line): (
                i32,
                Option<i32>,
                Option<i32>,
                Option<i32>,
            )|
             -> Result<bool> {
                let mut parsed_buffers = PARSED_BUFFERS.lock().unwrap();

                let buffer = Buffer::from(bufnr);
                let buffer_handle = buffer.handle();

                // Incremental parse
                if let Some(existing_matches_by_line) = parsed_buffers.get_mut(&buffer_handle) {
                    let start_line = start_line.unwrap_or(0) as usize;
                    let old_end_line = old_end_line.unwrap_or(0) as usize;
                    let new_end_line =
                        new_end_line.unwrap_or(buffer.line_count().unwrap() as i32) as usize;

                    let old_range = start_line..old_end_line;
                    let new_range = start_line..new_end_line;

                    let matches_by_line = parse_lines(buffer, new_range);
                    return match matches_by_line {
                        None => Ok(false),
                        Some(matches_by_line) => {
                            existing_matches_by_line.splice(old_range, matches_by_line);
                            recalculate_stack_heights(existing_matches_by_line);
                            Ok(true)
                        }
                    };
                }
                // Full parse
                else if let Some(matches_by_line) = parse_lines(buffer, 0..) {
                    parsed_buffers.insert(buffer_handle, matches_by_line);
                    return Ok(true);
                }

                Ok(false)
            },
        );

    let get_parsed_line: Function<(i32, i32), Result<Vec<Match>>> =
        Function::from_fn(|(bufnr, line_number): (i32, i32)| -> Result<Vec<Match>> {
            let parsed_buffers = PARSED_BUFFERS.lock().unwrap();
            let parsed_buffer = parsed_buffers.get(&bufnr);

            if let Some(parsed_buffer) = parsed_buffer {
                let line = parsed_buffer.get(line_number as usize);
                if let Some(line) = line {
                    return Ok(line.clone());
                }
            }

            Ok(Vec::new())
        });

    Ok(Dictionary::from_iter([
        ("parse_buffer", Object::from(parse_buffer)),
        ("get_parsed_line", Object::from(get_parsed_line)),
    ]))
}
