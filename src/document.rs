/// The basic building block of an XML document.
///
/// Each element must have *all* of its attributes.
pub struct Element {
  name: Name,
  context: Context,
  attributes: Vec<Attribute>,
  children: Vec<ElementChildren>,
}


impl Element {
  pub fn new(name: &str, ctx: &Context, attrs: &[Attribute]) -> Element {
    Element {
      name: Name::from_str(name, ""), // TODO: set namespace_uri correctly.
      context: context::clone(),
      attributes: Vec::from(attrs),
      children: Vec::new(),
    }
  }
}


pub struct Attribute {
  name: String,
  value: String,
}


enum ElementChildren {
  El(Element),
  Text(String)
}


struct Name {
  namespace_uri: NamespaceUri,
  local_name: String
}


// TODO: from string
impl Name {
  pub fn from_str(local_name: &str, namespace_uri: &str) -> Name {
    Name {
      local_name: String::from(local_name),
      namespace_uri: NamespaceUri(String::from(namespace_uri)),
    }
  }
}


/// Represents the namespace URI
///
/// > a string representing the namespace URI; the empty string has special
/// > significance, representing the absence of any namespace
pub struct NamespaceUri(String);


pub struct Context {
  uri: String,
  namespace_map: NamespaceMap,
}

impl Context {}


struct NamespaceMap {}
