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

Both the stack and the heap are parts of memory that are available to your code to use
at runtime, but structered in diffent ways. Stack stores values in the order it gets them
and removes them in the opposite order (*last-in*, *first-out*). Adding cata is the same as
*pushing onto the stack* and removing data is *popping off the stack*.

The stack is fast because it never has to search for a place to put new data or a place to
get data because the place is **always on top**. Another thing is that all data in the stack
must take up a known, fixed size.

Data with unknown size at compile time or size that might change is **stored on the heap** intead. When you put data on the heap, you ask for some amount of space; the operating system
finds an empty spot somewhere that is big enough, **marks it as in use** and returns a *pointer* which is the address of that location. Because the pointer is a known, fixed size,
you can store the pointer on the stack but to get the actual data, you have to follow the pointer.

>  Heap analogy -

> Think of being seated at a restaurant. When you enter, you state the number of people in your > group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.
> Consider a server at a restaurant taking orders from many tables. It’s most efficient to get >all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). 
> Allocating a large amount of space on the heap can also take time.

Accessing data in the heap is slower because you have to follow a pointer to get to the data.
Contemporary processor are faster if they jump around less in memory.

When calling a function, the values passed to it and potentially pointers to data on the heap,
as well as the function's local variables, they are all pushed onto the stack.
When the function is done, all the values are popped off of the stack.

These memory problems of track data from parts of the code that should be stacked on the heap,
minimizing the amount of duplicate data on the heap and cleaningup unused data on it so you don't
run out of space are all problems addressed on the ownership feature.

## Ownership Rules