use module_098_solutions::{OrderProcessor, OrderStatus};

#[test]
fn create_order_returns_unique_ids() {
    let mut processor = OrderProcessor::new();
    let id1 = processor.create_order();
    let id2 = processor.create_order();
    assert_ne!(id1, id2);
}

#[test]
fn add_item_to_pending_order() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    assert!(processor.add_item(id, "Widget", 1000, 2));
    let order = processor.get_order(id).unwrap();
    assert_eq!(order.items.len(), 1);
    assert_eq!(order.items[0].name, "Widget");
    assert_eq!(order.items[0].price_cents, 1000);
    assert_eq!(order.items[0].quantity, 2);
}

#[test]
fn cannot_add_item_to_finalized_order() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.finalize(id);
    assert!(!processor.add_item(id, "Widget", 1000, 1));
}

#[test]
fn cannot_add_item_to_canceled_order() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.cancel(id);
    assert!(!processor.add_item(id, "Widget", 1000, 1));
}

#[test]
fn remove_item_removes_only_first_occurrence() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 1);
    processor.add_item(id, "Widget", 2000, 1);
    processor.add_item(id, "Gadget", 3000, 1);
    assert!(processor.remove_item(id, "Widget"));
    let order = processor.get_order(id).unwrap();
    assert_eq!(
        order.items.len(),
        2,
        "only the first Widget should be removed"
    );
    assert_eq!(order.items[0].price_cents, 2000, "second Widget remains");
}

#[test]
fn remove_item_returns_false_when_not_found() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 1);
    assert!(!processor.remove_item(id, "NonExistent"));
}

#[test]
fn total_calculates_correctly_with_quantities() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 3);
    processor.add_item(id, "Gadget", 2000, 2);
    let total = processor.total(id).unwrap();
    assert_eq!(total, 7000, "3*1000 + 2*2000 = 7000");
}

#[test]
fn total_applies_discount() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 2);
    processor.apply_discount(id, 10);
    let total = processor.total(id).unwrap();
    assert_eq!(total, 1800, "2000 - 10% = 1800");
}

#[test]
fn total_returns_none_for_nonexistent_order() {
    let processor = OrderProcessor::new();
    assert_eq!(processor.total(999), None);
}

#[test]
fn apply_discount_rejects_invalid_percentages() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    assert!(!processor.apply_discount(id, 101));
    assert!(processor.apply_discount(id, 100));
}

#[test]
fn finalize_prevents_modifications() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 1);
    processor.finalize(id);
    assert!(!processor.add_item(id, "Gadget", 2000, 1));
    assert!(!processor.remove_item(id, "Widget"));
    assert!(!processor.apply_discount(id, 10));
}

#[test]
fn cancel_prevents_finalization() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.cancel(id);
    assert!(!processor.finalize(id));
}

#[test]
fn cannot_cancel_finalized_order() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.finalize(id);
    assert!(!processor.cancel(id));
}

#[test]
fn item_count_returns_total_quantity() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.add_item(id, "Widget", 1000, 3);
    processor.add_item(id, "Gadget", 2000, 2);
    let count = processor.item_count(id).unwrap();
    assert_eq!(count, 5, "3 + 2 = 5 total items");
}

#[test]
fn item_count_returns_none_for_nonexistent_order() {
    let processor = OrderProcessor::new();
    assert_eq!(processor.item_count(999), None);
}

#[test]
fn order_status_transitions() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    assert_eq!(
        processor.get_order(id).unwrap().status,
        OrderStatus::Pending
    );
    processor.finalize(id);
    assert_eq!(
        processor.get_order(id).unwrap().status,
        OrderStatus::Finalized
    );
}

#[test]
fn empty_order_has_zero_total() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    assert_eq!(processor.total(id), Some(0));
}

#[test]
fn discount_on_empty_order() {
    let mut processor = OrderProcessor::new();
    let id = processor.create_order();
    processor.apply_discount(id, 50);
    assert_eq!(processor.total(id), Some(0));
}
