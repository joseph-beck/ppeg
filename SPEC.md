# Meta Specification

This document defines the specification of the syntax for defining PEGs.

```txt
// Char expression of 'a'
char := { 'a' }

// Choice between a rule of char or 'b'.
choice := { char | 'b' }

// Sequence of rule char followed by a 'b'.
sequence := { char, 'b' }

// Not the rule char.
not := { !char }

// Optionally the character 'b'.
optional := { 'b'? }

// Zero or more instances of the rule char.
zero_or_more := { char* }

// One or more instances of the char 'b'.
one_or_more := { 'b'+ }

// Named rule pointing to the rule char.
named_rule := { char }
```
