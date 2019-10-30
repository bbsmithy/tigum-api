/////////////////////////////////
//// ARTICLE SNIPPETS ROUTES ////
/////////////////////////////////

#[delete("/videos/<id>")]
fn delete_single_video(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<String> {
    delete_video(&conn, id)
}

#[put("/videos/<id>", format = "application/json", data = "<video>")]
fn update_single_video(conn: TigumPgConn, id: i32, video: Json<NewVideo>) -> Json<Video> {
    update_video(&conn, id, video)
}

#[post("/videos/create", format = "application/json", data = "<video>")]
pub fn create_single_video(conn: TigumPgConn, video: Json<NewVideo>) -> Json<Id> {
    println!("{:?}", video);
    create_video(&conn, video)
}

#[get("/videos/<id>")]
pub fn single_video(conn: TigumPgConn, id: i32, _auth_user: User) -> Json<Video> {
    get_video(&conn, id)
}

#[post("/videos", format = "application/json", data = "<ids>")]
fn videos(conn: TigumPgConn, ids: Json<Ids>) -> Json<Vec<Video>> {
    println!("{:?}", ids);
    get_videos(&conn, ids)
}