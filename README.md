# Blink Pairs (blink.pairs)

Rainbow highlighting and autopairs (TODO) for Neovim. Uses a custom parser internally which takes ~4ms to parse a 400k line file, and ~0.1ms for subsequent incremental updates.

## Installation

```lua
{
  'saghen/blink.pairs',
  version = '*', -- (recommended) only required with prebuilt binaries

  -- download prebuilt binaries from github releases
  dependencies = 'saghen/blink.download',
  -- OR build from source
  build = 'cargo build --release',
  -- OR build from source with nix
  build = 'nix build .#build-plugin'

  opts = {
    highlights = {
      'RainbowOrange',
      'RainbowPurple',
      'RainbowBlue',
    },
    priority = 200,
    ns = vim.api.nvim_create_namespace('blink.pairs'),
    debug = false,
  }
}
