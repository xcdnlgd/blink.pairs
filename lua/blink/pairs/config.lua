--- Create a separate ConfigStrict which requires all fields to be set, for internal use
--- and a Config which marks all fields as optional, for use by the user like:
--- --- @module 'blink.pairs'
--- --- @type blink.pairs.Config
--- opts = {}

--- @class blink.pairs.ConfigStrict
--- @field mappings blink.pairs.MappingsConfig
--- @field highlights blink.pairs.HighlightsConfig
--- @field debug boolean

--- @class blink.pairs.MappingsConfig
--- @field enabled boolean
--- @field pairs blink.pairs.Pairs

--- @class blink.pairs.HighlightsConfig
--- @field enabled boolean
--- @field groups string[]
--- @field priority number
--- @field ns integer

--- @class blink.pairs.Config : blink.pairs.ConfigStrict, {}

--- @type blink.pairs.ConfigStrict
local config = {
  mappings = {
    enabled = true,
    pairs = {
      ['('] = ')',
      ['['] = ']',
      ['{'] = '}',
      ["'"] = { closing = "'", enter = false },
      ['"'] = { closing = '"', enter = false },
      ['`'] = { closing = '`', enter = false },
    },
  },
  highlights = {
    enabled = true,
    groups = {
      'RainbowOrange',
      'RainbowPurple',
      'RainbowBlue',
    },
    priority = 200,
    ns = vim.api.nvim_create_namespace('blink.pairs'),
  },
  debug = false,
}

--- @type blink.pairs.ConfigStrict
--- @diagnostic disable-next-line: missing-fields
local M = {}

--- @param config blink.pairs.ConfigStrict
function M.validate(config)
  -- use vim.validate to validate the config
end

--- @param user_config blink.pairs.Config
function M.merge_with(user_config)
  config = vim.tbl_deep_extend('force', config, user_config)
  M.validate(config)
end

return setmetatable(M, {
  __index = function(_, k) return config[k] end,
})
