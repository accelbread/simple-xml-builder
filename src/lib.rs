//! `simple_xml_builder` provides basic functionality for building and
//! outputting XML documents.
//!
//! The constructed model is write-only.
//!
//! # Usage
//!
//! Use [XMLElement] to create elements with tags,
//! attributes, and either text or children.
//! You can write an XML document by calling
//! [write](XMLElement::write) on your root element.
//!
//! # Example
//!
//! ```rust
//! # use std::io;
//! # fn main() -> io::Result<()> {
//! use std::fs::File;
//! use simple_xml_builder::XMLElement;
//!
//! # /*
//! let mut file = File::create("sample.xml")?;
//! # */
//! # let mut file: Vec<u8> = Vec::new();
//!
//! let mut person = XMLElement::new("person");
//! person.add_attribute("id", "232");
//! let mut name = XMLElement::new("name");
//! name.add_text("Joe Schmoe");
//! person.add_child(name);
//! let mut age = XMLElement::new("age");
//! age.add_text(24); // `add_text` and `add_attribute` accept any type
//!                   // that implements `ToString` (or `Display`)
//! person.add_child(age);
//! let hobbies = XMLElement::new("hobbies");
//! person.add_child(hobbies);
//!
//! person.write(file)?;
//! # Ok(())
//! # }
//! ```
//! `sample.xml` will contain:
//! ```xml
//! <?xml version = "1.0" encoding = "UTF-8"?>
//! <person id="232">
//!     <name>Joe Schmoe</name>
//!     <age>24</age>
//!     <hobbies />
//! </person>
//! ```

#![doc(html_root_url = "https://docs.rs/simple-xml-builder/1.1.0")]

extern crate indexmap;
use indexmap::IndexMap;
use std::fmt;
use std::io::{self, Write};

/// Represents an XML element
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XMLElement {
    name: String,
    attributes: IndexMap<String, String>,
    content: XMLElementContent,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum XMLElementContent {
    Empty,
    Elements(Vec<XMLElement>),
    Text(String),
}

impl fmt::Display for XMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: Vec<u8> = Vec::new();
        self.write(&mut s)
            .expect("Failure writing output to Vec<u8>");
        write!(f, "{}", unsafe { String::from_utf8_unchecked(s) })
    }
}

impl XMLElement {
    /// Creates a new empty XML element using the given name for the tag.
    pub fn new(name: impl ToString) -> Self {
        XMLElement {
            name: name.to_string(),
            attributes: IndexMap::new(),
            content: XMLElementContent::Empty,
        }
    }

    /// Adds an attribute to the XML element. The attribute value can take any
    /// type which implements [`fmt::Display`].
    pub fn add_attribute(&mut self, name: impl ToString, value: impl ToString) {
        self.attributes
            .insert(name.to_string(), escape_str(&value.to_string()));
    }

    /// Adds a child element to the XML element.
    /// The new child will be placed after previously added children.
    ///
    /// This method may only be called on an element that has children or is
    /// empty.
    ///
    /// # Panics
    ///
    /// Panics if the element contains text.
    pub fn add_child(&mut self, child: XMLElement) {
        use XMLElementContent::*;
        match self.content {
            Empty => {
                self.content = Elements(vec![child]);
            }
            Elements(ref mut list) => {
                list.push(child);
            }
            Text(_) => {
                panic!("Attempted adding child element to element with text.");
            }
        }
    }

    /// Adds text to the XML element.
    ///
    /// This method may only be called on an empty element.
    ///
    /// # Panics
    ///
    /// Panics if the element is not empty.
    pub fn add_text(&mut self, text: impl ToString) {
        use XMLElementContent::*;
        match self.content {
            Empty => {
                self.content = Text(escape_str(&text.to_string()));
            }
            _ => {
                panic!("Attempted adding text to non-empty element.");
            }
        }
    }

    /// Outputs a UTF-8 XML document, where this element is the root element.
    ///
    /// Output is properly indented.
    ///
    /// # Errors
    ///
    /// Returns Errors from writing to the Write object.
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(writer, r#"<?xml version = "1.0" encoding = "UTF-8"?>"#)?;
        self.write_level(&mut writer, 0)
    }

    fn write_level<W: Write>(&self, writer: &mut W, level: usize) -> io::Result<()> {
        use XMLElementContent::*;
        let prefix = "\t".repeat(level);
        match &self.content {
            Empty => {
                writeln!(
                    writer,
                    "{}<{}{} />",
                    prefix,
                    self.name,
                    self.attribute_string()
                )?;
            }
            Elements(list) => {
                writeln!(
                    writer,
                    "{}<{}{}>",
                    prefix,
                    self.name,
                    self.attribute_string()
                )?;
                for elem in list {
                    elem.write_level(writer, level + 1)?;
                }
                writeln!(writer, "{}</{}>", prefix, self.name)?;
            }
            Text(text) => {
                writeln!(
                    writer,
                    "{}<{}{}>{}</{1}>",
                    prefix,
                    self.name,
                    self.attribute_string(),
                    text
                )?;
            }
        }
        Ok(())
    }

    fn attribute_string(&self) -> String {
        if self.attributes.is_empty() {
            "".to_owned()
        } else {
            let mut result = "".to_owned();
            for (k, v) in &self.attributes {
                result = result + &format!(r#" {}="{}""#, k, v);
            }
            result
        }
    }
}

fn escape_str(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use XMLElement;

    #[test]
    fn write_xml() {
        let mut root = XMLElement::new("root");
        let mut child1 = XMLElement::new("child1");
        let inner1 = XMLElement::new("inner");
        child1.add_child(inner1);
        let mut inner2 = XMLElement::new("inner");
        inner2.add_text("Example Text\nNew line");
        child1.add_child(inner2);
        root.add_child(child1);
        let mut child2 = XMLElement::new("child2");
        child2.add_attribute("at1", "test &");
        child2.add_attribute("at2", "test <");
        child2.add_attribute("at3", "test \"");
        let mut inner3 = XMLElement::new("inner");
        inner3.add_attribute("test", "example");
        child2.add_child(inner3);
        root.add_child(child2);
        let mut child3 = XMLElement::new("child3");
        child3.add_text("&< &");
        root.add_child(child3);
        let mut child4 = XMLElement::new("child4");
        child4.add_attribute("non-str-attribute", 5);
        child4.add_text(6);
        root.add_child(child4);

        let expected = r#"<?xml version = "1.0" encoding = "UTF-8"?>
<root>
	<child1>
		<inner />
		<inner>Example Text
New line</inner>
	</child1>
	<child2 at1="test &amp;" at2="test &lt;" at3="test &quot;">
		<inner test="example" />
	</child2>
	<child3>&amp;&lt; &amp;</child3>
	<child4 non-str-attribute="5">6</child4>
</root>
"#;
        assert_eq!(
            format!("{}", root),
            expected,
            "Attempt to write XML did not give expected results."
        );
    }

    #[test]
    #[should_panic]
    fn add_text_to_parent_element() {
        let mut e = XMLElement::new("test");
        e.add_child(XMLElement::new("test"));
        e.add_text("example text");
    }

    #[test]
    #[should_panic]
    fn add_child_to_text_element() {
        let mut e = XMLElement::new("test");
        e.add_text("example text");
        e.add_child(XMLElement::new("test"));
    }
}
