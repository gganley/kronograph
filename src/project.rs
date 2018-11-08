use rocket::{get, post, put, delete};

#[post("/add", format = "application/json")]
pub fn add() {

}

#[put("/edit", format = "application/json")]
pub fn edit() {
    
}

#[delete("/delete", format = "application/json")]
pub fn delete() {
    
}

#[get("/id/project")]
pub fn data() {
    
}
