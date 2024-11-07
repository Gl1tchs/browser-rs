use crate::lexer::*;

#[derive(Debug)]
pub enum Node {
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);

        // TODO: if this fails print a good error message
        let tokens = lexer.lex().unwrap();

        assert!(Lexer::validate(&tokens));

        Self { tokens }
    }

    pub fn parse(&self) -> Option<Vec<Node>> {
        let mut elements = Vec::new();
        let mut index = 0;

        while index < self.tokens.len() {
            match &self.tokens[index] {
                Token::TagBegin(tag) => elements.push(self.parse_element(tag.clone(), &mut index)),
                Token::EOF => break,
                _ => index += 1,
            }
        }

        Some(elements)
    }

    fn parse_element(&self, tag: String, index: &mut usize) -> Node {
        let mut children = Vec::new();
        let mut attributes = Vec::new();

        *index += 1;

        while *index < self.tokens.len() {
            match &self.tokens[*index] {
                Token::TagBegin(child_tag) => {
                    // do not add children if the tag is self contained
                    if Lexer::is_tag_self_closing(tag.as_str()) {
                        break;
                    }

                    children.push(self.parse_element(child_tag.clone(), index))
                }
                Token::TagEnd(_) => {
                    *index += 1;
                    break;
                }
                Token::Attribute(attribute) => {
                    attributes.push(attribute.clone());
                    *index += 1;
                }
                Token::Content(content) => {
                    children.push(Node::Text(content.clone()));
                    *index += 1;
                }
                Token::EOF => break,
            }
        }

        Node::Element {
            tag,
            attributes,
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_element() {
        let input = "<html></html>";
        let parser = Parser::new(input);
        let nodes = parser.parse().expect("Parsing failed");

        assert_eq!(nodes.len(), 1);
        if let Node::Element {
            tag,
            attributes,
            children,
        } = &nodes[0]
        {
            assert_eq!(tag, "html");
            assert!(attributes.is_empty());
            assert!(children.is_empty());
        } else {
            panic!("Expected an Element node");
        }
    }

    #[test]
    fn test_parse_element_with_text() {
        let input = "<h1>Hello, World!</h1>";
        let parser = Parser::new(input);
        let nodes = parser.parse().expect("Parsing failed");

        assert_eq!(nodes.len(), 1);
        if let Node::Element {
            tag,
            attributes,
            children,
        } = &nodes[0]
        {
            assert_eq!(tag, "h1");
            assert!(attributes.is_empty());
            assert_eq!(children.len(), 1);

            if let Node::Text(content) = &children[0] {
                assert_eq!(content, "Hello, World!");
            } else {
                panic!("Expected a Text node");
            }
        } else {
            panic!("Expected an Element node");
        }
    }

    #[test]
    fn test_parse_nested_elements() {
        let input = "<div><p>Paragraph</p></div>";
        let parser = Parser::new(input);
        let nodes = parser.parse().expect("Parsing failed");

        assert_eq!(nodes.len(), 1);
        if let Node::Element { tag, children, .. } = &nodes[0] {
            assert_eq!(tag, "div");
            assert_eq!(children.len(), 1);

            if let Node::Element {
                tag: child_tag,
                children: child_children,
                ..
            } = &children[0]
            {
                assert_eq!(child_tag, "p");
                assert_eq!(child_children.len(), 1);

                if let Node::Text(content) = &child_children[0] {
                    assert_eq!(content, "Paragraph");
                } else {
                    panic!("Expected a Text node");
                }
            } else {
                panic!("Expected a nested Element node");
            }
        } else {
            panic!("Expected an Element node");
        }
    }

    #[test]
    fn test_parse_element_with_attributes() {
        let input = r#"<img src="image.png" alt="An image"/>"#;
        let parser = Parser::new(input);
        let nodes = parser.parse().expect("Parsing failed");

        assert_eq!(nodes.len(), 1);
        if let Node::Element {
            tag,
            attributes,
            children,
        } = &nodes[0]
        {
            assert_eq!(tag, "img");
            assert_eq!(attributes.len(), 2);
            assert!(children.is_empty());

            assert_eq!(attributes[0], ("src".to_string(), "image.png".to_string()));
            assert_eq!(attributes[1], ("alt".to_string(), "An image".to_string()));
        } else {
            panic!("Expected an Element node");
        }
    }

    #[test]
    fn test_parse_multiple_elements() {
        let input = "<html><body><h1>Title</h1><p>Paragraph</p></body></html>";
        let parser = Parser::new(input);
        let nodes = parser.parse().expect("Parsing failed");

        assert_eq!(nodes.len(), 1);
        if let Node::Element { tag, children, .. } = &nodes[0] {
            assert_eq!(tag, "html");
            assert_eq!(children.len(), 1);

            if let Node::Element {
                tag: body_tag,
                children: body_children,
                ..
            } = &children[0]
            {
                assert_eq!(body_tag, "body");
                assert_eq!(body_children.len(), 2);

                if let Node::Element {
                    tag: h1_tag,
                    children: h1_children,
                    ..
                } = &body_children[0]
                {
                    assert_eq!(h1_tag, "h1");
                    assert_eq!(h1_children.len(), 1);

                    if let Node::Text(content) = &h1_children[0] {
                        assert_eq!(content, "Title");
                    } else {
                        panic!("Expected a Text node");
                    }
                } else {
                    panic!("Expected an h1 Element node");
                }

                if let Node::Element {
                    tag: p_tag,
                    children: p_children,
                    ..
                } = &body_children[1]
                {
                    assert_eq!(p_tag, "p");
                    assert_eq!(p_children.len(), 1);

                    if let Node::Text(content) = &p_children[0] {
                        assert_eq!(content, "Paragraph");
                    } else {
                        panic!("Expected a Text node");
                    }
                } else {
                    panic!("Expected a p Element node");
                }
            } else {
                panic!("Expected a body Element node");
            }
        } else {
            panic!("Expected an html Element node");
        }
    }
}
