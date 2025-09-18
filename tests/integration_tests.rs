use tempel::template_vars;
use tempel::{TempelVar, Template};

#[test]
fn correct_template() {
    let template = Template::new("Hello {{name}}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo")).unwrap();

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn space_normalization() {
    let template = Template::new("Hello {{ name  }}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo")).unwrap();

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn tab_normalization() {
    let template = Template::new("Hello {{\tname\t\t}}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo")).unwrap();

    assert_eq!(rendered, "Hello foo");
}
#[test]
fn mixed_normalization() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(template_vars!("name" => "foo")).unwrap();

    assert_eq!(rendered, "Hello foo");
}

#[test]
fn missing_variable() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(std::collections::HashMap::new()).unwrap();

    assert_eq!(rendered, "Hello {{name}}");
}

#[test]
fn superflous_variable() {
    let template = Template::new("Hello {{\tname \t }}").unwrap();
    let rendered = template.render(template_vars!("foo" => "name")).unwrap();

    assert_eq!(rendered, "Hello {{name}}");
}

#[test]
fn multiple_variables() {
    let template = Template::new("Hello {{\tname \t }} this is {{ other_name }}").unwrap();
    let rendered = template
        .render(template_vars!("name" => "foo", "other_name" => "bar"))
        .unwrap();

    assert_eq!(rendered, "Hello foo this is bar");
}

#[test]
fn insert_list() {
    let template = Template::new("Hello to all {{ names }}").unwrap();
    let baz = String::from("baz");
    let rendered = template
        .render(template_vars!("names" => ["foo", "bar", baz]))
        .unwrap();

    assert_eq!(rendered, "Hello to all [foo, bar, baz]");
}

#[test]
fn simple_for_loop() {
    let template =
        Template::new("Hello from {% for name in names %}{{name}} {% endfor %}").unwrap();
    let rendered = template
        .render(template_vars!("names" => ["foo", "bar", "baz"]))
        .unwrap();

    assert_eq!(rendered, "Hello from foo bar baz ");
}

#[test]
fn complex_loop() {
    let template = Template::new(
        "Hello fellow blub!{% for blub in blab %} Greetings to {{blub}} from {{foop }}{% endfor %}",
    )
    .unwrap();
    let rendered = template
        .render(template_vars!("blab" => ["foo", "bar"], "foop" => "base"))
        .unwrap();

    assert_eq!(
        rendered,
        "Hello fellow blub! Greetings to foo from base Greetings to bar from base"
    );
}
