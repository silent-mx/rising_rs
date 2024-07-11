use super::gender::Gender;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::schema::sys_user)]
pub struct SysUser {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<Gender>,
    pub avatar: Option<String>,
    pub is_admin: bool,
    #[serde(with = "time::serde::iso8601")]
    pub create_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub update_at: OffsetDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sys_user)]
pub struct NewSysUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<Gender>,
    pub avatar: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sys_user)]
pub struct NewAdminSysUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<Gender>,
    pub avatar: Option<String>,
    pub is_admin: bool,
}
