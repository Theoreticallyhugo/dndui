## Simple template


The simple template will create the following project structure:

```text
spells_existing/  -> holds all existing spells as individual json files
spells_known/     -> holds all known spells as individual json files
src/
├── app.rs        -> holds the state and application logic
├── character.rs  -> holds the character information, partially provided by character_limbs
├┬─ character_limbs/
│├─          -> holds the character information, partially provided by character_limbs
├── event.rs      -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs    -> handles the key press events and updates the application
├── lib.rs        -> module definitions
├── main.rs       -> entry-point
├── tui.rs        -> initializes/exits the terminal interface
└── ui.rs         -> renders the widgets / UI

.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── spells_existing
├── spells_known
│   ├── bless.json
│   ├── command.json
│   ├── find_familiar.json
│   └── purify_food_and_drink.json
├── src
│   ├── app.rs
│   ├── character.rs
│   ├── character_limbs
│   │   ├── ability.rs
│   │   ├── advantage.rs
│   │   ├── mod.rs
│   │   ├── skill.rs
│   │   └── spells.rs
│   ├── event.rs
│   ├── handler.rs
│   ├── help_ui.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── refactor.py
│   ├── tabs_ui.rs
│   ├── tui.rs
│   └── ui.rs
└── target
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
