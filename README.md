# rust-learning

My learning code of [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

## Usage

### cargo command in watch mode

```shell
# install
cargo install cargo-watch
npm i -g browser-sync

# use
cargo watch -x doc
browser-sync ./target/doc

# open browser: http://localhost:3000/your_crate
```

## Rust Book - Table of Content

**Naming Format: `original article title number` - `related code` - `original article url`**

I believe it is unnecessary to document the code for some sections, therefore the following table of contents is not complete.

- 02 - [guessing_game](./guessing_game/) - [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- 05 - [using_structs_to_structure_related_data](./using_structs_to_structure_related_data/) - [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- 06 - [enums_and_pattern_matching](./enums_and_pattern_matching/) - [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- 07 - [managing_growing_projects_with_packages_crates_and_modules](./managing_growing_projects_with_packages_crates_and_modules/) - [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- 10 - [generic_types_traits_and_lifetimes](./generic_types_traits_and_lifetimes/) - [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- 11 - [writing_automated_tests](./writing_automated_tests/) - [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- 12 - [building_a_command_line_program](./building_a_command_line_program/) - [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- 13 - [iterators_and_closures](./iterators_and_closures/) - [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- 14 - [More About Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
  - [api_doc](./api_doc/)
  - [pub_use](./pub_use/)
- 15 - [smart_pointers](./smart_pointers/) - [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- 16 - [fearless_concurrency](./fearless_concurrency/) - [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

## GeekTime Rust First Class - Table of Content

### Get hands dirty

- 01 - [httpie](./httpie/): a command-line tool similar to curl
- 02 - [thumbor](./thumbor/): a image processing server

### Basics

- 01 - [Ownership and Borrow](./ownership-and-borrow/)
- 02 - [LifeTime](./lifetime/)
- 03 - [`Cow<T>` demos](./cow_demos/)

## Personal Notebooks

- 01 - [Differences between associated types and generics]()
