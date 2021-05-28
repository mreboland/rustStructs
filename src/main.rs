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
    
}
