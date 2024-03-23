use db_utils::create_database;
use eve_esi::model::character::Character as EsiCharacter;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

#[derive(Debug)]
pub struct Database {
    db_pool: MySqlPool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Character {
    pub character_id: u64,
    pub name: String,
    pub alliance_id: Option<i32>,
    pub corporation_id: i32,
    pub is_main: bool,
    pub requires_authorization: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CharacterId {
    id: u64,
}

impl Database {
    #[tracing::instrument]
    pub async fn connect(connection_string: &str) -> anyhow::Result<Database> {
        create_database(connection_string, env!("CARGO_PKG_NAME")).await?;

        let connection_string = format!("{}/{}", connection_string, env!("CARGO_PKG_NAME"));

        let db_pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&connection_string)
            .await?;

        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(Database { db_pool })
    }

    #[tracing::instrument(skip(self))]
    pub async fn insert_character(&self, user_id: u64, character_id: u64) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO characters (id, user_id, is_main) SELECT ?, ?, (COUNT(*) = 0) FROM characters WHERE user_id=?")
            .bind(character_id)
            .bind(user_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn insert_character_data(
        &self,
        character_id: u64,
        character: &EsiCharacter,
    ) -> anyhow::Result<()> {
        sqlx::query(r#"
            INSERT INTO character_data 
                (character_id, alliance_id, bloodline_id, corporation_id, description, faction_id, gender, name, race_id, security_status, title) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                alliance_id=VALUES(alliance_id), corporation_id=VALUES(corporation_id), description=VALUES(description), security_status=VALUES(security_status), title=VALUES(title), updated_at=CURRENT_TIMESTAMP()
            "#)
            .bind(character_id)
            .bind(character.alliance_id)
            .bind(character.bloodline_id)
            .bind(character.corporation_id)
            .bind(&character.description)
            .bind(character.faction_id)
            .bind(&character.gender)
            .bind(&character.name)
            .bind(character.race_id)
            .bind(character.security_status)
            .bind(&character.title)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_main(&self, user_id: u64) -> anyhow::Result<Option<Character>> {
        let character: Option<Character> = sqlx::query_as("SELECT cd.*, cc.is_main, cc.requires_authorization FROM character_data cd LEFT JOIN characters cc ON cc.id = cd.character_id WHERE cc.user_id = ? AND cc.is_main")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(character)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_characters(&self, user_id: u64) -> anyhow::Result<Vec<Character>> {
        let characters: Vec<Character> = sqlx::query_as("SELECT cd.*, cc.is_main, cc.requires_authorization FROM character_data cd LEFT JOIN characters cc ON cc.id = cd.character_id WHERE cc.user_id = ?")
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(characters)
    }

    pub async fn select_character(
        &self,
        user_id: u64,
        character_id: u64,
    ) -> anyhow::Result<Character> {
        let character = sqlx::query_as("SELECT cd.*, cc.is_main, cc.requires_authorization FROM character_data cd LEFT JOIN characters cc ON cc.id = cd.character_id WHERE cc.user_id = ? AND cc.id = ?")
            .bind(user_id)
            .bind(character_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(character)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_characters_for_data_processing(
        &self,
        regular_update_limit: i32,
    ) -> anyhow::Result<Vec<u64>> {
        let character_ids: Vec<CharacterId> = sqlx::query_as(
            r#"
            SELECT cha.id FROM characters cha 
                LEFT JOIN character_data cda ON cha.id=cda.character_id 
                WHERE cda.character_id IS NULL
            UNION (SELECT cha.id FROM characters cha 
                LEFT JOIN character_data cda ON cha.id=cda.character_id 
                WHERE cda.updated_at <= DATE_SUB(NOW(), INTERVAL 2 HOUR)
                LIMIT ?)
        "#,
        )
        .bind(regular_update_limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(character_ids.iter().map(|c| c.id).collect())
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_character_token_valid(
        &self,
        character_id: u64,
        is_valid: bool,
    ) -> anyhow::Result<()> {
        sqlx::query("UPDATE `characters` SET requires_authorization=? WHERE id=?")
            .bind(false == is_valid)
            .bind(character_id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}
