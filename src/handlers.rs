    pub async fn base() -> &'static str {
        "Handling GET request for /"
    }

    pub async fn event_get() -> &'static str {
        "Handling GET request for /api/event"
    }

    pub async fn event_post() -> &'static str {
        "Handling POST request for /api/event"
    }

    pub async fn event_put() -> &'static str {
        "Handling PUT request for /api/event"
    }

    pub async fn event_delete() -> &'static str {
        "Handling DELETE request for /api/event"
    }