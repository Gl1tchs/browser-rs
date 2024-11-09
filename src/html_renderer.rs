use glium::Display;
use regex::Regex;

use glyph_brush::{HorizontalAlign, VerticalAlign};
use html::parser::{Node, Parser};

use crate::renderer::{get_line_height_of_text, Renderer, TextDrawConfig};

#[derive(Debug, PartialEq, Eq)]
pub enum HtmlElement {
    Html,
    Header,
    Body,
    Div,
    Img,
    H1,
    H2,
    H3,
    Paragraph,
    Content,
    Unknown,
}

#[derive(Debug)]
pub struct HtmlElementLayout {
    h_align: HorizontalAlign,
    v_align: VerticalAlign,
}

#[derive(Debug)]
pub struct RenderNode {
    position: (u32, u32), // row, column
    element: HtmlElement,
    content: Option<String>,
    fg_color: [f32; 4],
    bg_color: [f32; 4],
    layout: HtmlElementLayout,
    children: Vec<RenderNode>,
}

pub struct HtmlRenderGraph {
    pub nodes: Vec<RenderNode>,
}

impl HtmlRenderGraph {
    pub fn new(input: &str) -> Self {
        let parser = Parser::new(input);
        let nodes = parser.parse().unwrap_or(Vec::new());

        // parse attributes and build render tree
        let mut render_nodes = Vec::with_capacity(nodes.len());
        let mut last_line: u32 = 0;
        for node in nodes {
            if let Some(render_node) = HtmlRenderGraph::parse_node(
                &node,
                (0, last_line),
                &mut last_line,
            ) {
                render_nodes.push(render_node);
            }
        }

        Self {
            nodes: render_nodes,
        }
    }

    fn parse_node(
        node: &Node,
        _parent_position: (u32, u32),
        last_line: &mut u32,
    ) -> Option<RenderNode> {
        match &node {
            Node::Element {
                tag,
                attributes,
                children,
            } => {
                let element = match tag.to_lowercase().as_str() {
                    "html" => HtmlElement::Html,
                    "header" => HtmlElement::Header,
                    "body" => HtmlElement::Body,
                    "div" => HtmlElement::Div,
                    "img" => HtmlElement::Img,
                    "h1" => HtmlElement::H1,
                    "h2" => HtmlElement::H2,
                    "h3" => HtmlElement::H3,
                    "p" => HtmlElement::Paragraph,
                    "content" => HtmlElement::Content,
                    _ => HtmlElement::Unknown,
                };

                if element == HtmlElement::Unknown {
                    return None;
                }

                let content = if let [Node::Text(text)] = &children[..] {
                    Some(text.clone())
                } else {
                    None
                };

                // style = "color: #ffaa00
                let style = attributes.get("style");

                // TODO: bg color should persist between childs
                let fg_color = if let Some(style) = style {
                    parse_style(style, "color").as_deref().and_then(hex_to_rgba)
                } else {
                    None
                };
                let bg_color = if let Some(style) = style {
                    parse_style(style, "background-color")
                        .as_deref()
                        .and_then(hex_to_rgba)
                } else {
                    None
                };

                // position for childs
                let position = (0, *last_line);
                *last_line += 1;

                let mut render_children: Vec<RenderNode> = Vec::new();
                for child in children {
                    if let Some(render_node) =
                        HtmlRenderGraph::parse_node(child, position, last_line)
                    {
                        render_children.push(render_node);
                    }
                }

                let render_node = RenderNode {
                    // TODO: make child positions relative to their parents using
                    // parent_position
                    position,
                    element,
                    content,
                    fg_color: fg_color.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                    bg_color: bg_color.unwrap_or([0.0, 0.0, 0.0, 0.0]),
                    layout: HtmlElementLayout {
                        h_align: HorizontalAlign::Left,
                        v_align: VerticalAlign::Top,
                    },
                    children: render_children,
                };

                Some(render_node)
            }
            _ => None,
        }
    }
}

pub struct HtmlRenderer {
    render_graph: Option<HtmlRenderGraph>,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self { render_graph: None }
    }

    pub fn load_html(&mut self, html: &str) {
        self.render_graph = Some(HtmlRenderGraph::new(html));
    }

    pub fn render(&self, renderer: &mut Renderer, display: &mut Display) {
        if let Some(render_graph) = &self.render_graph {
            let mut line_height: f32 = 0.0;
            for node in &render_graph.nodes {
                self.render_node(node, renderer, display, &mut line_height);
            }
        }
    }

    fn render_node(
        &self,
        node: &RenderNode,
        renderer: &mut Renderer,
        display: &mut Display,
        line_height: &mut f32,
    ) {
        // TODO: only draw background if there is background color
        // draw the element if is there a content
        if let Some(content) = &node.content {
            let font_size = match node.element {
                HtmlElement::H1 => 32.0,
                HtmlElement::H2 => 28.0,
                HtmlElement::H3 => 24.0,
                HtmlElement::Paragraph | HtmlElement::Content => 16.0,
                _ => 14.0,
            };

            // Draw the text with provided styles and layout
            renderer.draw_text(
                display,
                content,
                font_size,
                TextDrawConfig {
                    screen_pos: (0.0, *line_height),
                    fg_color: node.fg_color,
                    bg_color: node.bg_color,
                    h_align: node.layout.h_align,
                    v_align: node.layout.v_align,
                    ..Default::default()
                },
            );

            *line_height += get_line_height_of_text(content, font_size);
        }

        for child in &node.children {
            self.render_node(child, renderer, display, line_height);
        }
    }
}

pub fn parse_style(style: &str, property: &str) -> Option<String> {
    let pattern =
        format!(r"(^|\s*;\s*){}\s*:\s*([^;]+)", regex::escape(property));
    let re = Regex::new(&pattern).unwrap();

    re.captures(style)
        .and_then(|cap| cap.get(2).map(|m| m.as_str().trim().to_string()))
}

pub fn hex_to_rgba(hex: &str) -> Option<[f32; 4]> {
    let hex = if hex.starts_with('#') { &hex[1..] } else { hex };

    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]) // Alpha set to 1.0
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_parsing() {
        let style = "my-style: 15; my-other-style: 'hello'";

        assert_eq!(parse_style(style, "my-style").unwrap(), "15");
        assert_eq!(parse_style(style, "my-other-style").unwrap(), "'hello'");

        let style2 = "background-color: #ff0000";

        assert_eq!(parse_style(style2, "background-color").unwrap(), "#ff0000");
        assert_eq!(parse_style(style2, "color").is_none(), true);
    }

    #[test]
    fn test_hex_to_rgba() {
        let color = "#ffaa00";
        assert_eq!(hex_to_rgba(color).unwrap(), [1.0, 0.6666667, 0.0, 1.0]);
    }
}
