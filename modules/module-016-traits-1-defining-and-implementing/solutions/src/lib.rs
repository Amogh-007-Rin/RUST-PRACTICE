//! Module 016: Traits I — defining and implementing traits, default methods
//! (reference solution).

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
        &self.name
    }

    fn greet(&self) -> String {
        format!("Hi, I'm {}!", self.name())
    }

    fn farewell(&self) -> String {
        "Goodbye, human.".to_string()
    }
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person named {}", self.name)
    }
}

/// A robot identified by its model name.
pub struct Robot {
    pub model: String,
}

impl Greeter for Robot {
    fn name(&self) -> &str {
        &self.model
    }

    fn farewell(&self) -> String {
        "Beep. Shutting down.".to_string()
    }
}

impl Describable for Robot {
    fn describe(&self) -> String {
        format!("Robot model {}", self.model)
    }

    fn summary(&self) -> String {
        format!("Robot {} reporting for duty.", self.model)
    }
}

/// A cat with a name.
pub struct Cat {
    pub name: String,
}

impl Greeter for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn farewell(&self) -> String {
        "Meow.".to_string()
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!("A cat named {}", self.name)
    }
}
