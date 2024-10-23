use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use tide::{http::mime::JSON, Request, Response, Result, StatusCode};

use crate::{models::{claims::Claims, user::{LoginInput, LoginResponse, UserInput}}, repositories::user_repo::UserRepo};

pub struct UserHandler {
  repo: UserRepo
}

impl UserHandler {
  pub fn new(repo: UserRepo) -> UserHandler {
    UserHandler {
      repo
    }
  }

	pub async fn read(&self, req: Request<()>) -> Result {
		let _ = req;

		match self.repo.read().await {
			Ok(users) => {
				// send_json_response!(users, StatusCode::Ok)
				let body = serde_json::to_string(&users)?;

				let res = Response::builder(StatusCode::Ok)
					.content_type(JSON)
					.body(body)
					.build();
				Ok(res)
			},
			Err(err) => {
				let res = Response::builder(StatusCode::InternalServerError)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				Ok(res)
			},
		}
	}

	pub async fn create(&self, mut req: Request<()>) -> Result<String> {
		let mut user: UserInput = req.body_json().await?;

		match hash(user.password, DEFAULT_COST) {
			Ok(hashed_pass) => user.password = hashed_pass,
			Err(err) => return Ok(format!("{}", err)),
		}
		
		match self.repo.create(user).await {
			Ok(id) => Ok(format!("{}", id)),
			Err(err) => Ok(format!("{}", err))
		}
	}

	pub async fn update(&self, mut req: Request<()>) -> Result {
		let user: UserInput = match req.body_json().await {
			Ok(res) => res,
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		let id = match req.param("id") {
			Ok(val) => val.parse::<i32>()?,
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		match self.repo.update(id, user).await {
			Ok(result) => {
				let res = Response::builder(StatusCode::Ok)
					.content_type(JSON)
					.body(result)
					.build();
				Ok(res)
			},
			Err(err) => {
				let res = Response::builder(StatusCode::InternalServerError)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		}
	}

	pub async fn delete(&self, req: Request<()>) -> Result {
		let id = match req.param("id") {
			Ok(val) => val.parse::<i32>()?,
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		match self.repo.delete(id).await {
			Ok(result) => {
				let res = Response::builder(StatusCode::Ok)
					.content_type(JSON)
					.body(result)
					.build();
				Ok(res)
			},
			Err(err) => {
				let res = Response::builder(StatusCode::InternalServerError)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		}
	}

	pub async fn get_by_id(&self, req: Request<()>) -> Result {
		let id = match req.param("id") {
			Ok(val) => val.parse::<i32>().unwrap_or_default(),
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		match self.repo.get_by_id(id).await {
			Ok(user) => {
				let body = serde_json::to_string(&user).unwrap_or_default();
				let res = Response::builder(StatusCode::Ok)
					.content_type(JSON)
					.body(body)
					.build();
				Ok(res)
			},
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		}
	}

	pub async fn login(&self, mut req: Request<()>) -> Result {
		let login_input: LoginInput = match req.body_json().await {
			Ok(body) => body,
			Err(err) => {
				let res = Response::builder(StatusCode::BadRequest)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		let result = match self.repo.get_by_username(login_input.username).await {
			Ok(res) => res,
			Err(err) => {
				let res = Response::builder(StatusCode::InternalServerError)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		};

		let mut user = match result {
			Some(res) => res,
			None => {
				let res = Response::builder(StatusCode::Unauthorized)
					.content_type(JSON)
					.body("username or password invalid")
					.build();
				return Ok(res)
			}
		};

		let is_correct = verify(login_input.password, &user.password).unwrap_or(false);
		if !is_correct {
			let res = Response::builder(StatusCode::Unauthorized)
				.content_type(JSON)
				.body("username or password invalid")
				.build();
			return Ok(res)
		} 

		user.password = String::from("");
		let claims = Claims {
			user: user.clone(),
			exp: 10000000000
		};

		match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
			Ok(token) => {
				let login_response = LoginResponse {
					token,
					id: user.id,
					username: user.username,
					email: user.email,
				};
				
				let body = serde_json::to_string(&login_response).unwrap_or_default();

				let res = Response::builder(StatusCode::Ok)
					.content_type(JSON)
					.body(body)
					.build();
				Ok(res)
			},
			Err(err) => {
				let res = Response::builder(StatusCode::InternalServerError)
					.content_type(JSON)
					.body(err.to_string())
					.build();
				return Ok(res)
			}
		}
	}
}

