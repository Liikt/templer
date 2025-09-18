use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

pub type TempelResult<T> = std::result::Result<T, TempelError>;

#[derive(Debug)]
pub enum TempelError {
    TemplateRead(std::io::Error),

    UnbalancedBraces,
    UnbalancedForLoop,

    NoSuchList(String),

    FormatError { start: usize, end: usize },
}

#[derive(Debug, Clone)]
pub enum TempelVar {
    String(String),
    List(Vec<String>),
}

impl TempelVar {
    fn as_string(&self) -> String {
        match self {
            Self::String(s) => s.to_string(),
            Self::List(l) => format!("[{}]", l.join(", ")),
        }
    }
}

struct LoopInfo {
    head: String,
    body: String,
    end: String,
}

#[derive(Debug)]
pub struct Template {
    content: String,
    variables: Vec<String>,
}

impl Template {
    fn parse_vars(&mut self) -> TempelResult<()> {
        let regex = Regex::new(r"\{\{[ \t]*([a-zA-Z_]+)[ \t]*}}").unwrap();
        for (_, [name]) in regex
            .captures_iter(&self.content)
            .map(|caps| caps.extract())
        {
            self.variables.push(name.to_string());
        }

        Ok(())
    }

    pub fn new<S: AsRef<str>>(content: S) -> TempelResult<Self> {
        let mut ret = Self {
            content: content.as_ref().to_string(),
            variables: Vec::new(),
        };

        ret.normalize_template();
        ret.parse_vars()?;

        Ok(ret)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> TempelResult<Self> {
        Self::new(std::fs::read_to_string(path).map_err(TempelError::TemplateRead)?)
    }

    fn normalize_template(&mut self) {
        let mut cur = self.content.clone();

        self.content = loop {
            let mut new = cur.replace("{{ ", "{{");
            new = new.replace("{{\t", "{{");

            new = new.replace(" }}", "}}");
            new = new.replace("\t}}", "}}");

            if cur == new {
                break new;
            }

            cur = new;
        };
    }

    fn parse_loops(
        mut template: String,
        vars: &HashMap<String, TempelVar>,
    ) -> TempelResult<String> {
        let regex = Regex::new(
            r"\{%[ \t]*for ([a-zA-Z_]+) in ([a-zA-Z_]+)[ \t]*%}(.*?)\{%[ \t]*endfor[ \t]*%}",
        )
        .unwrap();

        let mut changes = Vec::new();

        for (_, [key, list, body]) in regex.captures_iter(&template).map(|caps| caps.extract()) {
            let mut new = String::new();
            if let TempelVar::List(l) = vars
                .get(list)
                .ok_or(TempelError::NoSuchList(list.to_string()))?
            {
                for val in l {
                    let mut tmp = vars.clone();
                    tmp.insert(key.to_string(), TempelVar::String(val.to_string()));
                    new.push_str(&Self::replace_vars(body.to_string(), &tmp));
                }
            }
            changes.push((
                format!("{{% for {key} in {list} %}}{body}{{% endfor %}}"),
                new,
            ));
        }

        for (old, new) in changes {
            template = template.replace(&old, &new);
        }

        Ok(template)
    }

    fn replace_vars(mut template: String, vars: &HashMap<String, TempelVar>) -> String {
        for (key, val) in vars.iter() {
            template = template.replace(&format!("{{{{{}}}}}", key), &val.as_string());
        }
        template
    }

    pub fn render(&self, vars: HashMap<String, TempelVar>) -> TempelResult<String> {
        let template = Self::replace_vars(self.content.clone(), &vars);
        Self::parse_loops(template, &vars)
    }
}
