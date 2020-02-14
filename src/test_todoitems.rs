#[cfg(test)]
mod tests {
    use actix_web::{ test, http } ;
    mod todoitems;

    #[actix_rt::test]
    async fn test_todoitems_get_all_ok() {
        let req = test::TestRequest::default().to_http_request();
        let res = todoitems::get_all(req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}