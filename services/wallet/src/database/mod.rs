use db_utils::create_database;
use eve_esi::model::wallet_journal::WalletJournal;
use eve_sso::AccessToken;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use self::model::{BalanceSummary, BountySummary, Character, CharacterBalance, JournalEntry};

pub mod model;

#[derive(Debug)]
pub struct Database {
    db_pool: MySqlPool,
}

impl Database {
    #[tracing::instrument(skip(connection_string))]
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

    #[tracing::instrument(skip(self, access_token))]
    pub async fn update_credentials(
        &self,
        user_id: u64,
        access_token: &AccessToken,
    ) -> anyhow::Result<()> {
        sqlx::query(r#"
            INSERT INTO characters (id, user_id, access_token, access_token_expire_at) VALUES(?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                access_token=VALUES(access_token),
                access_token_expire_at=VALUES(access_token_expire_at)
        "#)
        .bind(access_token.character_id)
        .bind(user_id)
        .bind(&access_token.access_token)
        .bind(access_token.expires_at)
        .execute(&self.db_pool).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_wallet_balance(
        &self,
        character_id: u64,
        balance: i64,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO balance_history (character_id, balance) VALUES (?, ?)
        "#,
        )
        .bind(character_id)
        .bind(balance)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_wallet_journal(
        &self,
        character_id: u64,
        journal_entries: &Vec<WalletJournal>,
    ) -> anyhow::Result<()> {
        let mut tx = self.db_pool.begin().await?;

        for journal in journal_entries {
            sqlx::query(
                r#"
            INSERT INTO journal (
                id,
                character_id,
                date,
                description,
                ref_type,
                reason,
                amount,
                balance,
                context_id,
                context_id_type,
                first_party_id,
                second_party_id, 
                tax,
                tax_receiver_id
            ) VALUES (?, ?, ?, ?, ?, ? ,? ,? ,? ,? ,? ,? ,? ,?)
        "#,
            )
            .bind(journal.id)
            .bind(character_id)
            .bind(journal.date)
            .bind(&journal.description)
            .bind(&journal.ref_type)
            .bind(&journal.reason)
            .bind(journal.amount)
            .bind(journal.balance)
            .bind(journal.context_id)
            .bind(&journal.context_id_type)
            .bind(journal.first_party_id)
            .bind(journal.second_party_id)
            .bind(journal.tax)
            .bind(journal.tax_receiver_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_characters_for_processing(
        &self,
        regular_update_limit: i32,
    ) -> anyhow::Result<Vec<Character>> {
        let characters = sqlx::query_as(
            r#"
            SELECT cha.id, cha.access_token, cha.access_token_expire_at FROM characters cha 
                LEFT JOIN balance_history bh ON cha.id=bh.character_id 
                WHERE bh.character_id IS NULL
            UNION (
                SELECT cha.id, cha.access_token, cha.access_token_expire_at FROM characters cha
                LEFT JOIN (
                    SELECT MAX(date) most_recent_update, character_id FROM balance_history GROUP BY character_id
                ) upd ON upd.character_id=cha.id
                WHERE upd.most_recent_update <= DATE_SUB(NOW(), INTERVAL 1 HOUR)
                ORDER BY upd.most_recent_update ASC
                LIMIT ?
            )
        "#,
        )
        .bind(regular_update_limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(characters)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_balance(
        &self,
        user_id: u64,
        character_id: u64,
    ) -> anyhow::Result<Option<CharacterBalance>> {
        let balance = sqlx::query_as(
            r#"
            SELECT b.* FROM balance_history b
            INNER JOIN characters c ON c.id=b.character_id 
            WHERE c.user_id=? AND b.character_id=?
            ORDER BY date DESC
            LIMIT 1
        "#,
        )
        .bind(user_id)
        .bind(character_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(balance)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_user_balance(&self, user_id: u64) -> anyhow::Result<Vec<CharacterBalance>> {
        let wallets = sqlx::query_as(
            r#"
            SELECT b.date, b.character_id, b.balance FROM characters ch JOIN (
                SELECT bh.* FROM balance_history bh JOIN (
                    SELECT MAX(date) max_date, character_id, balance FROM balance_history GROUP BY character_id
                ) bhm ON bh.date=bhm.max_date AND bh.character_id=bhm.character_id
            ) b ON ch.id=b.character_id
            WHERE ch.user_id=?
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(wallets)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_last_journal_id(&self, character_id: u64) -> anyhow::Result<Option<i64>> {
        let last_journal_id: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT id FROM journal
            WHERE character_id=? 
            ORDER BY id 
            DESC LIMIT 1;
        "#,
        )
        .bind(character_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(last_journal_id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_monthly_balance(
        &self,
        user_id: u64,
    ) -> anyhow::Result<Vec<BalanceSummary>> {
        let balance_summary = sqlx::query_as(r#"
            SELECT date_range.date, db.balance, da.amount FROM (
                SELECT curdate() - INTERVAL (a.date_range + (10 * b.date_range) + (100 * c.date_range) + (1000 * d.date_range) ) DAY as date
                FROM (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as a
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) AS b
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as c
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as d
            ) date_range 
            
            LEFT JOIN (
                SELECT cdb.date, SUM(cdb.balance) balance FROM (
                    SELECT DATE(b.date) date, b.character_id, AVG(b.balance) balance FROM balance_history b
                    INNER JOIN characters c ON c.id=b.character_id
                    WHERE c.user_id=1 GROUP BY c.id, DATE(b.date)
                ) cdb GROUP BY cdb.date
            ) db ON date_range.date=db.date
            LEFT JOIN (
                SELECT cda.date, SUM(cda.amount) amount FROM (
                    SELECT DATE(j.date) date, j.character_id, SUM(j.amount) amount FROM journal j
                    INNER JOIN characters c ON c.id=j.character_id
                    WHERE c.user_id=1 GROUP BY c.id, DATE(j.date)
                ) cda GROUP BY cda.date
            ) da ON date_range.date=da.date
            
            WHERE date_range.date > CURDATE() - INTERVAL 1 MONTH
            GROUP BY date_range.date
            ORDER BY date_range.date ASC
        "#)
        .bind(user_id)
        .bind(user_id)
        .fetch_all(&self.db_pool).await?;

        Ok(balance_summary)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_monthly_bounty_summary(
        &self,
        user_id: u64,
    ) -> anyhow::Result<Vec<BountySummary>> {
        let bounty_summary = sqlx::query_as(r#"
            SELECT date_range.date, ucj.character_id, ucj.sum_bounties, ucj.sum_taxes FROM (
                SELECT curdate() - INTERVAL (a.date_range + (10 * b.date_range) + (100 * c.date_range) + (1000 * d.date_range) ) DAY as date
                FROM (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as a
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) AS b
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as c
                CROSS JOIN (select 0 as date_range UNION all select 1 UNION all select 2 UNION all select 3 UNION all select 4 UNION all select 5 UNION all select 6 UNION all select 7 UNION all select 8 UNION all select 9) as d
            ) date_range LEFT JOIN ( 
                SELECT DATE(j.date) DATE, j.character_id, SUM(j.amount) sum_bounties, SUM(j.tax) sum_taxes
                FROM journal j INNER JOIN characters c
                ON c.id=j.character_id
                WHERE c.user_id=? AND j.ref_type="bounty_prizes" 
                GROUP BY j.character_id, DATE(j.date)
            ) ucj ON date_range.date=ucj.date
            WHERE date_range.date > CURDATE() - INTERVAL 1 MONTH
            ORDER BY date_range.date ASC
        "#)
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(bounty_summary)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_journal_entry_count(&self, character_id: u64) -> anyhow::Result<i64> {
        let entry_count = sqlx::query_scalar("SELECT COUNT(*) FROM journal WHERE character_id=?")
            .bind(character_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(entry_count)
    }

    #[tracing::instrument(skip(self))]
    pub async fn select_journal_entries(
        &self,
        character_id: u64,
        limit: usize,
        offset: usize,
    ) -> anyhow::Result<Vec<JournalEntry>> {
        let entries = sqlx::query_as(
            r#"
            SELECT * FROM journal WHERE character_id=?
            ORDER BY date DESC
            LIMIT ? OFFSET ?
        "#,
        )
        .bind(character_id)
        .bind(limit as u64)
        .bind(offset as u64)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(entries)
    }
}
