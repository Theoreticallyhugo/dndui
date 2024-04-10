## Simple template


The simple template will create the following project structure:

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
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
| add left-arrow and right-arrow, as well as h and l for switching between bot right windows. add down-arrow and up-arrow, as well as j and k for scrolling through these windows. (remember scrolling for each page?) | ---    |
| Calculate AC properly                                        | ---    |
| put + or - for damaging/ healing per default. pressing + or - switches the sign to the respective sign at any point. | ---    |
