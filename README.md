Fuzzy string matching for code
==============================

[<img alt="crates.io" src="https://img.shields.io/crates/v/code-fuzzy-match">](https://crates.io/crates/code-fuzzy-match)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/code-fuzzy-match">](https://docs.rs/code-fuzzy-match)

Fuzzy string matching inspired by [Visual Studio Code](https://github.com/microsoft/vscode).

The fuzzy matching algorithm used in this crate is optimized for use cases such as
command palettes, quick file navigation, and code searching. It does not use Levenshtein
distance, which is more suited to use cases like spell checking.

The algorithm only allows matches where the characters in the query string are present and
in the same order as the characters in the target string. All queries are substring queries,
so it is not a major hit to the match score to search for a term in the middle of the target
string. The algorithm prefers matches that are at the beginning of words in the target
string, with words treated as they might appear in code (letters following a separator or
in camel case are treated as a word). Sequential matches are also favored.

## Example usage

```rust
let mut matcher = code_fuzzy_match::FuzzyMatcher::new();
let matches = matcher.fuzzy_match("the quick brown fox", "bro fox");
assert!(matches.is_some());
let no_match = matcher.fuzzy_match("the quick brown fox", "cat");
assert!(no_match.is_none());

let high_score = matcher.fuzzy_match("Example string", "example");
let lower_score = matcher.fuzzy_match("Example string", "str");
assert!(high_score.unwrap() > lower_score.unwrap());
```
