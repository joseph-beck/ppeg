//! PPEG meta defines the meta grammar that can be used to write grammars.
//! The meta grammar is translated into a grammar that the parser uses to parse an input.
//!
//! Character
//! - A single character that must be matched for the rule to progress the input.
//! - For example the rule `char_c = { 'c' }` matches the character 'c'.
//!
//! Sequence
//! - A series of rule that must be in order for the rule to match.
//! - For example the rule `sequence = { char_c, char_c }` matches character 'c' followed by character 'c'.
//!
//! Choice
//! - A series of rules that any one of them can match for the rule to match.
//! - For example the rule `choice = { char_c | 'b' }` matches either character 'c' or character 'b'.
//!
//! Zero or more
//! - A rule that can match zero or more times.
//! - For example the rule `zero_or_more = { char_c* }` matches zero or more characters 'c'.
//!
//! One or more
//! - A rule that must match one or more times.
//! - For example the rule `one_or_more = { char_c+ }` matches one or more characters 'c'.
//!
//! Named rule
//! - A rule that matches the result of another rule.
//! - For example the rule `named_rule = { char_c }` matches the result of the rule `char_c`.
//! - This is more powerful when used in other rules, for example `sequence = { char_c, named_rule }`.
//! - Which matches with `char_c` twice.
//!
//! An example using this meta grammar to define some rules of a grammar.
//! ```txt
//! // A character rule that matches the character 'c'.
//! char_c = { 'c' }
//!
//! // The sequence of characters that must be matched in order for the rule to match.
//! // In this case must match with character 'c' followed by character 'c'.
//! sequence = { char_c, char_c }
//!
//! // The pipe symbol indicates that the given rule can match either the left or right side of the pipe.
//! // Trying to match with the left side first, then right side if the left side fails.
//! choice = { char_c | sequence }
//!
//! // The asterisk indicates that the given rule can match zero or more times.
//! zero_or_more = { char_c* }
//!
//! // The plus sign indicates that the given rule must match one or more times.
//! one_or_more = { char_c+ }
//!
//! // Takes the name of the rule to match
//! named_rule = { char_c }
//!
//! // A more complex rule that is matches with 'a' or a sequence of 'b' followed by 'c'.
//! complex = { 'a' | { 'b', char_c } }
//! ```
