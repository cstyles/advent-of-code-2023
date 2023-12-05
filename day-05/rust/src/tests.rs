use super::smart_overlap;
use super::OverlapResult;

#[test]
fn smart_overlap_full() {
    assert_eq!(OverlapResult::Full(3..5), smart_overlap(&(3..5), &(3..5)));
    assert_eq!(OverlapResult::Full(3..5), smart_overlap(&(3..5), &(0..5)));
    assert_eq!(OverlapResult::Full(3..5), smart_overlap(&(3..5), &(3..10)));
    assert_eq!(OverlapResult::Full(3..5), smart_overlap(&(3..5), &(0..10)));
}

#[test]
fn smart_overlap_none() {
    assert_eq!(OverlapResult::None, smart_overlap(&(0..3), &(5..10)));
    assert_eq!(OverlapResult::None, smart_overlap(&(5..10), &(0..3)));
    assert_eq!(OverlapResult::None, smart_overlap(&(3..5), &(0..3)));
    assert_eq!(OverlapResult::None, smart_overlap(&(3..5), &(5..10)));
}

#[test]
fn smart_overlap_split() {
    assert_eq!(
        OverlapResult::Split {
            overlap: 3..5,
            left: 0..3,
            right: 5..10
        },
        smart_overlap(&(0..10), &(3..5))
    );
}

#[test]
fn smart_overlap_partial() {
    assert_eq!(
        OverlapResult::Partial {
            overlap: 3..5,
            leftovers: 5..10,
        },
        smart_overlap(&(3..10), &(0..5))
    );

    assert_eq!(
        OverlapResult::Partial {
            overlap: 3..5,
            leftovers: 0..3,
        },
        smart_overlap(&(0..5), &(3..10))
    );
}
