# Deprecated Reason

This file contains the reason of discontinuing the maintenance of certain modules of the crate
[bevy-discord](https://docs.rs/bevy-discord)

# `webhook` Module 
![maintenance-status](https://img.shields.io/badge/maintenance-deprecated-red.svg)
**Please Use `http` module instead of `webhook`**

This module was up until v0.2.2 and will be deprecated in the next release i.e. v0.3.0 _(Not released at the
time of writing)_. The main problem with this crate is not the maintenance overhead of code architecture with
Bevy but with the serenity.

You might think that this should be the easy part, it only looks like that but the 
[`events.rs`](https://github.com/AS1100K/bevy-discord/blob/v0.2.2/src/bot/events.rs) in module `bot` is over
700 lines of code that only covers the events struct which is just the representation of 70+ serenity events
[`handle.rs`](https://github.com/AS1100K/bevy-discord/blob/v0.2.2/src/bot/handle.rs).

Although I used rust macros [`common.rs`](https://github.com/AS1100K/bevy-discord/blob/v0.2.2/src/common.rs) 
in best of my ability to reduce a lot of effort and nonsense maintenance overburden. The module `bot` was 
handled in the best possible manner _(at least best of my abilities)_ for the crate to be efficient and 
performance oriented while reduce maintenance overburden.

You might be thinking that `serenity` API(s) is only going to change when discord is going to change their API(s),
that's correct but for this crate as it's just a wrapper around `serenity`, I want my minimum effort on maintaining
as those events were written manually. _(I believe there are more efficient manner to achieve this, but I ignored
them for now)_.

Also, `webhook` module was providing everything expect a single feature that is provided by serenity `Http`
module but at this moment using `Http` was 
[quite expensive](https://docs.rs/bevy-discord/latest/bevy_discord/bot/struct.DiscordBotRes.html#method.get_http)
in bevy-discord. Also, maintaining `webhook` module is double maintaining as serenity is already doing that. 
Therefore, I will be introducing a new module named `http` _(WIP at time of writing)_ in `bevy-discord` that is 
going to be a wrapper around serenity `Http` but with improved performance in bevy environment.