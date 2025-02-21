use std::{
    fs::File,
    io::{Error, ErrorKind, Read as _},
    path::Path,
};

use markdown::mdast::Node;
use serde::Deserialize;

pub fn read_file<T>(path: impl AsRef<Path>) -> Result<(T, String), Box<dyn std::error::Error>>
where
    for<'a> T: Deserialize<'a>,
{
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let options = markdown::ParseOptions {
        constructs: markdown::Constructs {
            frontmatter: true,
            ..markdown::Constructs::gfm()
        },
        ..Default::default()
    };

    let mdast = markdown::to_mdast(
            &buf,
            &options,
        )
        .unwrap(/* non-mdx md doesn't error */);

    let Some(Node::Toml(frontmatter)) = mdast.children().and_then(|c| c.first()) else {
        return Err("could not find frontmatter".into());
    };
    let frontmatter: T = toml::from_str(&frontmatter.value)?;
    let body = markdown::to_html_with_options(
        &buf,
        &markdown::Options {
            parse: options,
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..Default::default()
            },
        },
    )
    .map_err(|e| e.to_string())?;
    Ok((frontmatter, body))
}
