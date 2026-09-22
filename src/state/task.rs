use {
	crate::state::{
		State,
		host::{ConnectionInfo, Enabled, HostName},
	},
	std::{thread::JoinHandle, time::Duration},
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
		CompletedTask::AddHost(result) => match result {
			Ok(conn_info) => {
				state
					.hosts
					.spawn((HostName(String::from("todo")), conn_info, Enabled));
			}
			Err(err) => match err {
				AddHostError::ConnectionFailed => {
					todo!("show error to user")
				}
			},
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
	AddHost(Result<ConnectionInfo, AddHostError>)
}

//
// AddHost task
//

pub enum AddHostError {
	ConnectionFailed,
}

impl State {
	pub fn task_add_host(&mut self, conn_info: ConnectionInfo) {
		self.spawn(move || {
			std::thread::sleep(Duration::from_secs(5));
			Ok(conn_info)
		})
	}
}
