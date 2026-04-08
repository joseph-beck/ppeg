//! Parser module contains all the components to build a PEG parser.
//!
//! This module is comprised of:
//! - context: stores information about parser state and used throughout parsing.
//! - cst: stores the concrete syntax tree (CST) generated during parsing.
//! - error: parsing failures.
//! - expression: all expression types used to build PEGs.
//! - grammar: stores all rules.
//! - history: stores parsing history, as nodes, that are used to parse left recursive expressions.
//! - packrat: stores the packrat memoization table that prevents super linear parse times.
//! - peg: main PEG parser logic using other components in this module.
//! - rule: stores name against an expression, used to build grammars.

pub mod context;
pub mod cst;
pub mod error;
pub mod expression;
pub mod grammar;
pub mod history;
pub mod packrat;
pub mod peg;
pub mod rule;
