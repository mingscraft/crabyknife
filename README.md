# 🦀🗡️🧰crabyknife

crabyknife is a curated collection of small, sharp, and practical Rust-powered command-line tools. Each binary in this repository is designed to solve a specific problem efficiently — think of it as your lightweight utility belt for day-to-day development, scripting, or system tasks.

Whether it's parsing, formatting, inspecting, or transforming data, crabyknife has your back with fast and reliable CLI programs written in safe, modern Rust.

# 🔧 Features
- 📦 Multiple standalone binaries in crate package. 
- ⚡ Fast startup and execution (Rust native).
- 🧪 Tested and modular utilities.
- 🧰 Easy to install all tools at once with `cargo install --path .`


# 📦 Installation 
```
cargo install --path .
```

# 🧰 Included Tools
This repository currently includes the following Rust-based CLI tools:

## 🧼 prettify-xml
Format and indent raw or minified XML input to make it human-readable.

- 📥 Input: Raw XML via file or stdin
- 📤 Output: Pretty-printed XML with consistent indentation
- 🛠️ Great for debugging, version control diffs, or cleaning API responses
- ✅ Handles DOCTYPE, CDATA, comments, and attributes gracefully

### Example:

```
cat messy.xml | prettify-xml > clean.xml
```

## 🆕 new-uuid
Generate fresh, RFC-compliant UUIDs from the command line.

🔢 Supports: UUIDv4 (random)

🧪 Useful for scripting, databases, testing

### Example:

```
new-uuid
```
