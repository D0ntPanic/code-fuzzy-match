//! Fuzzy string matching inspired by [Visual Studio Code](https://github.com/microsoft/vscode).
//!
//! The fuzzy matching algorithm used in this crate is optimized for use cases such as
//! command palettes, quick file navigation, and code searching. It does not use Levenshtein
//! distance, which is more suited to use cases like spell checking.
//!
//! The algorithm only allows matches where the characters in the query string are present and
//! in the same order as the characters in the target string. All queries are substring queries,
//! so it is not a major hit to the match score to search for a term in the middle of the target
//! string. The algorithm prefers matches that are at the beginning of words in the target
//! string, with words treated as they might appear in code (letters following a separator or
//! in camel case are treated as a word). Sequential matches are also favored.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Fuzzy match a string against a query string. Returns a score that is higher for
/// a more confident match, or `None` if the query does not match the target string.
///
/// # Examples
///
/// ```
/// let matches = code_fuzzy_match::fuzzy_match("the quick brown fox", "bro fox");
/// assert!(matches.is_some());
/// let no_match = code_fuzzy_match::fuzzy_match("the quick brown fox", "cat");
/// assert!(no_match.is_none());
///
/// let high_score = code_fuzzy_match::fuzzy_match("Example string", "example");
/// let lower_score = code_fuzzy_match::fuzzy_match("Example string", "str");
/// assert!(high_score.unwrap() > lower_score.unwrap());
/// ```
pub fn fuzzy_match(target: &str, query: &str) -> Option<usize> {
    // Break the target string into a vector of characters, since we need to manage
    // parallel vectors with information per character.
    let target_chars = target.chars().collect::<Vec<char>>();

    // Create vectors holding the score and sequential counts for two query characters.
    // This algorithm implements a matrix-based method of fuzzy matching, but we don't
    // need to hold the entire matrix in memory, just the current and previous rows.
    let mut prev_seq_match_counts = Vec::new();
    let mut prev_score: Vec<usize> = Vec::new();
    prev_seq_match_counts.resize(target_chars.len(), 0);
    prev_score.resize(target_chars.len(), 0);

    let mut seq_match_counts = Vec::new();
    let mut score: Vec<usize> = Vec::new();
    seq_match_counts.resize(target_chars.len(), 0);
    score.resize(target_chars.len(), 0);

    let mut first_possible_target_idx: usize = 0;

    // Compute match scores for each query character in sequence
    let mut first_query_char = true;
    for query_char in query.chars() {
        // If the starting point of the search is beyond the end of the target string,
        // we can't have a match.
        if first_possible_target_idx >= target_chars.len() {
            return None;
        }

        // Reset vector holding the score and sequential counts for this query character.
        // This algorithm implements a matrix-based method of fuzzy matching, but we don't
        // need to hold the entire matrix in memory, just the current and previous rows.
        seq_match_counts.fill(0);
        score.fill(0);

        let mut first_nonzero_score = None;
        let mut prev_is_separator = false;

        // Compute match scores for each target character in sequence, for this query character.
        // Start at the character after the previous earliest character that had a score. Any
        // character before that cannot have a score, so we don't need to check those.
        for i in first_possible_target_idx..target_chars.len() {
            // Get characters and the score for the previous character in the target
            let target_char = target_chars[i];
            let target_separator = matches!(target_char, '_' | '-' | '.' | ' ' | '\'' | '"' | ':');
            let prev_target_score = if i == 0 { 0 } else { score[i - 1] };

            // Previous score and sequential match count comes from the previous character
            // in both the target and the query
            let prev_query_score = if i == 0 { 0 } else { prev_score[i - 1] };
            let seq_match_count = if i == 0 {
                0
            } else {
                prev_seq_match_counts[i - 1]
            };

            if !first_query_char && prev_query_score == 0 {
                score[i] = prev_target_score;
                prev_is_separator = target_separator;
                continue;
            }

            // Check to ensure the characters match at all. Treat slashes and backslashes
            // as the same character to be able to use as a path matching function.
            let char_matches = match target_char {
                '/' => matches!(query_char, '/' | '\\'),
                '\\' => matches!(query_char, '/' | '\\'),
                _ => target_char
                    .to_lowercase()
                    .zip(query_char.to_lowercase())
                    .all(|(a, b)| a == b),
            };
            if !char_matches {
                // No match, use existing score and reset sequential count
                score[i] = prev_target_score;
                prev_is_separator = target_separator;
                continue;
            }

            // Compute score for this character match. These bonuses are inspired by
            // the algorithm used by Visual Studio Code.
            let mut char_score = 1;

            // Sequential match bonus
            char_score += seq_match_count * 5;

            if target_char == query_char {
                // Same case bonus
                char_score += 1;
            }

            if i == 0 {
                // Start of target bonus
                char_score += 8;
            } else {
                if matches!(target_char, '/' | '\\') {
                    // Path separator bonus
                    char_score += 5;
                } else if target_separator {
                    // Separator bonus
                    char_score += 4;
                } else if target_char.is_uppercase() || prev_is_separator && seq_match_count == 0 {
                    // Start of word bonus
                    char_score += 2;
                }
            }

            prev_is_separator = target_separator;

            // Compute new score and check if it's improved
            let new_score = prev_query_score + char_score;
            if new_score >= prev_target_score {
                // Score is at least the previous score, keep sequential match going
                score[i] = new_score;
                seq_match_counts[i] = seq_match_count;
                if first_nonzero_score.is_none() {
                    first_nonzero_score = Some(i);
                }
            } else {
                // Score is lower than the previous score, don't use this match
                score[i] = prev_target_score;
            }
        }

        if let Some(first_nonzero_score) = first_nonzero_score {
            // Keep scores and sequential match information for this character in the query
            // for lookup during the next character.
            prev_score.copy_from_slice(&score);
            prev_seq_match_counts.copy_from_slice(&seq_match_counts);
            first_query_char = false;

            // Start the next character's matching at the character following the one that
            // first set a valid score.
            first_possible_target_idx = first_nonzero_score + 1;
        } else {
            // If the all scores are zero, we already know we don't have a match. Exit early
            // in this case.
            return None;
        }
    }

    // Final score will always be in the last slot of the final score vector
    let score = *prev_score.last().unwrap_or(&0);
    if score == 0 {
        // Score of zero is not a match
        None
    } else {
        Some(score)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    #[test]
    fn test_match() {
        let result = crate::fuzzy_match("The quick brown fox jumps over the lazy dog.", "fox");
        assert!(result.is_some());
        let result = crate::fuzzy_match(
            "The quick brown fox jumps over the lazy dog.",
            "Quick fox jumps the dog",
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_no_match() {
        let result = crate::fuzzy_match("The quick brown fox jumps over the lazy dog.", "cat");
        assert!(result.is_none());
        let result = crate::fuzzy_match(
            "The quick brown fox jumps over the lazy dog.",
            "Quick fox jumps the cat",
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_ranking() {
        const TARGET: &str = "The quick brown fox jumps over the lazy dog.";
        const QUERIES: &[&str] = &[
            "fx",           // Match short word in the middle, omitted letter
            "fox",          // Match short word in the middle
            "jump",         // Match word in the middle
            "JUMP",         // Match word but not a case match, lower than "jump"
            "The",          // Short match on first word
            "the",          // Matches first word but not a case match, lower than "The"
            "fx over",      // Match with omitted letter
            "quick cat",    // Not a match, last word not present
            "The quick",    // Long case match at the start, this should be near the top
            "the quick",    // Long match at the start, this should be just below "The quick"
            "jump the dog", // Long match, highest because of three exact word matches
            "jmp the do",   // Match, but not as high as "jump the dog"
            "jmp the cat",  // Not a match, last word not present
            "dog the fox",  // Not a match, out of order
            "het",          // Matches part of "the" and then a later "t" but should be low rank
            "xz",           // Letters are in order but not related, low rank
            "xx",           // Not a match, letter is present but only once
            "ee",           // Match, letter is present twice in the target
        ];

        // Gather results for each query
        let mut results = QUERIES
            .iter()
            .map(|query| (query, crate::fuzzy_match(TARGET, query)))
            .collect::<Vec<_>>();

        // Get rid of anything that isn't a match
        results.retain(|(_, result)| result.is_some());

        // Sort by score
        results.sort_by_key(|(_, result)| result.unwrap());

        // Validate results
        assert_eq!(
            results.iter().map(|(query, _)| *query).collect::<Vec<_>>(),
            &[
                &"xz",
                &"ee",
                &"fx",
                &"JUMP",
                &"fox",
                &"het",
                &"jump",
                &"the",
                &"The",
                &"fx over",
                &"the quick",
                &"The quick",
                &"jmp the do",
                &"jump the dog",
            ]
        );
    }

    #[test]
    fn test_slash() {
        let result = crate::fuzzy_match("/bin/ls", "/ls");
        assert!(result.is_some());
        let result = crate::fuzzy_match("/bin/ls", "\\ls");
        assert!(result.is_some());
        let result = crate::fuzzy_match("c:\\windows\\notepad.exe", "/windows");
        assert!(result.is_some());
        let result = crate::fuzzy_match("c:\\windows\\notepad.exe", "\\windows");
        assert!(result.is_some());
    }

    #[test]
    fn test_word_bonus() {
        let higher = crate::fuzzy_match("words with spaces", "wit");
        let lower = crate::fuzzy_match("words with spaces", "ith");
        assert!(higher.is_some());
        assert!(lower.is_some());
        assert!(
            higher.unwrap() > lower.unwrap(),
            "higher = {:?}, lower = {:?}",
            higher,
            lower
        );

        let higher = crate::fuzzy_match("words_with_underscores", "wit");
        let lower = crate::fuzzy_match("words_with_underscores", "ith");
        assert!(higher.is_some());
        assert!(lower.is_some());
        assert!(
            higher.unwrap() > lower.unwrap(),
            "higher = {:?}, lower = {:?}",
            higher,
            lower
        );

        let higher = crate::fuzzy_match("camelCaseWords", "Cas");
        let lower = crate::fuzzy_match("camelCaseWords", "ase");
        assert!(higher.is_some());
        assert!(lower.is_some());
        assert!(
            higher.unwrap() > lower.unwrap(),
            "higher = {:?}, lower = {:?}",
            higher,
            lower
        );
    }
}