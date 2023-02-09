use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use jwt_simple::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_new_user(
    conn: &mut SqliteConnection,
    usrname: &str,
    pssword: &str,
    ) -> Result<crate::models::User, DbError> {

    use crate::schema::users::dsl::*;

    let new_user = crate::models::User {
        id: Uuid::new_v4().to_string(),
        username: usrname.to_owned(),
        password: hash(pssword.to_owned(), DEFAULT_COST).unwrap(),
        session_id: String::from(""),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn login_user(
    conn: &mut SqliteConnection,
    usrname: &str,
    pssword: &str,
    ) -> Result<String, DbError> {
    
    use crate::schema::users::dsl::*;

    let user_result = users
        .filter(username.eq(usrname))
        .first::<crate::models::User>(conn);

    let user = match user_result {
        Ok(ok) => ok,
        Err(_) => return Ok(String::from("")),
    };

    let valid_result = verify(pssword, &user.password);
    let valid = match valid_result {
        Ok(ok) => ok,
        Err(_) => false,
    };

    if valid {
        let token = create_token().unwrap();
        if let Some(jti) = validate_token_and_get_jti(&token) {
            diesel::update(users.filter(username.eq(usrname)))
                .set(session_id.eq(jti))
                .execute(conn)?;
            Ok(token)
        } else {
            Ok(String::from(""))
        }
        // diesel::update(users.filter(username.eq(usrname)))
        //     .set(session_id.eq(jti))
        //     .execute(conn)?;
        // Ok(token)
    }
    else {
        Ok(String::from(""))
    }
}

fn create_token() -> Result<String, jwt_simple::Error> {
    let secret_key = std::env::var("SECRET_KEY").expect("No SECRET_KEY provided");

    let key = HS256Key::from_bytes(secret_key.as_bytes());

    let claims = Claims::create(Duration::from_days(7))
       .with_jwt_id(Uuid::new_v4().to_string());

    key.authenticate(claims)
    
}

pub fn validate_token(token: &str, conn: &mut SqliteConnection) -> bool {
    let token_result = validate_token_and_get_jti(token);

    if let Some(jti) = token_result {
        use crate::schema::users::dsl::*;
        let database_result = users
            .filter(session_id.eq(jti))
            .first::<crate::models::User>(conn);
        if let Ok(_user) = database_result {
            return true;
        }

    }
    false
}

fn validate_token_and_get_jti(token: &str) -> Option<String> {
    let secret_key = std::env::var("SECRET_KEY").expect("No SECRET_KEY provided");
    let key = HS256Key::from_bytes(secret_key.as_bytes());

    let claims_result = key.verify_token::<NoCustomClaims>(&token, None);

    if let Ok(claims) = claims_result {
        claims.jwt_id
    } else {
        None
    }
}

