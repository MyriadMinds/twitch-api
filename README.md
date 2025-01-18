# twitch-api
Library for communicating with the Twitch API and Eventsub in an ergonomic fashion

## Examples

### Establishing a basic connection with the API and Eventsub

```Rust
use twitch-api::{Twitch, SubscriptionType, Conditions};

let client_id = std::env::var("CLIENT_ID").expect("No client ID provided!");
let access_token = std::env::var("ACCESS_TOKEN").expect("No access token provided!");
let broadcaster_id = std::env::var("BROADCASTER_ID").expect("No broadcaster ID provided!");
let token_user_id = broadcaster_id.clone();

let api = Twitch::new(client_id, access_token);
let eventsub = Twitch::connect_eventsub().expect("Failed to connect to Eventsub!");

let desired_subscriptions = vec![
  SubscriptionType::Follow,
  SubscriptionType::Raid,
  SubscriptionType::ChatMessage
];

let conditions = Conditions::new(broadcaster_id, token_user_id);

for subscription in desired_subscriptions {
  let subscription = subscription.build_subscription(&eventsub.session, &conditions);
  api.create_eventsub_subscription(subscription).expect("Failed to add subscription to eventsub session!");
}

for event in eventsub.iter() {
  // event handling code here
}

```

If you possess a refresh token, there is a helper function for acquiring an access token from it

```Rust
let (access_token, new_refresh_token) = Twitch::authenticate(&client_id, &client_secret, &refresh_token).expect("Failed to authenticate with twitch!");
```

## Features

- Fully synchronous communication
- Automatic handling of reconnect messages for EventSub

### Supported API Endpoints
- Get access token from refresh token
- Create EventSub connection
- Create EventSub Subscription

### Supported EventSub Subscription
- Follow
- AdBreakBegin
- ChatClear
- ChatClearUserMessages
- ChatMessage
- ChatMessageDelete
- Subscribe
- SubscriptionGift
- SubscriptionMessage
- Cheer
- Raid
- PointsCustomRewardRedemptionAdd
- PollBegin
- PollProgress
- PollEnd
- PredictionBegin
- PredictionProgress
- PredictionLock
- PredictionEnd
- CharityDonation
- HypeTrainBegin
- HypeTrainProgress
- HypeTrainEnd
- ShoutoutReceived

## Planned

- More EventSub subcriptions
- More API Endpoints
- Helpers for quick setup and teardown of http servers for acquiring tokens
- Better signaling of errors happening with the EventSub connection