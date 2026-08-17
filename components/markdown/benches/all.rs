use std::collections::HashMap;

use config::{Config, HighlightConfig, HighlightStyle, Highlighting, Registry};
use divan::Bencher;
use markdown::{RenderContext, render_content};
use templates::ZOLA_TERA;
use tera::Tera;
use utils::types::InsertAnchor;

fn main() {
    divan::main();
}

const CONTENT: &str = r#"
# Modus cognitius profanam ne duae virtutis mundi

## Ut vita

Lorem markdownum litora, care ponto nomina, et ut aspicit gelidas sui et
purpureo genuit. Tamen colla venientis [delphina](http://nil-sol.com/ecquis)
Tusci et temptata citaeque curam isto ubi vult vulnere reppulit.

- :one: Seque vidit flendoque de quodam
- :two: Dabit minimos deiecto caputque noctis pluma
- :three: Leti coniunx est Helicen
- :four: Illius pulvereumque Icare inpositos
- :five: Vivunt pereo pluvio tot ramos Olenios gelidis
- :six: Quater teretes natura inde

### A subsection

Protinus dicunt, breve per, et vivacis genus Orphei munere. Me terram [dimittere
casside](http://corpus.org/) pervenit saxo primoque frequentat genuum sorori
praeferre causas Libys. Illud in serpit adsuetam utrimque nunc haberent,
**terrae si** veni! Hectoreis potes sumite [Mavortis retusa](http://tua.org/)
granum captantur potuisse Minervae, frugum.

> Clivo sub inprovisoque nostrum minus fama est, discordia patrem petebat precatur
absumitur, poena per sit. Foramina *tamen cupidine* memor supplex tollentes
dictum unam orbem, Anubis caecae. Viderat formosior tegebat satis, Aethiopasque
sit submisso coniuge tristis ubi! :exclamation:

## Praeceps Corinthus totidem quem crus vultum cape

```rs
#[derive(Debug)]
pub struct Site {
    /// The base path of the gutenberg site
    pub base_path: PathBuf,
    /// The parsed config for the site
    pub config: Config,
    pub pages: HashMap<PathBuf, Page>,
    pub sections: HashMap<PathBuf, Section>,
    pub tera: Tera,
    live_reload: bool,
    output_path: PathBuf,
    static_path: PathBuf,
    pub tags: Option<Taxonomy>,
    pub categories: Option<Taxonomy>,
    /// A map of all .md files (section and pages) and their permalink
    /// We need that if there are relative links in the content that need to be resolved
    pub permalinks: HashMap<String, String>,
}
```

## More stuff
And a shortcode:

{{ youtube(id="my_youtube_id") }}

### Another subsection
Gotta make the toc do a little bit of work

# A big title :fire:

- hello
- world
- !

```py
if __name__ == "__main__":
    gen_site("basic-blog", [""], 250, paginate=True)
```
"#;

fn tera_with_shortcodes() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("shortcodes/youtube.html", "{{id}}").unwrap();
    tera
}

fn highlighting() -> Highlighting {
    let mut registry = Registry::builtin().unwrap();
    registry.link_grammars();

    Highlighting {
        error_on_missing_language: false,
        style: HighlightStyle::Inline,
        theme: HighlightConfig::Single { theme: "github-dark".to_string() },
        extra_grammars: vec![],
        extra_themes: vec![],
        registry,
    }
}

/// Renders `content` with the given config, benching only the rendering itself
fn bench_render(bencher: Bencher, content: &str, tera: &Tera, config: &Config) {
    let permalinks_ctx = HashMap::new();
    let mut context = RenderContext::new(
        tera,
        config,
        &config.default_language,
        "",
        &permalinks_ctx,
        InsertAnchor::None,
    );
    let shortcode_def = utils::templates::get_shortcodes(tera);
    context.set_shortcode_definitions(&shortcode_def);

    bencher.bench_local(|| render_content(divan::black_box(content), &context).unwrap());
}

#[divan::bench]
fn render_content_with_highlighting(bencher: Bencher) {
    let tera = tera_with_shortcodes();
    let mut config = Config::default_for_test();
    config.markdown.highlighting = Some(highlighting());
    bench_render(bencher, CONTENT, &tera, &config);
}

#[divan::bench]
fn render_content_without_highlighting(bencher: Bencher) {
    let tera = tera_with_shortcodes();
    let config = Config::default_for_test();
    bench_render(bencher, CONTENT, &tera, &config);
}

#[divan::bench]
fn render_content_no_shortcode(bencher: Bencher) {
    let tera = tera_with_shortcodes();
    let content = CONTENT.replace(r#"{{ youtube(id="my_youtube_id") }}"#, "");
    let config = Config::default_for_test();
    bench_render(bencher, &content, &tera, &config);
}

#[divan::bench]
fn render_content_with_emoji(bencher: Bencher) {
    let tera = tera_with_shortcodes();
    let content = CONTENT.replace(r#"{{ youtube(id="my_youtube_id") }}"#, "");
    let mut config = Config::default_for_test();
    config.markdown.render_emoji = true;
    bench_render(bencher, &content, &tera, &config);
}

#[divan::bench]
fn render_content_with_anchors(bencher: Bencher) {
    let mut tera = tera_with_shortcodes();
    // The anchor links rely on a builtin template
    tera.extend(&ZOLA_TERA).unwrap();
    let mut config = Config::default_for_test();
    config.markdown.highlighting = Some(highlighting());
    let permalinks_ctx = HashMap::new();
    let mut context = RenderContext::new(
        &tera,
        &config,
        &config.default_language,
        "",
        &permalinks_ctx,
        InsertAnchor::Right,
    );
    let shortcode_def = utils::templates::get_shortcodes(&tera);
    context.set_shortcode_definitions(&shortcode_def);

    bencher.bench_local(|| render_content(divan::black_box(CONTENT), &context).unwrap());
}
