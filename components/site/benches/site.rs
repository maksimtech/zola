//! Benchmarking the loading and the rendering of a site.
//!
//! The site used is `test_site`, the fixture at the root of the repository that is
//! also used by the integration tests, so the benchmarks stay reproducible.
use std::path::{Path, PathBuf};

use content::Paginator;
use divan::Bencher;
use site::Site;
use tempfile::{TempDir, tempdir};

fn main() {
    divan::main();
}

const SITE: &str = "test_site";

fn site_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().join(name)
}

fn new_site(name: &str) -> Site {
    let path = site_path(name);
    let config_file = path.join("config.toml");
    Site::new(&path, &config_file).unwrap()
}

/// A site that has been loaded and whose output path points to a temporary directory.
/// The temp dir is returned so it is not deleted while the benchmark runs.
fn loaded_site(name: &str) -> (Site, TempDir) {
    let mut site = new_site(name);
    site.load().unwrap();
    let tmp_dir = tempdir().expect("create temp dir");
    site.set_output_path(tmp_dir.path().join("public"));
    (site, tmp_dir)
}

/// Parsing the config and loading the templates
#[divan::bench]
fn new(bencher: Bencher) {
    let path = site_path(SITE);
    let config_file = path.join("config.toml");
    bencher.bench_local(|| Site::new(&path, &config_file).unwrap());
}

/// Reading all the content files and parsing them
#[divan::bench]
fn load(bencher: Bencher) {
    bencher.with_inputs(|| new_site(SITE)).bench_local_refs(|site| site.load().unwrap());
}

#[divan::bench]
fn render_markdown(bencher: Bencher) {
    let (mut site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.render_markdown().unwrap());
}

#[divan::bench]
fn populate_sections(bencher: Bencher) {
    let (mut site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.populate_sections());
}

#[divan::bench]
fn populate_taxonomies(bencher: Bencher) {
    let (mut site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.populate_taxonomies().unwrap());
}

#[divan::bench]
fn render_sections(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.render_sections().unwrap());
}

#[divan::bench]
fn render_taxonomies(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.render_taxonomies().unwrap());
}

#[divan::bench]
fn render_sitemap(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.render_sitemap().unwrap());
}

#[divan::bench]
fn render_feeds(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| {
        site.render_feeds(
            site.library.read().unwrap().pages.values().collect(),
            None,
            &site.config.default_language,
            |c| c,
        )
        .unwrap()
    });
}

#[divan::bench]
fn render_aliases(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.render_aliases().unwrap());
}

#[divan::bench]
fn render_paginated(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    let library = site.library.read().unwrap();
    let section = library
        .sections
        .values()
        .find(|s| s.meta.paginate_by.is_some())
        .expect("a paginated section in the test site");
    let paginator = Paginator::from_section(section, &library);
    bencher.bench_local(|| site.render_paginated(Vec::new(), &paginator).unwrap());
}

/// The whole `zola build`: rendering everything and writing it to disk
#[divan::bench]
fn build(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site(SITE);
    bencher.bench_local(|| site.build().unwrap());
}

/// Loading and building an i18n site
#[divan::bench]
fn build_i18n(bencher: Bencher) {
    let (site, _tmp_dir) = loaded_site("test_site_i18n");
    bencher.bench_local(|| site.build().unwrap());
}
