# Neovim Lua Rust Template

A template for creating Neovim plugins in Lua with an accompanying Rust library.

## Installation

### Development

```lua
{
  'saghen/blink.pairs',

  -- see lazy.nvim docs (`config.dev`): https://lazy.folke.io/configuration
  dev = true,

  -- optional, see `lua/blink.pairs/init.lua`
  dependencies = 'saghen/blink.download',

  build = 'cargo build --release',
  opts = {}
}
```

### Stable

```lua
{
  'saghen/blink.pairs',
  version = '*', -- only required with prebuilt binaries

  -- optional, see `lua/blink.pairs/init.lua`
  -- download prebuilt binaries, from github releases, and setup `cpath`
  dependencies = 'saghen/blink.download',
  -- OR build from source
  build = 'cargo build --release',

  opts = {}
}
