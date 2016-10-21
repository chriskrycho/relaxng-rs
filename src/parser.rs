use collections::str::FromStr;

enum Token {
  Keyword(Keyword),
  ElementName(String),
  OpenBrace,
  CloseBrace,
}

enum Declaration {
  Namespace { id_or_keyword: IdOrKeyword, namespace_uri_literal: String }
}


enum IdOrKeyword {

}


enum Keyword {
  Attribute,
  Default,
  Datatypes,
  Div,
  Element,
  Empty,
  External,
  Grammar,
  Include,
  Inherit,
  List,
  Mixed,
  Namespace,
  NotAllowed,
  Parent,
  Start,
  String,
  Text,
  Token,
}


impl FromStr for Keyword {
  type Err = ();
  fn from_str(s: &str) -> Result<Keyword, Self::Err> {
    match s {
      "attribute" => Ok(Keyword::Attribute),
      "default" => Ok(Keyword::Default),
      "datatypes" => Ok(Keyword::Datatypes),
      "div" => Ok(Keyword::Div),
      "element" => Ok(Keyword::Element),
      "empty" => Ok(Keyword::Empty),
      "external" => Ok(Keyword::External),
      "grammar" => Ok(Keyword::Grammar),
      "include" => Ok(Keyword::Include),
      "inherit" => Ok(Keyword::Inherit),
      "list" => Ok(Keyword::List),
      "mixed" => Ok(Keyword::Mixed),
      "namespace" => Ok(Keyword::Namespace),
      "notAllowed" => Ok(Keyword::NotAllowed),
      "parent" => Ok(Keyword::Parent),
      "start" => Ok(Keyword::Start),
      "string" => Ok(Keyword::String),
      "text" => Ok(Keyword::Text),
      "token" => Ok(Keyword::Token),
      _ => Err(),
    }
  }
}

