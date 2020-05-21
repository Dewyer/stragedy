use std::error::Error;
use std::fmt;

pub trait LibError
where Self : PartialEq, Self:serde::Serialize,Self:Clone, Self: serde::de::DeserializeOwned
{
	fn get_no_error() -> Self where Self: Sized;
	fn new_other(ss:&str) -> Self where Self: Sized;
	fn get_server_error() -> Self where Self: Sized;
}

#[derive(Debug,Clone,Deserialize,Serialize,PartialEq)]
pub enum ApiError
{
	ZeroModified,
	UpdateFailed,
	Other(String),
	AuthError,
	ServerError,
	NoError
}

#[derive(Debug,Clone,Deserialize,Serialize,PartialEq)]
pub enum AuthError
{
	UserExists,
	BadUsername,
	BadEmail,
	Other(String),
	NoUser,
	WrongPassword,
	ServerError,
	NoError
}

impl Error for ApiError{}
impl Error for AuthError{}

impl LibError for ApiError
{
	fn get_no_error() -> Self
	{
		ApiError::NoError
	}

	fn new_other(ss: &str) -> Self
	{
		ApiError::Other(ss.to_string())
	}

	fn get_server_error() -> Self {
		ApiError::ServerError
	}
}

impl LibError for AuthError
{
	fn get_no_error() -> Self {
		AuthError::NoError
	}

	fn new_other(ss: &str) -> Self {
		AuthError::Other(ss.to_string())
	}

	fn get_server_error() -> Self {
		AuthError::ServerError
	}
}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for AuthError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<Box<dyn Error>> for AuthError
{
	fn from(error: Box<dyn Error>) -> Self
	{
		Self::new_other(error.description())
	}
}

impl From<Box<dyn Error>> for ApiError
{
	fn from(error: Box<dyn Error>) -> Self
	{
		Self::new_other(error.description())
	}
}

impl From<AuthError> for ApiError
{
	fn from(error: AuthError) -> Self
	{
		ApiError::AuthError
	}
}

impl From<ApiError> for AuthError
{
	fn from(error:ApiError) -> Self
	{
		AuthError::ServerError
	}
}
