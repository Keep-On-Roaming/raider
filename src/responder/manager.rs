// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::collections::HashMap;

use rocket;
use rocket::config::{Config, Environment};
use rocket_contrib::templates::Template;

use super::{catchers, routes};
use storage::db;

use APP_CONF;

pub fn run() {
    // Build Rocket configuration
    let mut config = Config::build(Environment::Production)
        .address(APP_CONF.server.inet.ip().to_string())
        .port(APP_CONF.server.inet.port())
        .workers(APP_CONF.server.workers)
        .secret_key(APP_CONF.server.secret_key.as_str())
        .finalize()
        .unwrap();

    // Append extra options
    let mut extras = HashMap::new();

    extras.insert(
        "template_dir".to_string(),
        APP_CONF
            .assets
            .path
            .join("./templates")
            .to_str()
            .unwrap()
            .into(),
    );

    config.set_extras(extras);

    // Build and run Rocket instance
    rocket::custom(config)
        .mount(
            "/",
            routes![
                routes::get_index,
                routes::get_robots,
                routes::get_initiate_base,
                routes::get_initiate_login,
                routes::get_initiate_signup,
                routes::get_initiate_recover,
                routes::get_initiate_logout,
                routes::get_dashboard_base,
                routes::get_dashboard_welcome,
                routes::get_dashboard_trackers,
                routes::get_dashboard_payouts,
                routes::get_dashboard_payouts_partial_payouts,
                routes::get_dashboard_account,
                routes::get_assets_fonts,
                routes::get_assets_images,
                routes::get_assets_stylesheets,
                routes::get_assets_javascripts,
                routes::post_initiate_login_form_login,
                routes::post_initiate_signup_form_signup,
                routes::post_initiate_recover_form_recover,
                routes::post_dashboard_trackers_form_create,
                routes::post_dashboard_trackers_form_remove,
                routes::post_dashboard_payouts_form_request,
                routes::post_dashboard_account_form_account,
                routes::post_dashboard_account_form_payout,
                routes::post_track_payment,
                routes::post_track_signup,
            ],
        )
        .register(catchers![catchers::forbidden, catchers::gone,])
        .attach(Template::fairing())
        .manage(db::pool())
        .launch();
}
