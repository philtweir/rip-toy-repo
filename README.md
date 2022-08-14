# Toy repo to reverse in place

Please note, this was written offline with the local Rust docs,
so it is not expected to be in any way sensible or idiomatic,
and I haven't gone and checked how it should be done yet -
just practicing a fun puzzle.

(thanks to @sigma for the mem::swap and split_at_mut tips!)

There are a bunch of shortcomings, such as:
 * does not handle grapheme clusters (but does handle multibyte chars)
 * separators must be a single char (and not '|')
 * sentences and paragraphs treat whitespace naively
 * it's not very efficient, in any other way than keeping a single copy
