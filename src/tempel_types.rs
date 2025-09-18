use std::{collections::HashMap, path::Path};

pub type TempelResult<T> = std::result::Result<T, TempelError>;

#[derive(Debug)]
pub enum TempelError {
    TemplateRead(std::io::Error),

    UnbalancedBraces,

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

#[derive(Debug)]
pub struct Template {
    content: String,
    variables: Vec<String>,
}

impl Template {
    fn parse_vars(&mut self) -> TempelResult<()> {
        let starts: Vec<_> = self.content.match_indices("{{").collect();
        let ends: Vec<_> = self.content.match_indices("}}").collect();

        if starts.len() != ends.len() {
            return Err(TempelError::UnbalancedBraces);
        }

        for x in 0..starts.len() - 1 {
            if ends[x].0 > starts[x + 1].0 || ends[x].0 < starts[x].0 {
                return Err(TempelError::FormatError {
                    start: starts[x].0,
                    end: ends[x].0,
                });
            }

            let tmp = &self.content[starts[x].0 + 2..ends[x].0];

            if tmp
                .matches(char::is_whitespace)
                .collect::<Vec<_>>()
                .is_empty()
            {
                self.variables.push(tmp.to_string());
            }
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

    fn replace_vars(mut template: String, vars: HashMap<String, TempelVar>) -> String {
        for (key, val) in vars.iter() {
            template = template.replace(&format!("{{{{{}}}}}", key), &val.as_string());
        }
        template
    }

    pub fn render(&self, vars: HashMap<String, TempelVar>) -> String {
        Self::replace_vars(self.content.clone(), vars)
    }
}
