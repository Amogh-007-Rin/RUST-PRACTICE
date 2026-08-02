//! Module 016: Traits I — defining and implementing traits, default methods.
//!
//! The traits are defined below. Fill in the `TODO(module-016)` bodies inside
//! the `impl` blocks so the integration tests in `tests/module_016.rs` pass.

/// Something that greets people.
///
/// `name` and `farewell` are required; `greet` has a default implementation
/// that any type may override.
pub trait Greeter {
    /// The name used in greetings.
    fn name(&self) -> &str;

    /// A greeting addressed to this thing.
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }

    /// What this thing says when leaving.
    fn farewell(&self) -> String;
}

/// Something that can describe itself.
pub trait Describable {
    /// A one-sentence description.
    fn describe(&self) -> String;

    /// A short summary; defaults to the full description.
    fn summary(&self) -> String {
        self.describe()
    }
}

/// A person with a name.
pub struct Person {
    pub name: String,
}

impl Greeter for Person {
    fn name(&self) -> &str {
        // TODO(module-016): return a reference to this person's name.
        panic!("not implemented")
    }

    fn greet(&self) -> String {
        // TODO(module-016): override the default: "Hi, I'm {name}!"
        // (use `self.name()` for the name).
        panic!("not implemented")
    }

    fn farewell(&self) -> String {
        // TODO(module-016): return "Goodbye, human.".
        panic!("not implemented")
    }
}

impl Describable for Person {
    fn describe(&self) -> String {
        // TODO(module-016): return `format!("Person named {}", self.name)`.
        panic!("not implemented")
    }
}

/// A robot identified by its model name.
pub struct Robot {
    pub model: String,
}

impl Greeter for Robot {
    fn name(&self) -> &str {
        // TODO(module-016): return a reference to this robot's model.
        panic!("not implemented")
    }

    fn farewell(&self) -> String {
        // TODO(module-016): return "Beep. Shutting down.".
        panic!("not implemented")
    }
}

impl Describable for Robot {
    fn describe(&self) -> String {
        // TODO(module-016): return `format!("Robot model {}", self.model)`.
        panic!("not implemented")
    }

    fn summary(&self) -> String {
        // TODO(module-016): override the default: `format!("Robot {} reporting
        // for duty.", self.model)`.
        panic!("not implemented")
    }
}

/// A cat with a name.
pub struct Cat {
    pub name: String,
}

impl Greeter for Cat {
    fn name(&self) -> &str {
        // TODO(module-016): return a reference to this cat's name.
        panic!("not implemented")
    }

    fn farewell(&self) -> String {
        // TODO(module-016): return "Meow.".
        panic!("not implemented")
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        // TODO(module-016): return `format!("A cat named {}", self.name)`.
        panic!("not implemented")
    }
}
