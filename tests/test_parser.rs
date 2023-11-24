#[cfg(test)]
mod tests {
    use cfg::cfg::CFG;
    use cfg::token::Token;

    #[test]
    fn filter_empty_productions() {
        let cfg_string = r#"
            7
            S->XY
            X->AX
            X->!
            A->a
            Y->BY
            Y->!
            B->b
        "#;

        let my_cfg = cfg_string.parse::<CFG>().expect("Building a CFG Failed.");
        let empty_variables = &[Token::Variable("X".into()), Token::Variable("Y".into())];

        assert!(itertools::equal(
            my_cfg.variables_with_empty_productions().iter(),
            empty_variables.iter()
        ));
    }

    #[test]
    fn remove_empty_productions() {
        let mut my_cfg = r#"
            7
            S->XY
            S->!
            X->AX
            X->!
            A->a
            Y->BY
            Y->!
            B->b        
        "#
        .parse::<CFG>()
        .expect("Building CFG Failed.");
        my_cfg.remove_empty_productions();
    }
}
