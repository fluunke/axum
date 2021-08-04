use tower::timeout::TimeoutLayer;

use super::*;

#[tokio::test]
async fn basic() {
    let one = route("/foo", get(|| async {})).route("/bar", get(|| async {}));
    let two = route("/baz", get(|| async {}));
    let app = one.or(two);

    let addr = run_in_background(app).await;

    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://{}/foo", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get(format!("http://{}/bar", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get(format!("http://{}/baz", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .get(format!("http://{}/qux", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn layer_and_handle_error() {
    let one = route("/foo", get(|| async {}));
    let two = route("/time-out", get(futures::future::pending::<()>))
        .layer(TimeoutLayer::new(Duration::from_millis(10)))
        .handle_error(|_| Ok(StatusCode::REQUEST_TIMEOUT));
    let app = one.or(two);

    let addr = run_in_background(app).await;

    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://{}/time-out", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::REQUEST_TIMEOUT);
}

// TODO(david): layered without handle error

#[tokio::test]
async fn nesting() {
    let one = route("/foo", get(|| async {}));
    let two = nest("/bar", route("/baz", get(|| async {})));
    let app = one.or(two);

    let addr = run_in_background(app).await;

    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://{}/bar/baz", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn boxed() {
    let one = route("/foo", get(|| async {})).boxed();
    let two = route("/bar", get(|| async {})).boxed();
    let app = one.or(two);

    let addr = run_in_background(app).await;

    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://{}/bar", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

// TODO(david): boxed

#[tokio::test]
async fn many_ors() {
    let app = route("/r1", get(|| async {}))
        .or(route("/r2", get(|| async {})))
        .or(route("/r3", get(|| async {})))
        .or(route("/r4", get(|| async {})))
        .or(route("/r5", get(|| async {})))
        .or(route("/r6", get(|| async {})))
        .or(route("/r7", get(|| async {})));

    let addr = run_in_background(app).await;

    let client = reqwest::Client::new();

    for n in 1..=7 {
        let res = client
            .get(format!("http://{}/r{}", addr, n))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    let res = client
        .get(format!("http://{}/r8", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

// TODO(david): service
