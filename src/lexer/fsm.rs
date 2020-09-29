pub use FsmTransitions::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FsmTransitions {
    _Reject,
    _Integer,
    _Real,
    _Operator,
    _String,
    _Unknown,
    _Space,
    _Comment,
    _Separator,
    _Keyword,
    _Identifier,
}

pub const STATE_TABLE: &[&[FsmTransitions]] = &[
    &[
        _Reject, _Integer, _Real, _Operator, _String, _Unknown, _Space, _Comment, _Separator,
    ], //Default
    &[
        _Integer, _Integer, _Real, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject,
    ], //State 1
    &[
        _Real, _Real, _Unknown, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject,
    ], //State 2
    &[
        _Operator, _Reject, _Reject, _Reject, _String, _Reject, _Reject, _Reject, _Reject,
    ], //State 3
    &[
        _String, _String, _Reject, _String, _String, _Reject, _Reject, _Reject, _Reject,
    ], //State 4
    &[
        _Unknown, _Unknown, _Unknown, _Unknown, _Unknown, _Unknown, _Reject, _Reject, _Reject,
    ], //State 5
    &[
        _Space, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject,
    ], //State 6
    &[
        _Comment, _Comment, _Comment, _Comment, _Comment, _Comment, _Comment, _Reject, _Comment,
    ], //State 7
    &[
        _Reject, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject, _Reject,
    ], //State 8
];