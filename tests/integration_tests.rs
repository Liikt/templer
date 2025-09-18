use tempel::template_vars;
use tempel::{TempelVar, Template};

#[test]
fn unbalanced_template() {
    let template = Template::new("{{ {{ test }}");
    assert!(template.is_err());
}

#[test]
fn wrong_brace_order() {
    let template = Template::new("{{ {{ }} }}");
    assert!(template.is_err());

    let template = Template::new("}} {{ }} {{");
    assert!(template.is_err());
}

#[test]
fn correct_template() {
    let template = Template::new("Hello {{name}}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo"));

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn space_normalization() {
    let template = Template::new("Hello {{ name  }}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo"));

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn tab_normalization() {
    let template = Template::new("Hello {{\tname\t\t}}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo"));

    assert_eq!(rendered, "Hello foo");
}
#[test]
fn mixed_normalization() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo"));

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn missing_variable() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(std::collections::HashMap::new());

    assert_eq!(rendered, "Hello {{name}}");
}

#[test]
fn superflous_variable() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(template_vars!("foo" => "name"));

    assert_eq!(rendered, "Hello {{name}}");
}

#[test]
fn multiple_variables() {
    let template = Template::new("Hello {{\tname \t }} this is {{ other_name }}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo", "other_name" => "bar"));

    assert_eq!(rendered, "Hello foo this is bar");
}

#[test]
fn insert_list() {
    let template = Template::new("Hello to all {{ names }}").unwrap();
    let baz = String::from("baz");
    let rendered = template.render(template_vars!("names" => ["foo", "bar", baz]));

    assert_eq!(rendered, "Hello to all [foo, bar, baz]");
}
