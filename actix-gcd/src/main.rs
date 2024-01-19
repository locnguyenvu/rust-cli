use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


#[derive(Deserialize)]
struct GCDInfo {
    n: u64,
    m: u64,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000").expect("error binding server to address")
        .run().await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title> GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="number" name="n" />
                <input type="number" name="m" />
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

async fn post_gcd(form: web::Form<GCDInfo>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }
    let result = gcd(form.n, form.m);
    println!("{:#?}", form.n);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(["Result:", &result.to_string()].join(" "))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
