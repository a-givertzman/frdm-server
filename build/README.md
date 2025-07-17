# Converts markdown files to the solid html document

## Required folder structure

```markdown
.
├── ...
├── doc                    # Documentation files
│   ├── algorithm              # Documantation of the algorithms 
│   │   ├── algorithm.md        # Contains a header of the algorithm
│   │   ├── part01              # Part 1 of algorithms
│   │   │   ├── part01.md              # Contains a header of the part01
│   │   │   ├── chapter01              # Chapter 1 of Part 1 of algorithms
│   │   │   │   ├── chapter01.md           # Contains a header of the chapter01
│   │   │   │   ├── section01              # Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   ├── section01.md             # Contains a header of the section01
│   │   │   │   │   ├── algorithm01              # Algorithm 1 of Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   │   ├── algorithm01.md          # Contains a header of the algorithm01
│   │   │   │   │   │   ├── point01.md              # Point 1 of Algorithm 1 of Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   │   ├── point02.md              # Point 2 of Algorithm 1 of Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   │   ├── point03.md              # Point 3 of Algorithm 1 of Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   │   └── ...                 # etc.
│   │   │   │   │   ├── algorithm02              # Algorithm 2 of Section 1 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   │   └── ...                 # etc.
│   │   │   │   ├── section02              # Section 2 of Chapter 1 of Part 1 of algorithms
│   │   │   │   │   └── ...                 # etc.
│   │   │   ├── chapter02              # Chapter 2 of algorithms
│   │   │   │   └── ...                 # etc.
│   │   ├── part02              # Part 2 of algorithms
│   │   │   └── ...                 # etc.
│   │   └── ...                # etc.
│   ├── reference              # Reference Documantation 
│   └── ...                 # etc.
└── ...
```

## Assets

- SVG images will be embedded into the HTML code
- Other kind of images will be loaded by the ref

## Execute in cli

- Having folders:

```markdown
├── doc                    # Documentation files
│   ├── assets/            # Assets files 
│   │   ├── img.svg             # Image files...
│   │   └── ...
│   ├── html/              # Target folder, where generated htm to be stored
│   │   ├── doc.md              # Target solid markdown document to be crated
│   │   └── doc.html            # Target solid html document to be crated
│   ├── algorithm/         # Source md folders ans files 
│   │   ├── algorithm.md        # Contains a header of the algorithm
│   │   ├── part01/             # Part 1 of algorithms
│   │   │   └── ...
│   │   └── ...
│   └── ...
└── ...

```

- Use command:

```bash
cargo run --release -- ./doc/algorithm/ --assets ./doc/assets --output ./doc/html/
```

or

```bash
cargo run --release -- ./doc/algorithm/ --assets ./doc/assets/ --output ./doc/html/doc.html
```

## References

Rust crate [build_html](https://docs.rs/build_html/latest/build_html/)
Rust pacages for [HTML Templating](https://www.arewewebyet.org/topics/templating/)
Rust macro for writing [HTML templates](https://docs.rs/maud/latest/maud/)