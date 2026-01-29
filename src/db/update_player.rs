use eyre::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn change_player_ship(pool: &Pool<Postgres>, ship_id: Uuid, token: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET ship_id = $1
            WHERE token = $2
            AND (
                SELECT status
                FROM game_players
                RIGHT OUTER JOIN games ON games.id = game_players.game_id
                RIGHT OUTER JOIN players ON players.id = game_players.player_id
                WHERE players.token = $2
            ) = 'lobby';
        "#,
        ship_id,
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}
