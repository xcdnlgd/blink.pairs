-- TODO: injected languages for markdown
-- TODO: many many more language definitions

local pairs = {}

--- @param user_config blink.pairs.Config
function pairs.setup(user_config)
  local config = require('blink.pairs.config')
  config.merge_with(user_config)

  pairs.download_if_available(function(err)
    if err then error(err) end

    vim.api.nvim_set_decoration_provider(config.ns, {
      on_win = function(_, _, bufnr) return require('blink.pairs.watcher').attach(bufnr) end,
      on_line = function(_, _, bufnr, line_number)
        for _, match in ipairs(require('blink.pairs.rust').get_parsed_line(bufnr, line_number)) do
          vim.api.nvim_buf_set_extmark(bufnr, config.ns, line_number, match.col, {
            end_col = match.col + 1,
            hl_group = config.highlights[match.stack_height % #config.highlights + 1],
            hl_mode = 'combine',
            priority = config.priority,
            ephemeral = true,
          })
        end
      end,
    })
  end)
end

--- You may optionally use `blink.download` for prebuilt binaries with the included `Cross.toml`
--- and `.github/workflows/release.yaml`
function pairs.download_if_available(callback)
  local success, downloader = pcall(require, 'blink.download')
  if not success then return callback() end

  -- See https://github.com/Saghen/blink.download for more info
  local root_dir = vim.fn.resolve(debug.getinfo(1).source:match('@?(.*/)') .. '../../../')

  downloader.ensure_downloaded({
    -- omit this property to disable downloading
    -- i.e. https://github.com/Saghen/blink.pairs/releases/download/v0.1.0/x86_64-unknown-linux-gnu.so
    download_url = function(version, system_triple, extension)
      return 'https://github.com/saghen/blink.pairs/releases/download/' .. version .. '/' .. system_triple .. extension
    end,
    on_download = function()
      vim.notify('[blink.pairs] Downloading prebuilt binary...', vim.log.levels.INFO, { title = 'blink.pairs' })
    end,

    root_dir,
    output_dir = '/target/release',
    binary_name = 'blink_pairs', -- excluding `lib` prefix
  }, callback)
end

return pairs
