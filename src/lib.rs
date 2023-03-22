pub struct XmlDocument<'a> {
    pub path: &'a str,
    pub root: Option<Box<&'a XmlNode<'a>>>,
}

pub struct XmlNode<'a> {
    pub tag: &'a str,
    pub value: Option<&'a str>,
    pub parent: Option<&'a XmlNode<'a>>,
    pub children: Vec<&'a XmlNode<'a>>,
}

impl<'a> XmlDocument<'a> {
    pub fn new(path: &'a str) -> Self {
        XmlDocument {
            path,
            root: Option::None,
        }
    }
}

impl<'a> XmlNode<'a> {
    pub fn new(
        parent: Option<&'a XmlNode<'a>>,
        tag: &'a str,
        value: Option<&'a str>,
        children: Vec<&'a XmlNode<'a>>,
    ) -> Self {
        XmlNode {
            parent,
            tag,
            value,
            children,
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{XmlDocument, XmlNode};

    #[test]
    fn create_xml_document() {
        let path = "./form.xml";
        let doc = XmlDocument::new(path);
        assert_eq!(doc.path, "./form.xml");
    }

    #[test]
    fn create_xml_node() {
        let node = XmlNode::new(Option::None, "root", Option::None, Vec::new());
        assert_eq!(node.tag, "root");
        assert_eq!(node.value, Option::None);
    }
}
