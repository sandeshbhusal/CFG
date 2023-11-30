//! Tests for custom grammars

#[cfg(test)]
mod tests {
    use cfg::pda::PDA;
    use cfg::cfg::CFG;

    #[test]
    fn at_least_3_1s() {
        let cfg = r#"
            S->T1T1T1T
            T->0T
            T->1T
            T->! 
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("111", 100), true);
        assert_eq!(pda.trace_string("000101", 100), true);
        assert_ne!(pda.trace_string("101", 100), false);
    }

    #[test]
    fn derives_empty_string() {
        let cfg = r#"
            S->!
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("", 100), true);
    }

    #[test]
    fn starts_ends_with_same_symbol() {
        let cfg = r#"
            S->0
            S->1
            S->!
            S->1T1
            S->0T0
            T->0T
            T->1T
            T->!
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("101", 100), true);
        assert_eq!(pda.trace_string("100101", 100), true);
        assert_ne!(pda.trace_string("100", 100), false);
    }

    #[test]
    fn pallindrome() {
        let cfg = r#"
            S->0
            S->1
            S->!
            S->0S0
            S->1S1
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("", 100), true);
        assert_eq!(pda.trace_string("101101", 100), true);
        assert_eq!(pda.trace_string("1011101", 100), true);
        assert_ne!(pda.trace_string("100", 100), false);
    }

    #[test]
    fn equal_as_and_bs() {
        let cfg = r#"
            S->aSb
            S->bSa
            S->!
            S->SS
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("", 100), true);
        assert_eq!(pda.trace_string("ababab", 100), true);
        assert_eq!(pda.trace_string("aaabbb", 100), true);
        assert_ne!(pda.trace_string("aba", 100), false);
    }

    #[test]
    fn not_pallindrome() {
        let cfg = r#"
            S->aSa
            S->bSb
            S->aTb
            S->bTa
            T->aT
            T->bT
            T->!
        "#.parse::<CFG>().unwrap();

        let pda = PDA::from(cfg);
        assert_eq!(pda.trace_string("ab", 100), true);
        assert_eq!(pda.trace_string("aaaabb", 100), true);
        assert_eq!(pda.trace_string("abababb", 100), true);
        assert_ne!(pda.trace_string("ababa", 100), false);
        assert_ne!(pda.trace_string("a", 100), false);
    }
}
