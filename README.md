# King Triton's Ocean

In this assignment, you will be implementing some unfinished Rust code which models a mythical ocean ecosystem populated by crabs that live on beaches, and various prey they hunt in reefs.

These crabs have names, and come in any and all colors. Groups of them live together (and reproduce) at beaches along the coast line. Once the sun sets, they head out to their favorite reefs to hunt for food that matches their tastes. Some crabs only eat fish, some only shellfish, while yet others are vegetarian.

Luckily, the reefs dotting this ocean are populated with plenty of prey that fit into each crab's dietary preferences. However, finding the right reef with the right food can prove a challenge.

It's a tough life for a crab out there. 

## Learning Goals

By the end of this assignment you should have gained some hands-on experience the basics of the Rust language, as well as some features peculiar to its ownership model.

  - Functions
  - Structs
  - Methods
  - Trait Objects
  - Mutable vs Immutable References
  - Smart Pointers
    - Single Ownership (Boxes)
    - Multiple Ownership (Reference-counted Boxes)

This assignment is not intended to be algorithmically challenging, but rather to be something of a guided tour through some of Rust's distinct characteristics. No individual task should be too complicated to implement, and almost all have corresponding tests you can use to check your progress as you work.

# Instructions

**If you are unsure about how to do something in Rust, you should probably first consult the [Rust Book](https://doc.rust-lang.org/book/).**

## Submission

**You should only modify and submit the following files:** 
  - __`beach.rs`__
  - __`color.rs`__
  - __`crab.rs`__
  - __`ocean.rs`__
  - __`reef.rs`__

Any changes made to other files will be **ignored** by the autograder, which will always use the originals.

## Running Tests

You can run all tests with `cargo test`, and all tests with a given prefix with `cargo test <prefix>`. 

To make things easy, all of the tests in this assignment are named according to the following convention:

```sh
cargo test part${n}_${entity_name}_${tested_behavior}
```

For example, to run all tests that relate to `Crab` from Part 1, you can simply run:

```sh
cargo test part1_crab 
```

By default, `cargo` _captures_ the console output of your code while running tests. However, while working on this assignment you might want to use `print!` or `println!` while running your tests to debug. You can do this by passing the `--nocapture` flag while running tests after `--`, which separates test names from flags.

For example, to run tests involving the crab hunting tasks in Part 2 while showing console output:

```sh
cargo test part2_crab_hunt -- --nocapture
```

**You should read the tests carefully. They may serve as examples of unfamiliar syntax, and even provide some useful insights into how to implement some of the later tasks.**

## Writing Tests

You should not modify the contents of the `tests/public.rs` file. 

However, you may find it useful to write your own tests in `tests/student.rs`.

These tests will be ignored by the autograder, so don't worry if they don't pass. 

# Part 1. Basic

We will first implement a simple model of a `Crab`, and then a `Beach` which contains zero or more crabs. In this manner, we demonstrate how to define and work with single objects as well as collections in Rust. We will also implement a function for breeding crabs.

For this part, you will not have to worry about lifetimes, and ownership should be straightforward: just simple borrows and dereferences as shown in lectures. 

## Tasks

### Simple Functions and Methods

First, implement the missing bits in __`crab.rs`__ and __`color.rs`__:
  - __`color.rs`__
    - The `cross` function, which crosses two colors by performing wrapped arithmetic.
    - Refer to the specification in the function's doc string for more details. Note that Rust, being a "safe" language, handles arithmetic operations much more strictly than you may be used to, and the "obvious" solution here is not the correct one and will panic.
    - [Documentation for Primitive Type u8](https://doc.rust-lang.org/std/primitive.u8.html)
  - __`crab.rs`__
    - A `new` function according to the given signature.
    - The `name`, `speed`, `color` and `diet` fields on `Crab` and corresponding accessor functions.

You should now have the following tests passing:
  - `part1_color_cross_no_panic`
  - `part1_crab_new`

### Crabs & Braches

Then, implement the missing functions in __`beach.rs`__:

  - A `new` function according to the given signature.
  - `size`: returns the number of `Crab`s on the beach.
  - `add_crab`: adds a `Crab` to the beach.
  - `get_crab`: returns a reference to a `Crab` at the given index.
  - `get_fastest_crab`: returns `None` if the beach is empty. Otherwise, return `Some` of a reference to the `Crab` with the highest `speed`.
  - `breed_crabs`: uses the functions in `genetics` to breed two crabs, resulting in a new `Crab`. 
    - You will want to add a new `breed` function to `Crab` to avoid exposing `Crab` implementation details. 
    - The new `Crab` should have its diet selected randomly using the provided `Diet::random_diet`.
    - The new `Crab` should have its color computed by `Color::cross` from before.
    - The new `Crab` should have a speed of `1` (babies go slowly).

The tests for these functions are:
  - `part1_beach_add_get_crab`
  - `part1_beach_iter_crabs`
  - `part1_beach_size`
  - `part1_beach_fastest_empty`
  - `part1_beach_fastest_fastest_first`
  - `part1_beach_fastest_fastest_second`
  - `part1_crab_breeding`

Lastly, implement the remaining method in __`crab.rs`__:

  - `find_crabs_by_name`: returns a vector of references to crabs with the given name.

When you're done, the test `part1_crab_find_by_name` should pass.

## Notes

  - If you are accustomed to functional-style programming, you may find methods on [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html) useful or more natural to implement `get_fastest_crab`, `find_crabs_by_name`, and some functions in the next part. 


# Part 2. Advanced

**While working on this section, you should consult some sections of the Rust Book for guidance and examples.**

We will make use of _trait objects_ to represent `Prey` that `Crab`s go out and hunt. Trait objects are often used in similar places to interfaces, abstract classes in other languages like Java, Go, and C++, though they differ in some ways. They allow us to refer to _something_ which implements a given trait.

  - [17.2 Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

We will also make use of _smart pointers_ to heap allocations: both single-ownership `Box<T>`, and multiple-ownership (reference-counted) `Rc<T>`. The referents of smart pointers in Rust are _immutable_. Therefore, in order to represent a pointer to something _mutable_, we use `Box<RefCell<T>>` or `Rc<RefCell<T>>`. This is a common, _but cumbersome_, design pattern in Rust that results from some of the choices made in the design of the language.

  - [15.1 Box\<T\>](https://doc.rust-lang.org/book/ch15-01-box.html)
  - [15.4 Rc\<T\>](https://doc.rust-lang.org/book/ch15-04-rc.html)
  - [15.5 RefCell\<T\> and _interior mutability_](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

## Things to Note in the Starter Code

In __`prey.rs`__ note (but not modify) the definitions: 

  - `pub trait Prey`: defines the interface which all `Prey` must implement.
    - The `diet` method returns the kind of `Diet` this `Prey` is compatible with.
    - The `try_escape` method returns whether or not this `Prey` can escape from a given `Crab`, in a given `Reef`.
      - This method is side-effecting, and can therefore mutate the `Prey` it is called on, such as with the `Shrimp`. 
  
In __`reef.rs`__, you should take note of a few facts:

  - A `VecDeque` is Rust's standard library queue structure. 
    - It can be used as a double-ended queue, but here we are only using it as a single-ended queue.
  - We had to declare the `prey` field of `struct Reef` using the `dyn` keyword.
    - However, `VecDeque<dyn Prey>` would not work, because the size in memory of `dyn Prey` is not known at compile time. Therefore, we use `Box<dyn Prey>`, which represents a pointer to _something_ implementing `Prey` on the heap.
    

## Tasks 

### Lifetimes with Recipes

First, let's look at a simple example of where explicit lifetime annotations are needed. It turns out that our crabs like to cook their food before they eat it (this is a _mythical_ ocean, remember?). In __`crab.rs`__ implement:

  - `choose_recipe`: returns a recipe from the `Cookbook` that matches the `Crab`'s diet.
    - This is the _only_ case in this assignment where you will need to modify the signature of a method.
    - You should _only_ add lifetime annotations.

### Implementing Reefs

Next, let's implement the missing methods on __`reef.rs`__. A `Reef` is a wrapper around a queue of `Box<dyn Prey>`. We use a queue here as we will later have a `Crab` take `Prey` from the `Reef`, and potentially put it back. Using a queue removes the need to keep track of indices and supports both fast insertion and removal at the ends.

  - `new`
  - `add_prey`: push prey to the _back_ of the reef.
  - `take_prey`: pop prey from the _front_ of the reef.

At this point the test `part2_reef_add_take_prey` should pass.

Next, add a `reefs` field to `struct Crab` with an appropriate type. Consult `discover_reef` for some ideas. Note that multiple crabs can share the same `Reef`, and both can mutate it. We represent a reference-counter/multiply-owned pointer with `Rc`, and a cell containing a mutable reference with `RefCell`. Put together: `Rc<RefCell<Reef>>`. 

Then, in __`crab.rs`__, implement:

  - `discover_reef`: add a reef to a `Crab`'s list of reefs.
    - There is no corresponding `forget_reef`, but you are welcome to add one if it helps you write your own tests. We will ignore it.

At this point the test `part2_crab_discover_reefs` should pass.


### Catching Prey and Going Hunting

Crabs in this ocean hunt in a very particular way. They are greedy, but they will never try to catch the same prey twice. In this manner, they avoid getting stuck chasing the same uncatchable prey all day long. When a `Crab` catches `Box<dyn Prey>`, it _removes it from the reef_, since a `Box` can only have one owner.

  - `catch_prey`: catch some `Prey` from one of the `Crab`'s reefs and return `Some` of it and the index of the `Reef` it came from, or `None` if none can be found.
    - Consult the docstring for specific details.
    - In order to call `take_prey` on one of the `Reef`s stored in a `Rc<RefCell<Reef>>`, you will need to _borrow mutably_ from it.
      - Consult the documentation and relevant Rust book chapter to figure out which method to use.
      - You may also find it helpful to look at some of the tests.
  - `release_prey`: release `Prey` back into the `Reef` at the given index.
  - `hunt`: implements the hunting behavior of the crab. 
    - Consult the docstring for full pseudocode for this method, as it is a bit more involved than any so far.

Note that the latter of these two methods are _not public_. They are implementation details, and helper functions to implement `hunt`. 

You should now have the following tests passing:

  - `part2_crab_hunt_empty_reef`
  - `part2_crab_hunt_escaped_prey`
  - `part2_crab_hunt_incompatible`
  - `part2_crab_hunt_success`

### The Ocean Itself

So far, you've been tasked with using smart pointers from the callee side. That is, you have always received them from elsewhere, but have not had to construct them or in the case of `Rc` take additional references to them yourself using `Rc::clone`. 

We will now define `Ocean`, which provides an interface to many of the definitions implemented so far. 

First, in __`ocean.rs`__, add appropriate struct fields to back the `beaches` and `reefs` accessors. Then, implement the missing methods:

  - `new`
  - `add_beach`: adds a beach to this `Ocean`
  - `beaches`: returns an iterator over this `Ocean`'s beaches.
  - `reefs`: returns an iterator over this `Ocean`'s reefs.
  - `generate_reef`: generates a reef with the number of each type of prey specified in the arguments, adds a reference to it to this `Ocean`'s reefs, _and returns a reference_.
    - This will require creating both kinds of smart pointers used so far (`Box`, `Rc`) as well as `RefCell`.
    - Note that you will need to get a _second_ reference to an `Rc` here as well, since you need one to add to `reefs`, and one to return.

**Tip**: start by considering `Algae` only, getting `part2_ocean_generate_algae_only` passing.

You should now have the following tests passing:

 - `part2_ocean_generate_algae_only`
 - `part2_ocean_generate_bountiful`
 
 # Submitting your Work

 That's it! You're done!

 You should double check your tests all pass by ensuring all files have been saved and then running `cargo clean` and `cargo test`.

 You should submit the following files to Gradescope:
  - __`beach.rs`__
  - __`color.rs`__
  - __`crab.rs`__
  - __`ocean.rs`__
  - __`reef.rs`__