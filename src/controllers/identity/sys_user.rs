use crate::{app_state::AppState, models::SysUser};
use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;

/// Handles the `GET /api/identity/sys_users` route
pub async fn sys_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<SysUser>>, (StatusCode, String)> {
    let conn = state.pg_pool().await.unwrap();
    conn.interact(move |conn| {
        use crate::schema::sys_user::dsl::*;

        let res = sys_user
            .select(crate::models::SysUser::as_select())
            .load(conn)
            .unwrap();
        Ok(Json(res))
    })
    .await
    .unwrap()
}
