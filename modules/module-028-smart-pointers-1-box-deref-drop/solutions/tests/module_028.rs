use module_028_solutions::{eval, Expr, Gadget, MyBox, DROPPED_GADGETS};
use std::sync::atomic::Ordering;

#[test]
fn eval_handles_nested_expressions() {
    let expr = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
    );
    assert_eq!(eval(&expr), 14);

    let deep = Expr::Mul(Box::new(Expr::Num(0)), Box::new(Expr::Num(100)));
    assert_eq!(eval(&deep), 0);

    assert_eq!(eval(&Expr::Num(-7)), -7);
}

#[test]
fn my_box_derefs_to_the_inner_value() {
    let b = MyBox::new(5);
    assert_eq!(*b, 5);
}

#[test]
fn my_box_deref_coerces_for_method_calls() {
    let s = MyBox::new(String::from("hello"));
    assert_eq!(s.len(), 5);
    assert!(s.starts_with("he"));
}

#[test]
fn my_box_deref_mut_allows_mutation() {
    let mut b = MyBox::new(10);
    *b += 5;
    assert_eq!(*b, 15);
}

#[test]
fn gadget_drop_counts_releases() {
    DROPPED_GADGETS.store(0, Ordering::Relaxed);
    {
        let _g1 = Gadget;
        let _g2 = Gadget;
    }
    assert_eq!(DROPPED_GADGETS.load(Ordering::Relaxed), 2);
}

#[test]
fn gadget_drop_runs_even_for_explicit_drop() {
    DROPPED_GADGETS.store(0, Ordering::Relaxed);
    let g = Gadget;
    drop(g);
    assert_eq!(DROPPED_GADGETS.load(Ordering::Relaxed), 1);
}
