use crate::{render_to_html, RenderOption};
use yaml_rust::*;

#[test]
fn yaml_test() {
    let sample = "---
date: 2022-07-11
author: Baehyunsol
tags: [mdxt, blog]
num test : 0
---

hello
";
    let result = render_to_html(&sample.to_string(), RenderOption::default());

    assert_eq!(String::from("<p>hello</p>"), result.content);

    let mut yaml_hash = yaml::Hash::new();
    yaml_hash.insert(Yaml::from_str("date"), Yaml::from_str("2022-07-11"));
    yaml_hash.insert(Yaml::from_str("author"), Yaml::from_str("Baehyunsol"));
    yaml_hash.insert(Yaml::from_str("tags"), Yaml::Array(vec![Yaml::from_str("mdxt"), Yaml::from_str("blog")]));
    yaml_hash.insert(Yaml::from_str("num test"), Yaml::from_str("0"));

    assert_eq!(Yaml::Hash(yaml_hash), result.metadata.unwrap());

    let sample = "---
date: 2022-07-15
tags: [mdxt, md]
---

Some extra stuff

blahblahblah";

    let result = render_to_html(&sample.to_string(), RenderOption::default());

    assert_eq!(String::from("<p>Some extra stuff</p><p>blahblahblah</p>"), result.content);

    let mut yaml_hash = yaml::Hash::new();
    yaml_hash.insert(Yaml::from_str("date"), Yaml::from_str("2022-07-15"));
    yaml_hash.insert(Yaml::from_str("tags"), Yaml::Array(vec![Yaml::from_str("mdxt"), Yaml::from_str("md")]));

    assert_eq!(Yaml::Hash(yaml_hash), result.metadata.unwrap());
}