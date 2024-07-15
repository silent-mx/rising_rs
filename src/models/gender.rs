use std::io::Write;

use diesel::{
    deserialize::FromSql,
    pg::Pg,
    serialize::{IsNull, ToSql},
};

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = crate::schema::sql_types::Gender)]
pub enum Gender {
    Male,
    Female,
}

impl ToSql<crate::schema::sql_types::Gender, Pg> for Gender {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            Gender::Male => out.write_all(b"male")?,
            Gender::Female => out.write_all(b"female")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Gender, Pg> for Gender {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"male" => Ok(Gender::Male),
            b"female" => Ok(Gender::Female),
            _ => Err("Invalid gender value".into()),
        }
    }
}
