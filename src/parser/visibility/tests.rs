use crate::node::visibility::visibility::Visibility;
use crate::parser::visibility::visibility_parser;
use crate::utils::span::Span;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_parser_public() {
        let input = Span::new("pub");
        let result = visibility_parser(input);
        assert!(result.is_ok());
        let (_, visibility) = result.unwrap();
        assert!(matches!(visibility, Visibility::Public));
    }

    #[test]
    fn test_visibility_parser_public_with_whitespace_before() {
        let input = Span::new("  pub");
        let result = visibility_parser(input);
        assert!(result.is_ok());
        let (_, visibility) = result.unwrap();
        assert!(matches!(visibility, Visibility::Public));
    }

    #[test]
    fn test_visibility_parser_public_with_whitespace_after() {
        let input = Span::new("pub  ");
        let result = visibility_parser(input);
        assert!(result.is_ok());
        let (_, visibility) = result.unwrap();
        assert!(matches!(visibility, Visibility::Public));
    }

    #[test]
    fn test_visibility_parser_private() {
        let input = Span::new("");
        let result = visibility_parser(input);
        assert!(result.is_ok());
        let (_, visibility) = result.unwrap();
        assert!(matches!(visibility, Visibility::Private));
    }
}
