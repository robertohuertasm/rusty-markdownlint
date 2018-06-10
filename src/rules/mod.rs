use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

mod md001;

#[macro_export]
macro_rules! rule {
    ($name:ident : $func:expr) => {{
        pub struct $name {}

        impl RuleCheck for $name {
            fn check<'a>(&self, root: &'a AstNode<'a>) -> RuleResult {
                $func(root)
            }
        }
        $name {}
    }};
}

#[macro_export]
macro_rules! boxedrule {
    ($name:ident : $func:expr) => {{
        Box::new(rule! {$name: $func})
    }};
}

pub fn rule_md001() -> Box<impl RuleCheck> {
    boxedrule!{ MD001: md001::check }
}
