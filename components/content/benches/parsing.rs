//! Benchmarking the parsing of pages and sections: front matter deserialization,
//! slug generation and reading analytics.
use std::path::Path;

use config::Config;
use content::{Page, Section};
use divan::Bencher;

fn main() {
    divan::main();
}

const PAGE_BODY: &str = r#"
# Modus cognitius profanam ne duae virtutis mundi

Lorem markdownum litora, care ponto nomina, et ut aspicit gelidas sui et
purpureo genuit. Tamen colla venientis [delphina](http://nil-sol.com/ecquis)
Tusci et temptata citaeque curam isto ubi vult vulnere reppulit.

- Seque vidit flendoque de quodam
- Dabit minimos deiecto caputque noctis pluma
- Leti coniunx est Helicen
- Illius pulvereumque Icare inpositos

Protinus dicunt, breve per, et vivacis genus Orphei munere. Me terram [dimittere
casside](http://corpus.org/) pervenit saxo primoque frequentat genuum sorori
praeferre causas Libys. Illud in serpit adsuetam utrimque nunc haberent,
**terrae si** veni!
"#;

const PAGE_TOML: &str = r#"
+++
title = "Hello world"
description = "A page with a fairly standard front matter"
date = 2023-05-18T09:00:00Z
updated = 2023-06-01T09:00:00Z
weight = 42
slug = "hello-world"
aliases = ["/old/hello-world", "/even-older/hello-world"]
[taxonomies]
tags = ["a", "b", "c"]
categories = ["c1"]
[extra]
author = "Someone"
featured = true
+++
"#;

const PAGE_YAML: &str = r#"
---
title: Hello world
description: A page with a fairly standard front matter
date: 2023-05-18T09:00:00Z
weight: 42
slug: hello-world
taxonomies:
  tags: ["a", "b", "c"]
  categories: ["c1"]
extra:
  author: Someone
  featured: true
---
"#;

const SECTION_TOML: &str = r#"
+++
title = "A section"
description = "Sections have their own front matter"
sort_by = "date"
paginate_by = 10
template = "section.html"
page_template = "page.html"
insert_anchor_links = "left"
[extra]
class = "index"
+++
"#;

fn page_content(front_matter: &str) -> String {
    format!("{}{}", front_matter.trim_start(), PAGE_BODY)
}

#[divan::bench]
fn parse_page_toml(bencher: Bencher) {
    let config = Config::default_for_test();
    let content = page_content(PAGE_TOML);
    let path = Path::new("content/posts/hello-world.md");
    let base_path = Path::new("");

    bencher
        .bench_local(|| Page::parse(path, divan::black_box(&content), &config, base_path).unwrap());
}

#[divan::bench]
fn parse_page_yaml(bencher: Bencher) {
    let config = Config::default_for_test();
    let content = page_content(PAGE_YAML);
    let path = Path::new("content/posts/hello-world.md");
    let base_path = Path::new("");

    bencher
        .bench_local(|| Page::parse(path, divan::black_box(&content), &config, base_path).unwrap());
}

/// Pages with a date in their filename go through an extra regex + slugification path
#[divan::bench]
fn parse_page_with_dated_filename(bencher: Bencher) {
    let config = Config::default_for_test();
    let content = page_content("+++\ntitle = \"Hello world\"\n+++\n");
    let path = Path::new("content/posts/2023-05-18-hello-world.md");
    let base_path = Path::new("");

    bencher
        .bench_local(|| Page::parse(path, divan::black_box(&content), &config, base_path).unwrap());
}

#[divan::bench]
fn parse_section(bencher: Bencher) {
    let config = Config::default_for_test();
    let content = page_content(SECTION_TOML);
    let path = Path::new("content/posts/_index.md");
    let base_path = Path::new("");

    bencher.bench_local(|| {
        Section::parse(path, divan::black_box(&content), &config, base_path).unwrap()
    });
}

/// A long page: mostly measuring the reading analytics and the content copy
#[divan::bench]
fn parse_long_page(bencher: Bencher) {
    let config = Config::default_for_test();
    let content = format!("{}{}", PAGE_TOML.trim_start(), PAGE_BODY.repeat(100));
    let path = Path::new("content/posts/long.md");
    let base_path = Path::new("");

    bencher
        .bench_local(|| Page::parse(path, divan::black_box(&content), &config, base_path).unwrap());
}
