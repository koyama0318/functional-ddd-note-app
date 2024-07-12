use super::user::SaveUserFn;
use crate::Context;

pub fn save_user_fn(context: &Context) -> impl SaveUserFn {
    move |user| Ok(user)
}
