use actix_web::Result;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Error;

#[get("/")]
pub async fn index() -> Result<maud::Markup> {
    Ok(maud::html! {
        html {
            body {
                h1 { "Waterlevels" }

                p { "Please note that the application is resource-constrained to 25 MB RAM and 100 Landscape elements or 1000 hours of rain!" }
                p { "The application will fail with bigger numbers or if OOM." }

                form action="/make_landscape" method="post" {
                    label { "Elements" }
                    input type="number" name="elements" checked;

                    input type="submit";
                }
            }
        }
    })
}


#[derive(serde::Deserialize)]
pub struct LandscapeSize {
    elements: usize,
}

#[post("/make_landscape")]
pub async fn make_landscape(form: web::Form<LandscapeSize>) -> Result<maud::Markup> {
    if form.elements > 100 {
        return Ok(maud::html! {
            html {
                body {
                    p { "For runtime reasons, this application only supports landscapes up to 100 elements" }
                }
            }
        })
    }

    Ok(maud::html! {
        html {
            body {
                h1 { "Hello World!" }

                p { "Please fill in the landscape values" }
                form action="/calculate" method="get" {
                    @for _n in 0..form.elements {
                        p {
                            input type="number" name="levels[]" checked;
                        }
                    }

                    input type="number" name="hours" checked;
                    label for="hours" { "Hours of Rain" }

                    input type="submit";
                }
            }
        }
    })
}

#[derive(serde::Deserialize)]
pub struct Landscape {
    levels: Vec<usize>,
    hours: usize,
}


// I don't know why actix-web wont work with a POST parameter here,
// but taken from
//
//  https://github.com/actix/actix-web/issues/259#issuecomment-513942790
//
// this seems to work.
//
// This is neither how you do it, nor how I would like to do it, but it
// (Insert some random swearwords here) works.
//
#[get("/calculate")]
pub async fn calculate(req: HttpRequest) -> Result<maud::Markup> {
    let config = serde_qs::Config::new(10, false);
    let ls: Landscape = config.deserialize_str(&req.query_string())
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?;

    let calculated_landscape = crate::backend::landscape::Landscape::new(ls.levels.clone()).rain(ls.hours);
    Ok(maud::html! {
        html {
            body {
                h1 { "Landscape!" }

                p { "Filling in " (ls.hours) " hours" }

                @for value in &ls.levels {
                    p { (value) }
                }

                h2 { "Filled" }

                @for val in calculated_landscape.into_inner() {
                    p { (val.0) " + " (val.1) }
                }
            }
        }
    })
}


