[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#https://github.com/varazone/template)
[![CI](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=github&label=CI)](https://github.com/gear-foundation/dapp-template/actions/workflows/ci.yml)

# Gear Template Contract

<!-- Description starts here -->

A template application. Use this repository as a template when creating a new application repository.

> https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template

<!-- End of the description -->

## Initial checklist after creating a new repository

- [ ] Change the app name in `Cargo.toml`.
- [ ] Fix badges' links in `README.md` (replace `gear-foundation/dapp-template` with `<your_username>/<your_new_app>`).
- [ ] Replace a description in `README.md`.
- [ ] Remove this section.
### 🏗️ Building

```sh
cargo b --workspace
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t --workspace -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t --workspace
```

### 🚀 Run CI locally (should be done before a commit)
```sh
cargo xtask ci
```

# License

The source code is licensed under the [MIT license](LICENSE).
