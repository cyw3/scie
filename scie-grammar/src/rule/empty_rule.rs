use crate::rule::abstract_rule::RuleEnum;
use crate::rule::{AbstractRule, CompiledRule, Rule};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct EmptyRule {}

impl EmptyRule {
    pub fn new() -> Self {
        EmptyRule {}
    }
}

lazy_static! {
    static ref EMPTY_RULE: Rule = Rule {
        _type: "".to_string(),
        _location: None,
        id: 0,
        _name: None,
        _content_name: None,
    };
}

impl AbstractRule for EmptyRule {
    fn id(&self) -> i32 {
        0
    }
    fn type_of(&self) -> &'static str {
        "EmptyRule"
    }
    fn get_rule(&self) -> &Rule {
        &EMPTY_RULE
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::EmptyRule(self)
    }
    fn get_instance(&self) -> &dyn Any {
        self
    }
    // fn collect_patterns_recursive(
    //     &mut self,
    //     _container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
    //     _out: &mut RegExpSourceList,
    //     _is_first: bool,
    // ) {
    //     unimplemented!()
    // }
    //
    fn compile(
        &mut self,
        _container: &mut HashMap<i32, Rc<RefCell<dyn AbstractRule>>>,
        _end_regex_source: &Option<String>,
        _allow_a: bool,
        _allow_g: bool,
    ) -> CompiledRule {
        unimplemented!()
    }
}
