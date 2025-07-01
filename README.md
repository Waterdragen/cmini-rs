# rmini

rmini is a basic discord bot written in Rust that displays the stats of a particular layout 

## 🎉All cmini commands are complete!🎉

## 🆕Legacy minigame commands - `count`, `guess`!

## 🛠️Fixed command -  `freqd`!

### 🛠️ Other changes

- 📝 All commands have help messages
- 🌯 More rolls! Added `inrolltals`, `outrolltals`, `rolltals` commands
- 🤔 Added support for viewing empty layouts (why not?)
- 📐 `Add` command changes
  - Added support for adding layouts with arbitrary indents, which means you can now copy the result of `view` and paste layout to `add`!
  - Space between letters are now *optional*!
- 💣 Stay safe! Removed the `nuke` command
- 🔧 Permission changes
  - only owners can add admins
  - owner or admins can remove admins
  - owner cannot remove self
- ⚡ (Way) faster `filter` command - loops through the cached stats
- ⬆️ `freq` and `freqs` support up to 20 ngrams (from 6)
- 🛠️ Fixed setfingermap bounds check

### ❤️ Contributing - [Get started](GetStarted.md)