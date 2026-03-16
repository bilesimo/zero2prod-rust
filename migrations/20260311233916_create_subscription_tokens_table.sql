CREATE TABLE subscription_tokens(
    subscription_token TEXT NOT NULL,
    subscriber_id UUID NOT NULL
        REFERENCES subscriptions(id),
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (subscription_token, subscriber_id)
)