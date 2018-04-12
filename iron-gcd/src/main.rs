extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::satus;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwarp();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(satus::Ok);
    response.set_mut(mine!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n"/>
<input type="text" name="n"/>
<button type="submit">Compute GCD</button>
</from>
"#);
    Ok(response);
}
    
