use {
	crate::state::{
		Alert, State,
		host::{ConnectionInfo, Enabled, HostName},
	},
	ssh2::Session,
	std::{net::TcpStream, thread::JoinHandle, time::Duration},
};

/// Tasks are things the app spawns to run in the background, since they could
/// take a while to finish.
pub trait Task {
	/// Check if the task finished running.
	fn is_completed(&self) -> bool;
	/// Get whatever the task returned and store it in a [`CompletedTask`].
	fn complete(self: Box<Self>) -> CompletedTask;
}

impl State {
	/// Check on all running tasks. If any have completed, update state
	/// accordingly.
	pub fn check_tasks(&mut self) {
		let mut idx = 0;
		while idx < self.tasks.len() {
			if self.tasks[idx].is_completed() {
				let completed = self.tasks.remove(idx).complete();
				handle_task(self, completed);
			} else {
				idx += 1
			}
		}
	}

	pub fn spawn<F, R>(&mut self, f: F)
	where
		F: FnOnce() -> R + Send + 'static,
		R: Send + 'static,
		JoinHandle<R>: Task,
	{
		let task = std::thread::spawn(f);
		self.tasks.push(Box::new(task))
	}
}

fn handle_task(state: &mut State, task: CompletedTask) {
	match task {
		CompletedTask::AddHost((conn_info, result)) => match result {
			Ok(conn_info) => {
				state
					.hosts
					.spawn((HostName(String::from("todo")), conn_info, Enabled));
			}
			Err(err) => {
				let alert_body = match err {
					AddHostError::ConnectionFailed(err) => {
						format!("Failed to connect to host: {err}")
					}
					AddHostError::SshError(err) => {
						format!("SSH had an error: {err}")
					}
					AddHostError::AuthenticationFailed => "Invalid credentials".to_string(),
				};

				state.alerts.push(Alert {
					title: format!("Failed to add host {}", conn_info.ip),
					content: alert_body,
				})
			}
		},
	}
}

macro_rules! tasks {
	($($name:ident($returns:ty))*) => {
		/// Data that different tasks return when they finish.
		pub enum CompletedTask {
			$($name($returns)),*
		}

		$(impl Task for JoinHandle<$returns> {
			fn is_completed(&self) -> bool {
				self.is_finished()
			}
			fn complete(self: Box<Self>) -> CompletedTask {
				CompletedTask::$name((*self).join().unwrap())
			}
		})*
	};
}
tasks! {
	AddHost((ConnectionInfo, Result<ConnectionInfo, AddHostError>))
}

//
// AddHost task
//

pub enum AddHostError {
	ConnectionFailed(std::io::Error),
	SshError(ssh2::Error),
	AuthenticationFailed,
}

struct PasswordKip(String);
impl ssh2::KeyboardInteractivePrompt for PasswordKip {
	fn prompt<'a>(
		&mut self,
		_username: &str,
		_instructions: &str,
		_prompts: &[ssh2::Prompt<'a>],
	) -> Vec<String> {
		vec![self.0.clone()]
	}
}

impl State {
	pub fn task_add_host(&mut self, conn_info: ConnectionInfo, username: String, password: String) {
		let conn_info2 = conn_info.clone();
		let conn = move || {
			let conn = TcpStream::connect((conn_info.ip, conn_info.port))
				.map_err(AddHostError::ConnectionFailed)?;
			let mut session = Session::new().map_err(AddHostError::SshError)?;

			session.set_tcp_stream(conn);
			session.handshake().map_err(AddHostError::SshError)?;

			let password_auth = session.userauth_password(&username, &password);
			if password_auth.is_err() {
				let keyboard_interactive_auth =
					session.userauth_keyboard_interactive(&username, &mut PasswordKip(password));
				if keyboard_interactive_auth.is_err() {
					return Err(AddHostError::AuthenticationFailed);
				}
			}

			Ok(conn_info)
		};

		self.spawn(move || (conn_info2, conn()))
	}
}
