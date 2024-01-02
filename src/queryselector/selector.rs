use crate::{Node, Parser};

/// A single query selector node
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Selector<'a> {
    /// Tag selector: foo
    Tag(&'a [u8]),
    /// ID selector: #foo
    Id(&'a [u8]),
    /// Class selector: .foo
    Class(&'a [u8]),
    /// All selector: *
    All,
    /// And combinator: .foo.bar
    And(Box<Selector<'a>>, Box<Selector<'a>>),
    /// Or combinator: .foo, .bar
    Or(Box<Selector<'a>>, Box<Selector<'a>>),
    /// Descendant combinator: .foo .bar
    Descendant(Box<Selector<'a>>, Box<Selector<'a>>),
    /// Parent combinator: .foo > .bar
    Parent(Box<Selector<'a>>, Box<Selector<'a>>),
    /// Attribute: [foo]
    Attribute(&'a [u8]),
    /// Attribute with value: [foo=bar]
    AttributeValue(&'a [u8], &'a [u8]),
    /// Attribute with whitespace-separated list of values that contains a value: [foo~=bar]
    AttributeValueWhitespacedContains(&'a [u8], &'a [u8]),
    /// Attribute with value that starts with: [foo^=bar]
    AttributeValueStartsWith(&'a [u8], &'a [u8]),
    /// Attribute with value that ends with: [foo$=bar]
    AttributeValueEndsWith(&'a [u8], &'a [u8]),
    /// Attribute with value that contains: [foo*=bar]
    AttributeValueSubstring(&'a [u8], &'a [u8]),
}

impl<'a> Selector<'a> {
    /// Checks if the given node matches this selector
    pub fn matches<'b>(&self, node: &Node<'b>, parser: &Parser<'b>) -> bool {
        match self {
            Self::Tag(tag) => node.as_tag().map_or(false, |t| t._name.as_bytes().eq(*tag)),
            Self::Id(id) => node
                .as_tag()
                .map_or(false, |t| t._attributes.id == Some((*id).into())),
            Self::Class(class) => node
                .as_tag()
                .map_or(false, |t| t._attributes.is_class_member(*class)),
            Self::And(a, b) => a.matches(node, parser) && b.matches(node, parser),
            Self::Or(a, b) => a.matches(node, parser) || b.matches(node, parser),
            Self::All => true,
            Self::Parent(a, b) => node
                .as_tag()
                .and_then(|tag| {
                    Some(a.matches(tag.parent().get(parser)?, parser) && b.matches(node, parser))
                })
                .unwrap_or_default(),
            Self::Descendant(a, b) => {
                if !b.matches(node, parser) {
                    return false
                }
                let mut curr = node;
                while let Some(ancestor) = curr.as_tag().and_then(|t| t._parent?.get(parser)) {
                    if a.matches(ancestor, parser) {
                        return true
                    }
                    curr = ancestor;
                }
                false
            }
            Self::Attribute(attribute) => node
                .as_tag()
                .map_or(false, |t| t._attributes.get(*attribute).is_some()),
            Self::AttributeValue(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr == value)
            }
            Self::AttributeValueEndsWith(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr.ends_with(value))
            }
            Self::AttributeValueStartsWith(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| {
                    attr.starts_with(value)
                })
            }
            Self::AttributeValueSubstring(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| attr.contains(value))
            }
            Self::AttributeValueWhitespacedContains(attribute, value) => {
                check_attribute(node, attribute, value, |attr, value| {
                    attr.split_whitespace().any(|x| x == value)
                })
            }
        }
    }
}

fn check_attribute<F>(node: &Node, attribute: &[u8], value: &[u8], callback: F) -> bool
where
    F: Fn(&str, &str) -> bool,
{
    node.as_tag().map_or(false, |t| {
        t._attributes
            .get(attribute)
            .flatten()
            .map_or(false, |attr| {
                callback(&attr.as_utf8_str(), &String::from_utf8_lossy(value))
            })
    })
}
