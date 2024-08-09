use pest::iterators::Pair;
use vnsl_core::model::{VnslAtom, VnslExpr};

use crate::{
    data_type,
    utils::{extract_inner, extract_inner_as_rule},
    Rule,
};

pub fn parse_expr(rule: Pair<Rule>) -> anyhow::Result<VnslExpr> {
    let inner = extract_inner(rule);
    match inner.as_rule() {
        Rule::op_expr => Ok(VnslExpr::Op(ops_expr::parse_op_expr(inner)?)),
        Rule::assign_expr => todo!(),
        _ => unreachable!(),
    }
}

fn parse_atom(rule: Pair<Rule>) -> anyhow::Result<VnslAtom> {
    let inner = extract_inner(rule);
    match inner.as_rule() {
        Rule::literal => {
            let data = data_type::parse_data_type(inner.into_inner().next().unwrap())?;
            Ok(VnslAtom::Literal(data))
        }
        Rule::group => {
            let inner = extract_inner_as_rule(inner, Rule::expr);
            Ok(VnslAtom::Group(Box::new(parse_expr(inner)?)))
        }
        _ => unreachable!(),
    }
}

mod ops_expr {
    use pest::iterators::Pair;
    use vnsl_core::model::{VnslArithOps, VnslOpExpr, VnslOps, VnslRhsOp};

    use crate::{
        statement::expr::{parse_atom, parse_expr},
        utils::{extract_inner, extract_inners_optional},
        Rule,
    };

    pub fn parse_op_expr(rule: Pair<Rule>) -> anyhow::Result<VnslOpExpr> {
        let inners = extract_inners_optional(rule, [Rule::atom, Rule::ops, Rule::expr]);
        let atom = parse_atom(inners.get(&Rule::atom).unwrap().clone())?;
        let mut ops_expr = VnslOpExpr {
            lhs: atom,
            rhs_op: None,
        };
        if let Some(ops) = inners.get(&Rule::ops) {
            let op = parse_ops(ops.clone());
            let expr = inners.get(&Rule::expr).unwrap().clone();
            let rhs = Box::new(parse_expr(expr)?);
            ops_expr.rhs_op = Some(VnslRhsOp { op, rhs });
        }
        println!("{ops_expr:#?}");
        Ok(ops_expr)
    }

    pub fn parse_ops(rule: Pair<Rule>) -> VnslOps {
        let inner = extract_inner(rule);
        match inner.as_rule() {
            Rule::arithmetic_op => VnslOps::Arithmetic(parse_arith_op(inner)),
            Rule::comp_op => todo!(),
            _ => unreachable!(),
        }
    }

    fn parse_arith_op(rule: Pair<Rule>) -> VnslArithOps {
        let inner = extract_inner(rule);
        match inner.as_rule() {
            Rule::add => VnslArithOps::Add,
            Rule::subtract => VnslArithOps::Subtract,
            Rule::multiply => VnslArithOps::Multiply,
            Rule::divide => VnslArithOps::Divide,
            _ => todo!(),
        }
    }
}
