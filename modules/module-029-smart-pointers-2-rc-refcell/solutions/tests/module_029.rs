use module_029_solutions::{shared_members, Chat, Counter, Member, Wallet};
use std::rc::Rc;

#[test]
fn members_share_the_same_chat() {
    let chat = Rc::new(Chat {
        name: "rust-and-me".to_string(),
    });
    let members = shared_members(Rc::clone(&chat), &["ada", "grace", "linus"]);
    assert_eq!(Rc::strong_count(&chat), 4);

    for member in &members {
        assert_eq!(member.chat_name(), "rust-and-me");
    }
    assert_eq!(members[0].nickname, "ada");

    drop(members);
    assert_eq!(Rc::strong_count(&chat), 1);
}

#[test]
fn shared_members_with_no_nicknames_keeps_one_handle() {
    let chat = Rc::new(Chat {
        name: "empty".to_string(),
    });
    let members = shared_members(Rc::clone(&chat), &[]);
    assert_eq!(Rc::strong_count(&chat), 1);
    assert!(members.is_empty());
}

#[test]
fn counter_mutates_through_an_immutable_reference() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    assert_eq!(counter.value(), 3);
}

#[test]
fn wallet_handles_share_one_balance() {
    let wallet = Wallet::new(100);
    let wallet2 = wallet.share();
    wallet.deposit(50);
    wallet2.withdraw(30).unwrap();
    assert_eq!(wallet.balance(), 120);
    assert_eq!(wallet2.balance(), 120);
}

#[test]
fn wallet_rejects_insufficient_funds() {
    let wallet = Wallet::new(10);
    assert_eq!(wallet.withdraw(500), Err("insufficient funds".to_string()));
    assert_eq!(wallet.balance(), 10);
}

#[test]
fn wallet_share_increases_refcount() {
    let wallet = Wallet::new(0);
    let _wallet2 = wallet.share();
    let _wallet3 = wallet.share();
    drop(wallet);
}
