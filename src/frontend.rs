use actix_web::Result;
use actix_web::get;
use actix_web::post;
use actix_web::web;

#[get("/")]
pub async fn index() -> Result<maud::Markup> {
    Ok(maud::html! {
        html {
            body {
                h1 { "Hello World!" }

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
                form action="/calculate" method="post" {
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

#[post("/calculate")]
pub async fn calculate(form: web::Form<Landscape>) -> Result<maud::Markup> {
    Ok(maud::html! {
        html {
            body {
                h1 { "Landscape!" }

                p { "Filling in " (form.hours) " hours" }

                @for value in &form.levels {
                    p { (value) }
                }
            }
        }
    })
}

