use crate::miniflux::MinifluxContext;

pub(crate) fn perform(context: &MinifluxContext) {
    let feeds = context.fetch_feeds();

    // Makes the libreddit feed entries look pretty when you "download" them in miniflux.
    // We may still wish to enhance the feeds in giraffeed with some of this info to make kangaszuru's life easier.
    let feeds = feeds.into_iter()
        .filter(|feed| feed.feed_url.contains("libreddit"))
        .map(|mut feed| {
            feed.scraper_rules = ".post_media_content, .post_body, .gallery, #post_url, .thread".to_owned();
            feed.rewrite_rules = "remove(\".comment_score\")".to_owned();
            feed
        });

    for feed in feeds {
        context.update_feed(feed);
    }
}