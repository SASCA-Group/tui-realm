# tui-realm

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/tui-realm.svg)](https://github.com/veeso/tui-realm) [![Downloads](https://img.shields.io/crates/d/tuirealm.svg)](https://crates.io/crates/tuirealm) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/tuirealm) [![Docs](https://docs.rs/tuirealm/badge.svg)](https://docs.rs/tuirealm)  

[![Build](https://github.com/veeso/tui-realm/workflows/Linux/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/MacOS/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/Windows/badge.svg)](https://github.com/veeso/tui-realm/actions) [![codecov](https://codecov.io/gh/veeso/tui-realm/branch/main/graph/badge.svg?token=au67l7nQah)](https://codecov.io/gh/veeso/tui-realm)

Developed by Christian Visintin  
Current version: 0.1.0 (20/04/2021)

---

- [tui-realm](#tui-realm)
  - [About tui-realm 👑](#about-tui-realm-)
    - [Why tui-realm 🤔](#why-tui-realm-)
  - [Get started 🏁](#get-started-)
    - [Add tui-realm to your Cargo.toml 🦀](#add-tui-realm-to-your-cargotoml-)
    - [Create a tui-realm application](#create-a-tui-realm-application)
    - [Run examples](#run-examples)
  - [Standard components library 🎨](#standard-components-library-)
  - [Guides 🎓](#guides-)
  - [Documentation 📚](#documentation-)
  - [About other backends](#about-other-backends)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [Buy me a coffee ☕](#buy-me-a-coffee-)
  - [License 📃](#license-)

---

## About tui-realm 👑

tui-realm is a **framework** for [tui](https://github.com/fdehau/tui-rs) to simplify the implementation of terminal user interfaces adding the possibility to work with re-usable components with properties and states, as you'd do in React But that's not all: the components communicate with the ui engine via a system based on **Messages** and events, providing you with the possibility to implement `update` functions as happens in Elm. In addition, the components are organized inside **Views**, which manages mounting/umounting and focus for you.

And that's also explains the reason of the name: Realm stands for React and Elm.

Tui-realm also comes with a built-in standard library of components you may find very useful. Don't worry, they are optional if you don't want to use them 😉, just follow the guide in [get started](#get-started-).

### Why tui-realm 🤔

Personally I didn't start this project from scratch. I've just decided to make a library out of the already existing code in [termscp](https://github.com/veeso/termscp), which I had just finished at the time I started this project. I thought this library could have come handy for somebody.

You might be wondering now how much is this project influenced by the development of termscp. Well, a lot actually, I won't deny this, so don't expect this library to always try to fit the community needs, I'm just providing you with a tool I've made for myself, but that I wanted to share with the community.

---

## Get started 🏁

⚠ Warning: tui-realm works only with **crossterm** as backend ⚠

### Add tui-realm to your Cargo.toml 🦀

```toml
tuirealm = "0.1.0"
```

or if you want to include the [standard component library](#standard-component-library-)...

```toml
tuirealm = { "version" = "0.1.0", features = [ "with-components" ] }
```

Since this library requires `crossterm` too, you'll also need to add it to your Cargo.toml

```toml
crossterm = "0.19.0"
```

You don't need tui as dependency, since you can access to tui via `tuirealm::tui::*`

### Create a tui-realm application

View how to implement a tui-realm application in the [related guide](docs/get-started.md).

### Run examples

Still confused about how tui-realm works? Don't worry, try with the examples:

- [demo](examples/demo.rs): a simple application which shows how tui-realm works

    ```sh
    cargo run --features="with-components" --example demo
    ```

- [termscp](https://github.com/veeso/termscp): real production implemenetation of tui-realm; just browse the `src/ui/` folder.

---

## Standard components library 🎨

Tui-realm comes with an optional standard library of components I thought may be useful for most of the applications.
If you want to use it, just enable the `with-components` feature in your `Cargo.toml`.

For each component, the standard library provides a `PropsBuilder` in the same module (e.g. `input::Input => input::InputPropsBuilder`), which provides methods to set only the properties actually used by the component.

To have an overview of the components just run the gallery example 🦄

```sh
cargo run --features="with-components" --example gallery
```

If you want a super-detailed guide about components check out the [components guide](docs/std-components.md).

---

## Guides 🎓

- [Get Started Guide](docs/get-started.md)
- [The UI lifecycle](docs/lifecycle.md)
- [Standard Library Components](docs/std-components.md)
- [Implement components](docs/new-components.md)

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tui-realm>

---

## About other backends

As you've probably already noticed, tuirealm only supports `crossterm` as backend for the terminal, even if `tui` supports `termion` and other libraries. Why this?
Well the reasons are these two:

1. There's no reason to use the other backends: I use crossterm in termscp, and I don't find any advantage in using termion or other backends. Crossterm is cross platform and works perfectly fine.
2. Implementing the support for the other backends would force me in creating a mapper for input events from the different backends into a common type. Is it possible? Yes it is, but I'm really not interested in implementing it.

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tui-realm's changelog [HERE](CHANGELOG.md)

---

## Buy me a coffee ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

[![Buy-me-a-coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## License 📃

tui-realm is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
