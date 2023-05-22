use super::errors::{AppError, ErrorType};
use crate::domain::{Card, CardData};
use chrono::NaiveDate;
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use warp::{reject, Filter};

/// Database connection pool creation
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Database connection pool creation
/// # Arguments
/// * `url` - Database connection url
/// # Returns
/// * `PgPool` - The database connection pool, type PgPool
///
pub fn get_pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    Pool::new(manager).expect("PostgreSQL connection pool could not be created")
}

/// Database access for warp routes
/// # Arguments
/// * `pool` - The database connection pool, type PgPool
/// # Returns
/// * `Filter` - The database access filter, type Filter
///
pub fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err.to_string()).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}

/// Type alias for a database connection pool
type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

/// Database connection pool object
pub struct DBAccessManager {
    pub connection: PooledPg,
}

impl DBAccessManager {
    /// New Database connection pool object
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_card(&self, dto: CardData) -> Result<Card, AppError> {
        use super::schema::data;

        diesel::insert_into(data::table)
            .values(&dto)
            .get_result(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while creating card"))
    }

    pub fn get_card_by_id(&self, _id: i64) -> Result<Card, AppError> {
        use crate::schema::data::dsl::*;

        data.filter(id.eq(_id))
            .first(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while retrieving contract"))
    }

    pub fn check_total_duration_on_date(&self, date: NaiveDate) -> Result<i64, AppError> {
        use crate::schema::data::dsl::*;

        let total_duration = data
            .filter(date.eq(date))
            .select(diesel::dsl::sum(duration))
            .first::<Option<i64>>(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while retrieving total duration"))?
            .unwrap_or(0);

        Ok(total_duration)
    }

    pub fn get_all_cards(&self) -> Result<Vec<Card>, AppError> {
        use crate::schema::data::dsl::*;

        data.load(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while retrieving all cards"))
    }

    pub fn edit_card(&self, _id: i64, dto: CardData) -> Result<Card, AppError> {
        use crate::schema::data::dsl::*;

        let updated = diesel::update(data.filter(id.eq(_id)))
            .set((
                title.eq(dto.title),
                description.eq(dto.description),
                date.eq(dto.date),
                priority.eq(dto.priority),
                duration.eq(dto.duration),
                status.eq(dto.status),
            ))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while updating card"))?;

        if updated == 0 {
            return Err(AppError::new("card not found", ErrorType::NotFound));
        }

        self.get_card_by_id(_id)
    }

    pub fn delete_card(&self, _id: i64) -> Result<usize, AppError> {
        use crate::schema::data::dsl::*;

        let deleted = diesel::delete(data.filter(id.eq(_id)))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while deleting card"))?;

        if deleted == 0 {
            return Err(AppError::new("card not found", ErrorType::NotFound));
        }

        Ok(deleted)
    }
}
