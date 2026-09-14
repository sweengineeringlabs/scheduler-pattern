//! Integration tests for [`scheduler_pattern::Trigger`].

use std::time::Duration;

use scheduler_pattern::Trigger;

/// @covers: Trigger::is_once / is_recurring — correctly classify Once.
#[test]
fn test_once_is_once_not_recurring() {
    let t = Trigger::Once(Duration::from_secs(1));
    assert!(t.is_once());
    assert!(!t.is_recurring());
}

/// @covers: Trigger::is_once / is_recurring — correctly classify Every.
#[test]
fn test_every_is_recurring_not_once() {
    let t = Trigger::Every(Duration::from_secs(60));
    assert!(t.is_recurring());
    assert!(!t.is_once());
}
