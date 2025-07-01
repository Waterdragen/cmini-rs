# rmini

rmini is a basic discord bot written in Rust that displays the stats of a particular layout 

## 🎉All cmini commands are complete!🎉

## 🆕Legacy minigame commands - `count`, `guess`!
![Screenshot (331)](https://github.com/user-attachments/assets/56195094-b6bc-4861-b564-47c5c240f647)
![Screenshot (330)](https://github.com/user-attachments/assets/7b05ee47-4364-433b-aa93-6f9edfe35322)


## 🛠️Fixed command -  `freqd`!
![Screenshot (332)](https://github.com/user-attachments/assets/20df05ae-47f1-42a8-a96c-c050c6a10ad0)

### 🛠️ Other changes

- 📝 All commands have help messages
- 🌯 More rolls! Added `inrolltals`, `outrolltals`, `rolltals` commands
- 🤔 Added support for viewing empty layouts (why not?)
  - <img src="https://github.com/user-attachments/assets/35abcf0f-7c5b-40e0-8e15-7b6fdc785e6d" height="400">

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
