--- @class blink.pairs.Pair
--- @field closing string
--- @field enter? boolean
--- @field backspace? boolean
---
--- @alias blink.pairs.Pairs table<string, blink.pairs.Pair | string>
---
--- @class blink.pairs.PairStrict
--- @field opening string
--- @field closing string
--- @field enter? boolean
--- @field backspace? boolean
--- @field space? boolean
---
--- @alias blink.pairs.PairsStrict table<string, blink.pairs.PairStrict>

local pairs_iter = pairs
local mappings = {}

--- @param pairs blink.pairs.Pairs
function mappings.register(pairs)
  local map = function(lhs, rhs) vim.keymap.set('i', lhs, rhs, { silent = true, noremap = true, expr = true }) end

  pairs = vim.deepcopy(pairs)
  for opening, def in pairs_iter(pairs) do
    if type(def) == 'string' then
      pairs[opening] = { closing = def, opening = opening }
    else
      pairs[opening] = { closing = def.closing, opening = opening, enter = def.enter, backspace = def.backspace }
    end
  end
  --- @cast pairs blink.pairs.PairsStrict

  for opening, def in pairs_iter(pairs) do
    if opening == def.closing then
      map(opening, mappings.open_or_close_pair(opening))
    else
      map(opening, mappings.open_pair(opening, def.closing))
      map(def.closing, mappings.close_pair(def.closing))
    end
  end

  map('<BS>', mappings.backspace(pairs))
  map('<CR>', mappings.enter(pairs))
  map('<Space>', mappings.space(pairs))
end

function mappings.shift_backward_keycode(amount) return string.rep('<C-g>u<Left>', amount) end

function mappings.shift_forward_keycode(amount) return string.rep('<C-g>u<Right>', amount) end

function mappings.open_pair(opening, closing)
  return function()
    -- "\|" -> "\(|"
    if mappings.is_escaped() then return opening end
    -- "|" -> "(|)"
    return opening .. closing .. mappings.shift_backward_keycode(1)
  end
end

function mappings.close_pair(closing)
  return function()
    local cursor = vim.api.nvim_win_get_cursor(0)
    local line = vim.api.nvim_get_current_line()
    local next_char = line:sub(cursor[2] + 1, cursor[2] + 1)
    local next_next_char = line:sub(cursor[2] + 2, cursor[2] + 2)

    -- "|)" -> ")|"
    if next_char == closing then return mappings.shift_forward_keycode(1) end
    -- "| )" -> " )|"
    if next_char == ' ' and next_next_char == closing then return mappings.shift_forward_keycode(2) end

    return closing
  end
end

function mappings.open_or_close_pair(pair)
  return function()
    local cursor = vim.api.nvim_win_get_cursor(0)
    local line = vim.api.nvim_get_current_line()
    local next_char = line:sub(cursor[2] + 1, cursor[2] + 1)

    -- "|'" -> "'|"
    if next_char == pair then return mappings.close_pair(pair)() end

    -- "|" -> "'|'"
    return mappings.open_pair(pair, pair)()
  end
end

--- @param pairs blink.pairs.PairsStrict
function mappings.backspace(pairs)
  return function()
    local pair = mappings.get_current_pair(pairs)
    -- "(|)" -> "|"
    if pair ~= nil and pair.backspace ~= false then return mappings.shift_forward_keycode(1) .. '<BS><BS>' end

    return '<BS>'
  end
end

--- @param pairs blink.pairs.PairsStrict
function mappings.enter(pairs)
  return function()
    local pair = mappings.get_current_pair(pairs)
    -- "(|)" ->
    -- (
    --   |
    -- )
    if pair ~= nil and pair.enter ~= false then return '<CR><C-o>O' end

    return '<CR>'
  end
end

--- @param pairs blink.pairs.PairsStrict
function mappings.space(pairs)
  return function()
    local pair = mappings.get_current_pair(pairs)
    -- "(|)" -> "( | )"
    if pair ~= nil and pair.space ~= false then return '<Space><Space><C-g>u<Left>' end

    return '<Space>'
  end
end

--- @param pairs blink.pairs.PairsStrict
--- @return blink.pairs.PairStrict?
function mappings.get_current_pair(pairs)
  local cursor = vim.api.nvim_win_get_cursor(0)
  local line = vim.api.nvim_get_current_line()

  local prev_char = line:sub(cursor[2], cursor[2])
  local next_char = line:sub(cursor[2] + 1, cursor[2] + 1)

  local def = pairs[prev_char]
  if def ~= nil and next_char == def.closing then return def end
end

function mappings.is_escaped()
  local cursor = vim.api.nvim_win_get_cursor(0)
  local line = vim.api.nvim_get_current_line()

  return line:sub(cursor[2], cursor[2]) == '\\'
end

return mappings
