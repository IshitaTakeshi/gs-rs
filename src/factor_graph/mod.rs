pub mod variable;
pub mod factor;

/*
use crate::factor_graph::factor::Factor;
use crate::factor_graph::variable::Variable;
use crate::factor_graph::constraint::Constraint;

pub mod constraint;


#[derive(Debug)]
pub struct FactorGraph<'a> {
    variables: Vec<&'a dyn Variable<'a>>,
    factors: Vec<&'a dyn Factor<'a>>,
    constraints: Vec<&'a Constraint<'a>>,
}

impl<'a> FactorGraph<'a> {
    pub fn new(variables: Vec<&'a dyn Variable<'a>>, factors: Vec<&'a dyn Factor<'a>>, constraints: Vec<&'a Constraint<'a>>,) -> Self {
        FactorGraph { variables, factors, constraints }
    }
}

#[cfg(test)]
mod tests {
    use crate::factor_graph::FactorGraph;
    use log::LevelFilter;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn empty() {
        init();

        let empty = FactorGraph {
            factors: vec![],
            variables: vec![],
            constraints: vec![]
        };

        info!("{:?}", empty);
    }

}*/
