use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use module_083_solutions::{Memo, Runtime, Signal};

// --- signals ---------------------------------------------------------------

#[test]
fn signal_get_returns_initial_value() {
    let mut rt = Runtime::new();
    let (sig, _write) = rt.create_signal(41);
    assert_eq!(rt.get(sig), 41);
}

#[test]
fn set_updates_value_and_reads_back() {
    let mut rt = Runtime::new();
    let (sig, write) = rt.create_signal(0);
    rt.set(write, 10);
    assert_eq!(rt.get(sig), 10);
    rt.set(write, 20);
    assert_eq!(rt.get(sig), 20);
}

#[test]
fn update_mutates_in_place() {
    let mut rt = Runtime::new();
    let (sig, write) = rt.create_signal(1);
    rt.update(write, |n| *n += 2);
    rt.update(write, |n| *n *= 3);
    assert_eq!(rt.get(sig), 9);
}

#[test]
fn signals_are_independent() {
    let mut rt = Runtime::new();
    let (sig_a, write_a) = rt.create_signal(1);
    let (sig_b, _write_b) = rt.create_signal(2);
    rt.set(write_a, 100);
    assert_eq!(rt.get(sig_a), 100);
    assert_eq!(rt.get(sig_b), 2);
}

#[test]
fn different_value_types_can_coexist() {
    let mut rt = Runtime::new();
    let (name, write_name) = rt.create_signal("rust".to_string());
    let (count, write_count) = rt.create_signal(0u32);
    rt.set(write_name, "leptos".to_string());
    rt.set(write_count, 3);
    assert_eq!(rt.get(name), "leptos");
    assert_eq!(rt.get(count), 3);
}

// --- memos ------------------------------------------------------------------

#[test]
fn memo_reflects_dependency_changes() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(2);
    let (b, write_b) = rt.create_signal(3);
    let sum = rt.create_memo(move |rt| rt.get(a) + rt.get(b));

    assert_eq!(rt.read_memo(sum), 5);
    rt.set(write_a, 10);
    assert_eq!(rt.read_memo(sum), 13);
    rt.set(write_b, 1);
    assert_eq!(rt.read_memo(sum), 11);
}

#[test]
fn memo_computes_once_and_is_cached() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(1);
    let runs = Rc::new(Cell::new(0));
    let runs_memo = runs.clone();
    let doubled = rt.create_memo(move |rt| {
        runs_memo.set(runs_memo.get() + 1);
        rt.get(a) * 2
    });

    assert_eq!(rt.read_memo(doubled), 2);
    assert_eq!(rt.read_memo(doubled), 2);
    assert_eq!(rt.read_memo(doubled), 2);
    assert_eq!(runs.get(), 1, "cached reads must not recompute");

    rt.set(write_a, 5);
    assert_eq!(rt.read_memo(doubled), 10);
    assert_eq!(runs.get(), 2, "one recompute per dependency change");
}

#[test]
fn memo_chains_through_other_memos() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(2);
    let squared = rt.create_memo(move |rt| rt.get(a) * rt.get(a));
    let plus_one = rt.create_memo(move |rt| rt.read_memo(squared) + 1);

    assert_eq!(rt.read_memo(plus_one), 5);
    rt.set(write_a, 3);
    assert_eq!(rt.read_memo(plus_one), 10);
}

#[test]
fn memo_ignores_other_signals() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(1);
    let (b, write_b) = rt.create_signal(99);
    let runs = Rc::new(Cell::new(0));
    let runs_memo = runs.clone();
    let only_a = rt.create_memo(move |rt| {
        runs_memo.set(runs_memo.get() + 1);
        rt.get(a)
    });

    assert_eq!(rt.read_memo(only_a), 1);
    rt.set(write_b, 5);
    assert_eq!(rt.read_memo(only_a), 1);
    assert_eq!(runs.get(), 1, "writing an unread signal must not recompute");

    rt.set(write_a, 2);
    assert_eq!(runs.get(), 2);
}

#[test]
fn memo_read_outside_effect_is_safe() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(1);
    let double = rt.create_memo(move |rt| rt.get(a) * 2);
    assert_eq!(rt.read_memo(double), 2);
    rt.set(write_a, 4);
    assert_eq!(rt.read_memo(double), 8);
}

// --- effects ----------------------------------------------------------------

#[test]
fn effect_runs_once_on_creation() {
    let mut rt = Runtime::new();
    let (a, _write_a) = rt.create_signal(1);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
    });
    assert_eq!(runs.get(), 1);
}

#[test]
fn effect_reruns_on_dependency_change() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
    });
    assert_eq!(runs.get(), 1);

    rt.set(write_a, 1);
    assert_eq!(runs.get(), 2);
    rt.set(write_a, 2);
    assert_eq!(runs.get(), 3);
}

#[test]
fn effect_ignores_unrelated_writes() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let (b, write_b) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
    });
    assert_eq!(runs.get(), 1);

    rt.set(write_b, 5);
    rt.set(write_b, 6);
    rt.set(write_b, 7);
    assert_eq!(runs.get(), 1, "unrelated writes must not rerun the effect");

    rt.set(write_a, 1);
    assert_eq!(runs.get(), 2);
}

#[test]
fn effect_switches_dependencies() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let (b, write_b) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        if rt.get(a) > 0 {
            let _ = rt.get(b);
        }
    });
    assert_eq!(runs.get(), 1);

    rt.set(write_b, 1);
    assert_eq!(runs.get(), 1, "b is not a dependency while a == 0");

    rt.set(write_a, 1);
    assert_eq!(runs.get(), 2, "a changes, so the effect reruns");

    rt.set(write_b, 2);
    assert_eq!(runs.get(), 3, "now b is a dependency, so this reruns too");

    rt.set(write_a, 0);
    assert_eq!(runs.get(), 4);
    rt.set(write_b, 3);
    assert_eq!(runs.get(), 4, "b is no longer read while a == 0");
}

#[test]
fn effect_sees_final_value_of_batched_writes() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let last_seen = Rc::new(Cell::new(-1));
    let last = last_seen.clone();
    rt.create_effect(move |rt| {
        last.set(rt.get(a));
    });

    rt.batch(|rt| {
        rt.set(write_a, 1);
        rt.set(write_a, 2);
        rt.set(write_a, 3);
        assert_eq!(last_seen.get(), 0, "inside the batch, nothing ran yet");
    });

    assert_eq!(last_seen.get(), 3, "one flush at batch end, final value");
}

#[test]
fn batch_flushes_effect_once_not_per_write() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
    });
    assert_eq!(runs.get(), 1);

    rt.batch(|rt| {
        rt.set(write_a, 1);
        rt.set(write_a, 2);
        rt.set(write_a, 3);
    });
    assert_eq!(runs.get(), 2, "one initial run + one batched run");
}

#[test]
fn nested_batches_flush_at_outer_boundary() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let runs_effect = runs.clone();
    rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
    });

    rt.batch(|rt| {
        rt.set(write_a, 1);
        rt.batch(|rt| {
            rt.set(write_a, 2);
            assert_eq!(runs.get(), 1, "nested batch: still not flushed");
        });
        assert_eq!(runs.get(), 1);
    });
    assert_eq!(runs.get(), 2);
}

// --- cleanup & disposal ------------------------------------------------------

#[test]
fn cleanup_runs_before_effect_reruns() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let events: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let events_effect = events.clone();
    rt.create_effect(move |rt| {
        let value = rt.get(a);
        events_effect.borrow_mut().push(format!("run({value})"));
        let ef = events_effect.clone();
        rt.on_cleanup(move || {
            ef.borrow_mut().push("cleanup".to_string());
        });
    });

    assert_eq!(events.borrow().as_slice(), &["run(0)".to_string()]);
    rt.set(write_a, 1);
    assert_eq!(
        events.borrow().as_slice(),
        &[
            "run(0)".to_string(),
            "cleanup".to_string(),
            "run(1)".to_string()
        ]
    );
}

#[test]
fn dispose_runs_cleanup_and_stops_effect() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let cleaned = Rc::new(Cell::new(false));
    let runs_effect = runs.clone();
    let cleaned_effect = cleaned.clone();
    let handle = rt.create_effect(move |rt| {
        runs_effect.set(runs_effect.get() + 1);
        let _ = rt.get(a);
        let ce = cleaned_effect.clone();
        rt.on_cleanup(move || ce.set(true));
    });
    assert_eq!(runs.get(), 1);

    rt.dispose(handle);
    assert!(cleaned.get(), "dispose must run the cleanup");

    rt.set(write_a, 1);
    rt.set(write_a, 2);
    assert_eq!(runs.get(), 1, "disposed effect must not rerun");
}

#[test]
fn dispose_returns_slot_to_free_list() {
    let mut rt = Runtime::new();
    let handle = rt.create_effect(|_| {});
    rt.dispose(handle);
    let (sig, _write) = rt.create_signal(42);
    assert_eq!(rt.get(sig), 42, "new nodes must work after a dispose");
}

#[test]
fn on_cleanup_outside_effect_is_a_no_op() {
    let mut rt = Runtime::new();
    rt.on_cleanup(|| panic!("must not be registered"));
    let (sig, write) = rt.create_signal(0);
    rt.set(write, 1);
    assert_eq!(rt.get(sig), 1);
}

// --- scopes ------------------------------------------------------------------

#[test]
fn scope_disposal_runs_cleanups_and_disconnects() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let runs = Rc::new(Cell::new(0));
    let cleaned = Rc::new(Cell::new(false));
    let runs_effect = runs.clone();
    let cleaned_effect = cleaned.clone();
    let scope = rt.create_scope(move |rt| {
        let ce = cleaned_effect.clone();
        let re = runs_effect.clone();
        let _handle = rt.create_effect(move |rt| {
            re.set(re.get() + 1);
            let _ = rt.get(a);
            let c = ce.clone();
            rt.on_cleanup(move || c.set(true));
        });
    });
    assert_eq!(runs.get(), 1);

    rt.dispose_scope(scope);
    assert!(cleaned.get(), "scope disposal must run child cleanups");

    rt.set(write_a, 1);
    rt.set(write_a, 2);
    assert_eq!(runs.get(), 1, "scope-disposed effects must not rerun");
}

#[test]
fn nested_scopes_dispose_children_first() {
    let mut rt = Runtime::new();
    let (a, write_a) = rt.create_signal(0);
    let order: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let order_inner = order.clone();
    let order_outer = order.clone();

    let scope = rt.create_scope(move |rt| {
        let o_outer = order_outer.clone();
        rt.create_effect(move |rt| {
            let _ = rt.get(a);
            let oo = o_outer.clone();
            rt.on_cleanup(move || oo.borrow_mut().push("outer".to_string()));
        });
        let o_inner = order_inner.clone();
        rt.create_scope(move |rt| {
            rt.create_effect(move |rt| {
                let _ = rt.get(a);
                let oi = o_inner.clone();
                rt.on_cleanup(move || oi.borrow_mut().push("inner".to_string()));
            });
        });
    });

    rt.dispose_scope(scope);
    assert_eq!(
        order.borrow().as_slice(),
        &["inner".to_string(), "outer".to_string()],
        "children must be disposed before their parent"
    );
}

#[test]
#[should_panic(expected = "disposed")]
fn signal_read_after_scope_disposal_panics() {
    let mut rt = Runtime::new();
    let mut captured: Option<Signal<i32>> = None;
    let scope = rt.create_scope(|rt| {
        let (sig, _write) = rt.create_signal(7);
        captured = Some(sig);
    });
    rt.dispose_scope(scope);

    let _ = rt.get(captured.unwrap());
}

#[test]
#[should_panic(expected = "disposed")]
fn memo_read_after_scope_disposal_panics() {
    let mut rt = Runtime::new();
    let (a, _write_a) = rt.create_signal(1);
    let mut memo: Option<Memo<i32>> = None;
    let scope = rt.create_scope(|rt| {
        let m = rt.create_memo(move |rt| rt.get(a) * 10);
        memo = Some(m);
    });
    rt.dispose_scope(scope);

    let _ = rt.read_memo(memo.unwrap());
}

// --- misc --------------------------------------------------------------------

#[test]
fn runtime_defaults_to_empty() {
    let mut rt = Runtime::default();
    let (sig, _write) = rt.create_signal(true);
    assert!(rt.get(sig));
}
