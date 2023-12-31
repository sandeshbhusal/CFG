//! Tests for custom grammars

#[cfg(test)]
mod tests {
    use cfg::cfg::CFG;

    #[test]
    fn derives_empty_string() {
        let cfg = r#"
            S->!
        "#.parse::<CFG>().unwrap();

        assert_eq!(cfg.trace_string("", 100), true);
    }

    #[test]
    fn sum() {
        let cfg = r#"
            S->T+T
            T->1
            T->2
            T->S
        "#.parse::<CFG>().unwrap();

        assert_eq!(cfg.trace_string("1+1+1+1", 100), true);
        assert_eq!(cfg.trace_string("1+1", 100), true);
        assert_eq!(cfg.trace_string("1+2+1+2", 100), true);
        assert_eq!(cfg.trace_string("+2+1+2", 100), false);
        assert_eq!(cfg.trace_string("+", 100), false);
    }
}
