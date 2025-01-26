use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_rectangle, draw_block};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.8, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 3.0;

pub struct Game {
	snake: Snake,
	food_exists: bool,
	food_x: i32,
	food_y: i32,
	width: i32,
	height: i32,
	game_over: bool,
	wait_time: f64,
}

impl Game {
	pub fn new(width: i32, height: i32) -> Game {
		let snake = Snake::new(2, 2);
		let (food_x, food_y) = Game::random_food_position(&snake, width, height);

		Game {
			snake,
			food_exists: true,
			food_x,
			food_y,
			width,
			height,
			game_over: false,
			wait_time: 0.0,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		self.snake.draw(con, g);

		if self.food_exists {
			draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
		}

		draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
		draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
		draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
		draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

		if self.game_over {
			draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
		}
	}

	pub fn update(&mut self, delta_time: f64) {
		self.wait_time += delta_time;

		if self.game_over {
			if self.wait_time > RESTART_TIME {
				self.restart();
			}

			return;
		}

		if !self.food_exists {
			self.add_food();
		}

		if self.wait_time > MOVING_PERIOD {
			self.update_snake(self.snake.head_direction());
		}
	}

	pub fn restart(&mut self) {
		self.snake = Snake::new(2, 2);
		self.game_over = false;
		self.wait_time = 0.0;
		self.add_food();
	}

	fn random_food_position(snake: &Snake, width: i32, height: i32) -> (i32, i32) {
		let mut rng = thread_rng();
		let mut new_x = rng.gen_range(1..width - 1);
		let mut new_y = rng.gen_range(1..height - 1);

		while snake.overlap_tail(new_x, new_y) {
			new_x = rng.gen_range(1..width - 1);
			new_y = rng.gen_range(1..height - 1);
		}

		(new_x, new_y)
	}

	fn add_food(&mut self) {
		let (new_x, new_y) = Game::random_food_position(&self.snake, self.width, self.height);
		self.food_x = new_x;
		self.food_y = new_y;
		self.food_exists = true;
	}

	pub fn key_pressed(&mut self, key: Key) {
		if self.game_over {
			return;
		}

		let new_dir = match key {
			Key::Up => Some(Direction::Up),
			Key::Down => Some(Direction::Down),
			Key::Left => Some(Direction::Left),
			Key::Right => Some(Direction::Right),
			_ => None,
		};

		if new_dir.unwrap() == self.snake.head_direction().unwrap().opposite() {
			return;
		}

		self.update_snake(new_dir);
	}

	fn update_snake(&mut self, dir: Option<Direction>) {
		if self.check_if_snake_alive(dir) {
			self.snake.move_forward(dir);
			self.check_if_food_eaten();
		} else {
			self.game_over = true;
		}

		self.wait_time = 0.0;
	}

	fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
		let (head_x, head_y) = self.snake.next_head(dir);

		if self.snake.overlap_tail(head_x, head_y) {
			return false;
		}

		head_x > 0 && head_y > 0 && head_x < self.width - 1 && head_y < self.height - 1
	}

	fn check_if_food_eaten(&mut self) {
		let (head_x, head_y) = self.snake.head_position();

		if self.food_exists && self.food_x == head_x && self.food_y == head_y {
			self.food_exists = false;
			self.snake.restore_tail();
		}
	}
}
