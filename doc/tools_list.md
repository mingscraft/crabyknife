# Full list of tools

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
