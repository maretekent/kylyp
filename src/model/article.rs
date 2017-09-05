use model::user::User;
use utils::schema::{article,comment};

#[derive(Clone,Debug,Serialize,Queryable, Associations)]
#[belongs_to(User)]
pub struct Article {
    pub id: i32,
    pub uid: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
}


#[derive(Insertable)]
#[table_name="article"]
pub struct NewArticle<'a> {
    pub uid: i32,
    pub title: &'a str,
    pub content: &'a str,
    pub createtime: &'a str,
}

#[derive(Clone,Debug,Serialize,Queryable,  Associations)]
#[belongs_to(User)]
pub struct Comment {
    pub id: i32,
    pub pid: i32,
    pub uid: i32,
    pub content: String,
    pub createtime: String,
}

#[derive(Insertable)]
#[table_name="comment"]
pub struct NewComment<'a> {
    pub pid: i32,
    pub uid: i32,
    pub content: &'a str,
    pub createtime: &'a str,
}