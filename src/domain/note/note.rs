// use crate::domain::user::UserId;

// #[derive(Debug)]
// pub struct NoteId {
//     id: String,
// }

// impl NoteId {
//     pub fn id(&self) -> String {
//         self.id.clone()
//     }

//     pub fn new(id: String) -> Self {
//         NoteId { id: id }
//     }
// }

// #[derive(Debug)]
// pub struct UnvalidatedNote {
//     id: NoteId,
// }

// impl UnvalidatedNote {
//     pub fn new(id: NoteId) -> Self {
//         UnvalidatedNote { id: id }
//     }
// }

// #[derive(Debug)]
// pub struct ValidatedNote {
//     id: NoteId,
// }

// impl ValidatedNote {
//     pub fn new(id: NoteId) -> Self {
//         ValidatedNote { id: id }
//     }
// }

// #[derive(Debug)]
// pub struct CreatedNote {
//     id: NoteId,
// }

// impl CreatedNote {
//     pub fn new(id: NoteId) -> Self {
//         CreatedNote { id: id }
//     }
// }

// #[derive(Debug)]
// pub enum NoteError {
//     NotFound,
// }

// // Action
// pub fn validate(note: UnvalidatedNote) -> Result<ValidatedNote, NoteError> {
//     Ok(ValidatedNote::new(NoteId::new("".to_string())))
// }

// pub fn create(note: ValidatedNote) -> Result<CreatedNote, NoteError> {
//     Ok(CreatedNote::new(NoteId::new("".to_string())))
// }

// // Workflow
// pub type Workflow = dyn Fn(UnvalidatedNote) -> Result<CreatedNote, NoteError>;

// type GetUserById = fn(UserId) -> Result<User, UserError>;

// async fn get_note_by_id(ctx: Arc<Mutex<Connection>>) -> impl Fn(UserId) -> Result<User, UserError> {
//     |id| Ok(User::new(id, "name".to_string()))
// }

// async fn get_user_by_ida(context: Connection, user_id: UserId) -> Result<User, UserError> {
//     task::spawn_blocking(move || {
//         // let conn = context.lock().unwrap();
//         let mut stmt = context
//             .prepare("SELECT id, name FROM user WHERE id = ?1")
//             .unwrap();

//         let mut user_iter = stmt
//             .query_map(params![user_id.id()], |row| {
//                 Ok(User::new(
//                     UserId::new(row.get(0).unwrap_or("".to_string())),
//                     row.get(1).unwrap_or("".to_string()),
//                 ))
//             })
//             .unwrap();

//         user_iter
//             .next()
//             .transpose()
//             .unwrap()
//             .ok_or(UserError::NotFound)
//     })
//     .await
//     .unwrap()
// }
