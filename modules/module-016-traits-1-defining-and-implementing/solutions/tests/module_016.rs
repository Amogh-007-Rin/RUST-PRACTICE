use module_016_solutions::{Cat, Describable, Greeter, Person, Robot};

#[test]
fn person_uses_custom_greeting() {
    let alice = Person {
        name: "Alice".to_string(),
    };
    assert_eq!(alice.greet(), "Hi, I'm Alice!");
}

#[test]
fn person_says_farewell() {
    let alice = Person {
        name: "Alice".to_string(),
    };
    assert_eq!(alice.farewell(), "Goodbye, human.");
}

#[test]
fn person_describes_itself() {
    let alice = Person {
        name: "Alice".to_string(),
    };
    assert_eq!(alice.describe(), "Person named Alice");
}

#[test]
fn person_uses_default_summary() {
    let alice = Person {
        name: "Alice".to_string(),
    };
    assert_eq!(alice.summary(), alice.describe());
}

#[test]
fn robot_uses_default_greeting() {
    let r2 = Robot {
        model: "R2".to_string(),
    };
    assert_eq!(r2.greet(), "Hello, R2!");
}

#[test]
fn robot_says_farewell() {
    let r2 = Robot {
        model: "R2".to_string(),
    };
    assert_eq!(r2.farewell(), "Beep. Shutting down.");
}

#[test]
fn robot_overrides_summary() {
    let r2 = Robot {
        model: "R2".to_string(),
    };
    assert_eq!(r2.summary(), "Robot R2 reporting for duty.");
}

#[test]
fn robot_still_describes_via_the_trait() {
    let r2 = Robot {
        model: "R2".to_string(),
    };
    assert_eq!(r2.describe(), "Robot model R2");
}

#[test]
fn cat_uses_default_greeting() {
    let whiskers = Cat {
        name: "Whiskers".to_string(),
    };
    assert_eq!(whiskers.greet(), "Hello, Whiskers!");
}

#[test]
fn cat_says_farewell() {
    let whiskers = Cat {
        name: "Whiskers".to_string(),
    };
    assert_eq!(whiskers.farewell(), "Meow.");
}

#[test]
fn cat_describes_itself() {
    let whiskers = Cat {
        name: "Whiskers".to_string(),
    };
    assert_eq!(whiskers.describe(), "A cat named Whiskers");
}

#[test]
fn greeting_uses_the_name_method() {
    let robot = Robot {
        model: "B-9".to_string(),
    };
    assert_eq!(robot.name(), "B-9");
}
