## Simple template


The simple template will create the following project structure:

```text
spells_existing/        -> holds all existing spells as individual json files
spells_known/           -> holds all known spells as individual json files
src/
├── app.rs              -> holds the state and application logic
├── character.rs        -> holds the character information, partially provided by character_limbs
├── character_limbs/    -> holds the character information, partially provided by character_limbs
│   ├── ability.rs      -> ability struct definition
│   ├── advantage.rs    -> advantage struct definition
│   ├── mod.rs          -> module definitions
│   ├── skill.rs        -> skill struct definition
│   └── spells.rs       -> spells struct definition
├── event.rs            -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs          -> handles the key press events and updates the application
├── help_ui.rs          -> renders help screen
├── lib.rs              -> module definitions
├── main.rs             -> entry-point
├── tabs_ui.rs          -> renders bottom right widget
├── tui.rs              -> initializes/exits the terminal interface
└── ui.rs               -> renders the widgets / UI
```

## FAQ's

- Is it any good?
  Yes!

- Sam, where are you from?
  Cambridge, MA. We've told you so many times.

## roadmap:
| Feature                                                      | Status |
| ------------------------------------------------------------ | ------ |
| bottom line with vim-like functionality for colon commands   | WIP    |
| Help screen with all key bindings, accessible through ?      | WIP    |
| switch to toml view of data if window too narrow             | ---    |
| add long and shortrest feature with R and r respectively     | ---    |
| add left-arrow and right-arrow, as well as h and l for switching between bot right windows. add down-arrow and up-arrow, as well as j and k for scrolling through these windows. (remember scrolling for each page?) | WIP    |
| Calculate AC properly                                        | ---    |

## [License](https://github.com/Theoreticallyhugo/dndui/blob/main/LICENSE.txt)
