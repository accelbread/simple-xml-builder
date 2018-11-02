# simple-xml-builder

[![Latest version](https://img.shields.io/crates/v/simple-xml-builder.svg)](https://crates.io/crates/simple-xml-builder)
[![Documentation](https://docs.rs/simple-xml-builder/badge.svg)](https://docs.rs/simple-xml-builder)
[![License](https://img.shields.io/crates/l/simple-xml-builder.svg)](https://github.com/Accelbread/simple-xml-builder#license)

A Rust library for building and outputing xml documents. The constructed model
is write-only, and allows for writing the represented XML document.

[Documentation](https://docs.rs/simple-xml-builder)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple-xml-builder = "1.0.0"
```

and this to your crate root:

```rust
extern crate simple_xml_builder;
```

## Example

```rust
use std::fs::File;
use simple_xml_builder::XMLElement;

let mut file = File::create("sample.xml")?;

let mut person = XMLElement::new("person");
person.add_attribute("id", "232");
let mut name = XMLElement::new("name");
name.add_text("Joe Schmoe");
person.add_child(name);
let mut age = XMLElement::new("age");
age.add_text("24");
person.add_child(age);
let hobbies = XMLElement::new("hobbies");
person.add_child(hobbies);

person.write(file)?;
```

`sample.xml` will contain:

```xml
<?xml version = "1.0" encoding = "UTF-8"?>
<person id="232">
    <name>Joe Schmoe</name>
    <age>24</age>
    <hobbies />
</person>
```
