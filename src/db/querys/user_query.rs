use rocket_contrib::json::Json;
use crate::db::models::user::{CreateUser, User, AuthUser};
use crate::db::querys::TigumPgConn;
use crate::db::api_response::ApiResponse;
use rocket::http::Status;
use rocket::response::status;
use diesel::result::Error;
use rocket_contrib::databases::diesel;
use diesel::{QueryDsl, RunQueryDsl, insert_into};
use diesel::ExpressionMethods;


pub fn get_user(conn: &diesel::PgConnection, email_hash_val: i64) -> Result<AuthUser, Error> {
    use crate::schema::users::dsl::*;
    users.filter(email_hash.eq(email_hash_val)).get_result::<AuthUser>(conn)
}

pub fn update_password(
    conn: &diesel::PgConnection,
    email_hash_val: i64, 
    password_hash_val: String
) -> ApiResponse {
    use crate::schema::users::dsl::*;
    let res = diesel::update(users.filter(email_hash.eq(email_hash_val))).set(password_hash.eq(password_hash_val)).get_result::<AuthUser>(conn);
    if res.is_ok() {
        ApiResponse {
            json: json!({ "msg": "Updated password" }),
            status: Status::raw(200)
        }
    } else {
        ApiResponse {
            json: json!({ "error": "Failed to update password" }),
            status: Status::raw(500)
        }
    }
}

pub fn create_user(
    conn: &diesel::PgConnection,
    new_user: Json<CreateUser>,
    hashed_password: String,
    hashed_email: u64,
    verify_hash_str: String
) -> Result<AuthUser, Error> {
    use crate::schema::users::dsl::*;
    let hashed_email_i = hashed_email as i64;
    diesel::insert_into(users).values((
        name.eq(&new_user.name), 
        email.eq(&new_user.email),
        password_hash.eq(&new_user.password), 
        email_hash.eq(hashed_email_i), 
        verify_hash.eq(verify_hash_str), 
        verified.eq(false)
    )).get_result::<AuthUser>(conn)
}

pub fn verify_user_with_hash(conn: &diesel::PgConnection, hash: String) -> bool {
    use crate::schema::users::dsl::*;
    let copied_hash = hash.clone();
    if users.filter(verify_hash.eq(hash)).get_result::<AuthUser>(conn).is_ok() {
        set_user_as_verified(conn, copied_hash)
    } else {
        false
    }
}

pub fn set_user_as_verified(conn: &diesel::PgConnection, hash: String) -> bool {
    use crate::schema::users::dsl::*;
    let verified_result = diesel::update(    users.filter(verify_hash.eq(hash))).set(verified.eq(true)).get_result::<AuthUser>(conn);
    verified_result.is_ok()
}
