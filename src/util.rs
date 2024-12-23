use crate::bindings::component::golem_x_stub::stub_golem_x::Username;

impl From<String>
    for crate::bindings::exports::component::golem_x_interface::tweet_api::PostedTweet
{
    #[inline(always)]
    fn from(content: String) -> Self {
        Self {
            content,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

impl From<crate::bindings::component::golem_x_interface::tweet_api::PostedTweet>
    for crate::bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet
{
    #[inline(always)]
    fn from(tweet: crate::bindings::component::golem_x_interface::tweet_api::PostedTweet) -> Self {
        Self {
            author: crate::state::get_username().to_string(),
            content: tweet.content,
            timestamp: tweet.timestamp,
        }
    }
}

impl
    From<(
        crate::bindings::component::golem_x_interface::tweet_api::PostedTweet,
        &Username,
    )> for crate::bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet
{
    fn from(
        data: (
            crate::bindings::component::golem_x_interface::tweet_api::PostedTweet,
            &Username,
        ),
    ) -> Self {
        let (tweet, author) = data;
        crate::bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet {
            author: author.to_string(),
            content: tweet.content,
            timestamp: tweet.timestamp,
        }
    }
}
