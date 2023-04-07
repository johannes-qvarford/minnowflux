use crate::miniflux::MinifluxContext;

const BLOCKLIST_FILE: &str = include_str!("blocklist.txt");

pub(crate) fn perform(context: &MinifluxContext) {
    let feeds = context.fetch_feeds();

    let rewrite_rules = rewrite_rules();
    println!("REWRITE RULES: {rewrite_rules}");

    // (?i)(A|B|C)
    // from file:
    // A
    // B
    // C
    let blocklist_rules = BLOCKLIST_FILE.split("\n").collect::<Vec<_>>();
    let blocklist_rules = blocklist_rules.join("|");
    let blocklist_rules = format!("(?i)({blocklist_rules})");
    println!("BLOCKLIST RULES: {blocklist_rules}");

    // Makes the libreddit feed entries look pretty when you "download" them in miniflux.
    // We may still wish to enhance the feeds in giraffeed with some of this info to make kangaszuru's life easier.
    let feeds = feeds.into_iter()
        .filter(|feed| feed.feed_url.contains("nitter"))
        .map(|mut feed| {
            feed.scraper_rules = ".main-thread, .replies".to_owned();
            feed.rewrite_rules = rewrite_rules.clone();
            feed.blocklist_rules = blocklist_rules.clone();
            feed
        });

    for feed in feeds {
        context.update_feed(feed);
    }
}

struct Replacement {
    tag: &'static str,
    class: &'static str,
    prefix: &'static str
}

impl Replacement {
    fn rewrite_rule(&self) -> String {
        let Replacement { tag, class, prefix } = *self;

        let original = format!("<{tag} class=\\\"{class}");
        let rule = format!("replace(\"{original}\"|\"{prefix}{original}\")");
        rule
    }
}

fn rewrite_rules() -> String {
    // Since Miniflux strips the styling from the scraped html, we need to manually indicate when line breaks should occur.
    let replacements = [
        Replacement { tag: "div", class: "tweet-content", prefix: "<br>" },
        Replacement { tag: "div", class: "timeline-item", prefix: "<br><br>" },
        // Needed to add the space to distinguish from the class "replying-to".
        // The hash chars are just there as a visual line separator. There is an ideal amount which fills an entire single line, but that is resolution and font-specific, and all styling is removed in the end.
        Replacement { tag: "div", class: "reply ", prefix: concat!("<br><br>", "####################################################") },
        Replacement { tag: "div", class: "replying-to", prefix: "<br>" },
    ];
    let rules = replacements.into_iter()
        .map(|repl| repl.rewrite_rule())
        .collect::<Vec<_>>();
    rules.join(",")
}