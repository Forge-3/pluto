use ic_cdk::{init, post_upgrade, query, update};
use ic_pluto::{
    http::{HttpServe, RawHttpRequest, RawHttpResponse},
    http_serve,
    router::Router,
};
use std::cell::RefCell;
use std::rc::Rc;
use ic_http_certification::HttpCertificationTree;
use crate::controller;

thread_local! {
    static ROUTER: RefCell<Router>  = RefCell::new(controller::setup());
    static CERT_TREE: Rc<RefCell<HttpCertificationTree>> = Rc::new(RefCell::new(HttpCertificationTree::default()));
}

#[init]
async fn init() {
    let cert_tree = CERT_TREE.with(|c| c.clone());
    let router = ROUTER.with(|r| r.borrow().clone());
    router.certify_get_responses(cert_tree).await;
}

// System functions
#[post_upgrade]
async fn post_upgrade() {
    let router = controller::setup();
    let cert_tree = CERT_TREE.with(|c| c.clone());
    router.certify_get_responses(cert_tree.clone()).await;
    ROUTER.with(|r| *r.borrow_mut() = router);
}

// Http interface
#[query]
async fn http_request(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}

#[update]
async fn http_request_update(req: RawHttpRequest) -> RawHttpResponse {
    bootstrap(http_serve!(), req).await
}

async fn bootstrap(mut app: HttpServe, req: RawHttpRequest) -> RawHttpResponse {
    let router = ROUTER.with(|r| r.borrow().clone());
    let cert_tree = CERT_TREE.with(|c| c.clone());
    app.set_router(router);
    app.set_certification_tree(cert_tree);
    app.serve_with_cert(req).await
}
