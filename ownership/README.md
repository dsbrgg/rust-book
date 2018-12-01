# Ownership

Rust ownership is a feature unique to this language.
It makes memory safety guarantees without needing a garbage collector.

All programs must manage the computer's memory while running. Some languages
have garbage collection that will constantly look for no longer used memory.
Other languages the programmer has to explicitly allocate and free memory.

In Rust uses ownership which is a set of rules that the compiler checks at compile time.
None of the features from ownership slows down your program while running.

Before diving too deep at this, let's look at other concepts...

## The Stack and the Heap