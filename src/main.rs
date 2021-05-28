use std::u32;

fn main() {
    println!("Hello, world!");



    // Structs

    // Rust structs, sometimes called structures, resemble classes in Python, or objects in JS. A struct assembles several values of assorted types together into a single value, so we can deal with them as a unit. Given a struct, you can read and modify its individual components. A struct can have methods associated with it that operate on its components.

    // Rust has three kinds of struct types. Named-field, tuple-like, and unit-like which each differ in how we refer to their components. A named-field struct gives a name to each component, whereas a tuple-like struct identifies them by the order in which they appear. Unit-like structs have no components at all. These are not common, but more useful then you'd think.



    // Named-Field Structs

    // The definition of a named-field struct type looks like this:
    /// A rectangle of eight-bit grayscale pixels.
    struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }

    // This declares a type GrayscaleMap with two fields named pixels and size, of the given types. The convention in Rust is for all types, structs included, to have names that capitalize the first letter of each word, like GrayscaleMap, a convention called CamelCase. Fields and methods are lowercase, with words separated by underscores. This is called snake_case.

    // We can construct a value of this type with a struct expression, like so:
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height)
    };

    // A struct expression starts with the type name (GrayscaleMap), and lists the name and value of each field, all enclosed in curly braces. There's also shorthand for populating fields from local variables or arguments with the same name:
    fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { pixels, size }
    }

    // The struct expression GrayscaleMap { pixels, size } is short for GrayscaleMap { pixels: pixels, size: size }. We can use key: value syntax for some fields and shorthand for others in the same struct expression.

    // To access a struct's fields, use the . operator:
    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    // Like all other items, structs are private by default, visible only in the module where they're declared. We can make a struct visible outside its module by prefixing its definition with pub. The same goes for each of its fields, which are also private by default:
    /// A rectangle of eight-bit grayscale pixels.
    pub struct GrayscaleMap {
        pub pixels: Vec<u8>,
        pub size: (usize, usize)
    }

    // Even if a struct is declared pub, its fields can be private:
    /// A rectangle....
    pub struct GrayscaleMap {
        pixels: ...,
        size: ...
    }

    // Other modules can use this struct and any public methods it might have, but can't access the private fields by name or use struct expressions to create new GrayscaleMap values. That is, creating a struct value requires all the struct's fields to be visible. This is why we can't write a struct expression to create a new String or Vec. These standard types are structs, but all their fields are private. To create one, we must use public methods like Vec::new().

    // When creating a named-field struct value, we can use another struct of the same type to supply values for fields we omit. In a struct expression, if the named fields are followed by .. EXPR, then any fields not mentioned take their values from EXPR, which must be another value of the same struct type. Suppose we have a struct representing a monster in a game:
    struct Broom {
        name: String,
        height: u32,
        health: u32,
        position: (f32, f32, f32),
        intent: BroomIntent
    }

    /// Two possible alternative for what a `Broom` could be working on
    #[derive(Copy, Clone)]
    enum BroomIntent { FetchWater, DumpWater }

    // A novice magician enchants a broom to do his work, but doesn't know how to stop it when the job is done. Chopping the broom in half with an axe just produces two brooms, each of half the size, but continuing the task with the same blind dedication as the original:
    // Receive the input Broom by value, taking ownership.
    fn chop(b: Broom) -> (Broom, Broom) {
        // Initialize `broom1` mostly from `b`, changing only `height`. Since
        // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
        let mut broom1 = Broom { height: b.height / 2, .. b};

        // Initialize `broom2` mostly from `broom1`. Since `String` is not
        // `Copy`, we must clone `name` explicitly.
        let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

        // Give each fragment a distinct name.
        broom1.name.push_str(" I");
        broom2.name.push_str(" II");

        (broom1, broom2)
    }

    // With that definition in place, we can create a broom, chop it in two, and see what we get:
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    }

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
    


    // Tuple-Like Structs

    // The second kind of struct type is called a tuple-like struct, because it resembles a tuple:
    struct Bounds(usize, usize);

    // We construct a value of this type much as we would construct a tuple, except that we must include the struct name:
    let image_bounds = Bounds(1024, 768);

    // The values held by a tuple-like struct are called elements, just as the values of a tuple are. We access them just as we would a tuple's:
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    // Individual elements of a tuple-like struct may be public or not:
    pub struct Bounds(pub usize, pub usize);

    // The expression Bounds(1024, 768) looks like a function call, and in fact it is. Defining the type also implicitly defines a function:
    fn Bounds(elem0: usize, elem1: usize) -> Bounds { ... }

    // At the most fundamental level, named-field and tuple-like structs are very similar. The choice of which to use comes down to questions of legibility, ambiguity, and brevity. If we will use the . operator to get at a value's components much at all, identifying fields by name provides the reader more info, and is probably more robust against types. If we will usually use pattern matching to find the elements, tuple-like structs can work nicely.

    // Tuple-like structs are good for newtypes, structs with a single component that we define to get stricter type checking. For example, if we are working with ASCII-only text, we might define a newtype like so:
    struct Ascii(Vec<u8>);

    // Using this type for our ASCII strings is much better than simply passing around Vec<u8> buffers and explaining what they are in the comments. The newtype helps Rust catch mistakes where some other byte buffer is passed to a function expecting ASCII text. More on this in chapt 21.



    // Unit-Like Structs

    // The third kind of struct is a little obscure. It declares a struct type with no elements at all:
    struct Onesuch;

    // A value of such a type occupies no memory, much like the unit type (). Rust doesn't bother actually storing unit-like struct values in memory or generating code to operate on them because it can tell everything it might need to know about the value from its type alone. But logically, an empty struct is a type with value like any other, or more precisely, a type of which there is only a single value:
    let o = Onesuch;

    // We've already encountered a unit-like struct when reading about "Fields and Elements" in chapt 6. Whereas an expression like 3..5 is shorthand for the struct value Range { start: 3, end: 5 }, the expression .., a range omitting both endpoints, is shorthand for the unit-like struct value RangeFull.

    // Unit-like structs can also be useful when working with traits which is covered in chapt 11.



    // Struct Layout

    // In memory, both named-field and tuple-like structs are the same thing. A collection of values, of possibly mixed types, laid out in a particular way in memory. For example, earlier in the chapter we defined:
    struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }

    // See page 315 for diagram

    // Rust doesn't make specific promises about how it will order a struct's fields or elements in memory. However, Rust does promise to store field's value directly in the struct's block of memory. Whereas JS and Python would put the pixels and size values each in their own heap-allocated blocks and have GrayscaleMap's fields point at them, Rust embeds pixels and size directly in the GrayscaleMap value. Only the heap-allocated buffer owned by the pixels vector remains in its own block.

    // We can ask Rust to lay out structures in a way compatible with C and C++, using the #[repr(C)] attribute, covered in chapt 21.



    // Defining Methods with impl

    // Throughout the book we've been calling methods on all sorts of values. We've pushed elements onto vectors with v.push(e), fetched their length with v.len(), checked Result values for errors with r.expect("msg"), and so on.

    // We can define methods on any struct type we define. Rather than appearing inside the struct definition (C++ or Java), Rust methods appear in a separate impl block:
    /// A last-in, first-out queue of characters.
    pub struct Queue {
        older: Vec<char>, // older elements, eldest last.
        younger: Vec<char> // younger elements, youngest last.
    }

    impl Queue {
        /// Push a character onto the back of a queue.
        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }

        /// Pop a character off the front of a queue. Return `Some(c)` if there
        /// was a character to pop, or `None` if the queue was empty.
        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }

                // Bring the elements in younger over to older, and put them in
                // the promised order.
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }

            // Now older is guaranteed to have something. Vec's pop method
            // already returns an Option, so we're set.
            self.older.pop()
        }
    }

    // An impl block is simply a collection of fn definitions, each of which becomes a method on the struct type named at the top of the block. Here we've defined a public struct Queue, and then given it two public methods, push and pop.

    // Methods are also known as associated functions, since they're associated with a specific type. The opposite is a free function, one that is not defined as an impl block's item.

    // Rust passes a method to the value it's being called on as its first argument, which must have the special name self. Since self's type is obviously the one named at the top of the impl block, or a reference to that, Rust lets you omit the type, and write self, &self, or &mut self as shorthand for self:Queue, self: &Queue, or self: &mut Queue. We can use the longhand forms if we like, but almost all Rust code uses the shorthand way.

    // In the example, the push and pop methods refer to the Queue's fields as self.older and self.younger.A Rust method must explicitly use self to refer to the value it was called on, similar to the way Python methods use self, and JS methods use this.

    // Since push and pop need to modify the Queue, they both take &mut self. However, when we call a method, we don't need to borrow the mutable ref ourself. The ordinary method call syntax takes care of the implicitly. So with these definitions in place, we can use Queue like this:
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    // Simply writing q.push(...) borrows a mutable ref to q, as if we had written (&mut q).push(...), since that's what the push method's self requires.

    // If a method doesn't need to modify its self, then we can define it to take a shared ref instead. For example:
    impl Queue {
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
    }

    // Again, the method call expression knows which sort of ref to borrow:
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());

    // Or, if a method wants to take ownership of self, it can take self by value:
    impl Queue {
        pub fn split(self) -> (Vec<char>, Vec<char>) {
            (self.older, self.younger)
        }
    }

    // Calling this split method looks like the other method calls:
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // q is now uninitialized
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // Since split takes its self by value, this moves the Queue out of q, leaving q uninitialized. Since split's self now owns the queue, it's able to move the individual vectors out of it, and return them to the caller.

    // We can also define methods that don't take self as an argument at all. These become functions associated with the struct type itself, not with any specific value of the type. Following the tradition established by C++ and Java, Rust calls these static methods. They're often used to provide constructor functions like so:
    impl Queue {
        pub fn new() -> {
            Queue { older: Vec::new(), younger: Vec::new() }
        }
    }

    // To use this method, we refer to it as Queue::new. The type name, a double colon, then the method name. Now our example code becomes a bit more svelte:
    let mut q = Queue::new()

    q.push('*');
    ...

    // It's conventional in Rust for constructor functions to be named new. We've already seen Vec::new, Box::new, HashMap::new, and others. But there's nothing special about the name new. It's not a keyword, and types often have other static methods that serve as constructors, like Vec::with_capacity.

    // Although we can have many separate impl blocks for a single type, they must all be in the same crate that defines that type. However, Rust does let us attach our own methods to other types, explained in chapt 11.



    // Generic Structs

    // Our earlier definition of Queue is unsatisfying. It is written to store characters, but there's nothing about its structure or methods that is specific to characters at all. If we were to define another struct that held, say, String values, the code could be identical, except that char would be replaced with String. That would be a waste of time.

    // Fortunately, Rust structs can be generic, meaning that their definition is a template into which we can plug whatever types we like. For example, here's a definition for Queue that can hold values of any type:
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>
    }

    // We can read the <T> in Queue<T> as "for any element type T...". So this definition reads, "For any type T, a Queue<T> is two fields of type Vec<T>". For example, in Queue<String>, T is String, so older and younger have type Vec<String>. In Queue<char>, T is char, and we get a struct identical to the char-specific definition we started with. In fact, Vec itself is a generic struct, defined in just this way.

    // In generic struct definitions, the type names used in <angle brackets> are called type parameters. An impl block for a generic struct looks like this:
    impl<T> Queue<T> {
        pub fn new() -> Queue<T> {
            Queue { older: Vec::new(), younger: Vec::new() }
        }

        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }

        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }

        ...
    }

    // We can read the line impl<T> Queue<T> as something like, "for any type T, here are some methods available on Queue<T>". Then, we can use the type parameter T as a type in the method definitions.

    // We've used Rust's shorthand for self parameters in the preceding code. Writing out Queue<T> everywhere becomes a mouthful and a distraction. As another shorthand, every impl block, generic or not, defines the special type parameter Self (note the CamelCase name) to be whatever type we're adding methods to. In the preceding code Self would be Queue<T>, so we can abbreviate Queue::new's def a bit further:
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    // In the body of new, we didn't need to write the type parameter in the construction expression. Simply writing Queue { ... } was good enough. This is Rust's type inference at work. Since there's only one type that works for that function's return value, namely Queue<T>, Rust supplies the parameter for us. However, we'll always need to supply type parameters in function signatures and type definitions. Rust doesn't infer those. Instead, it uses those explicit types as the basis from which it infers types within function bodies.

    // For static method calls, we can supply the type parameter explicitly using the turbofish ::<> notation:
    let mut q = Queue::<char>::new();

    // In practice, we can usually just let Rust figure it out for us:
    let mut q = Queue::new();
    let mut r = Queue::new();

    q.push("CAD"); // apparently a Queue<&'static str>
    r.push(0.74); // apparently a Queue<f64>
    q.push("BTC"); // Bitcoins per USD, 2017-5
    r.push(2737.7); // Rust fails to detect irrational exuberance

    // Vec is an example of another generic struct type. Also, it's not just structs that can be generic. Enums can take type parameters as well, with very similar syntax. Details later.



    // Structs with Lifetime Parameters

    // As discussed in "Structs Containing References" in chapt 5, if a struct type contains references, we must name those references' lifetimes. For example, here's a structure that might hold references to the greatest and least elements of some slice:
    struct Extrema<'elt> {
        greatest: &'elt i32,
        least: &'elt i32
    }

    // Earlier, we said to think of a declaration like struct Queue<T> as meaning that, given any specific type T, we can make a Queue<T> that holds that type. Similarly, we can think of struct Extrema<'elt> as meaning that, given any specific lifetime 'elt, you can make an Extrema<'elt> that holds references with that lifetime.

    // Here's a function to scan a slice and return an Extrema value whose fields refer to its elements:
    fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
        let mut greatest = &slice[0];
        let mut least = &slice[0];

        for i in 1..slice.len() {
            if slice[i] < *least    { least     = &slice[i]; }
            if slice[i] > *greatest { greatest = &slice[i]; }
        }

        Extrema { greatest, least }
    }

    // Since find_extrema borrows elements of slice, which has lifetime 's, the Extrema struct we return also uses 's as the lifetime of its references. Rust always infers lifetime parameters for calls, so calls to find_extrema needn't mention them:
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    // Because it's so common for the return type to use the same lifetime as an argument, Rust lets us omit the lifetimes when there's on obvious candidate. We could also have written find_extrema's signature like so, with no change in meaning:
    fn find_extrema(slice: &[i32]) -> Extrema {
        ...
    }

    // Granted, we might have meant Extrema<'static>, but that's pretty unusual. Rust provides a shorthand for the common case.



    // Deriving Common Traits for Struct Types

    // Structs can be very easy to write:
    struct Point {
        x: f64,
        y: f64
    }

    // However, if we were to start using this Point type, we would quickly notice that it's a bit of a pain. As written, Point is not copyable or cloneable. We can't print it with println!("{:?}", point), and it doesn't support the == and != operators.

    // Each of these features has a name in Rust, Copy, Clone, Debug, and PartialEq. They are called traits. In chapt 11, goes into more detail on how to implement traits by hand for our own structs. But in the case of these standard traits, and several others, we don't need to implement them by hand unless we want some kind of custom behaviour. Rust can automatically implement them for us, with mechanical accuracy. Just add a #[derive] attribute to the struct:
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64
    }

    // Each of these traits can be implemented automatically for a struct, provided that each of its fields implements the trait. We can ask Rust to derive PartialEq for Point because its two fields are both of type f64, which already implements PartialEq.

    // Rust can also derive PartialCmp, which would add support for the comparison operators <, >, <=, and >=. We haven't done so here because comparing two points to see if one is "less than" the other is actually a pretty weird thing to do. There's no one conventional order on points. So we choose not to support those operators for Point values.

    // Cases like this are one reason that Rust makes us write the #[derive] attribute rather than automatically deriving every trait it can. Another reason is that implementing a trait is automatically a public feature, so copyability, cloneability, and so forth are all part of our struct's public API and should be chosen deliberately. More on this in chapt 13.



    




}
