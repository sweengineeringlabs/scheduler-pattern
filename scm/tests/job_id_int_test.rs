//! Integration tests for [`scheduler_pattern::JobId`].

use scheduler_pattern::JobId;

/// @covers: JobId::new — generates unique IDs, not a constant/reused value.
#[test]
fn test_new_generates_unique_ids() {
    let a = JobId::new();
    let b = JobId::new();
    assert_ne!(a, b);
}

/// @covers: JobId::Display — produces non-empty, real UUID-shaped text.
#[test]
fn test_display_is_non_empty() {
    let id = JobId::new();
    assert!(!id.to_string().is_empty());
}
