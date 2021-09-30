use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use rustc_serialize::json::Json;

const SIZE: usize = 9;
const CELL_SIZE: usize = 3;

struct Sudoku {
	data: Vec<Vec<usize>>,
	
	rows: Vec<Collection>,
	cols: Vec<Collection>,
	cells: Vec<Collection>,
	
	count: i64,
}

impl Sudoku {
	pub fn new(_data: Vec<Vec<Option<usize>>>) -> Sudoku {
		let mut sudoku = Sudoku { data: vec![], rows: vec![], cols: vec![], cells: vec![], count: 0 };
		for row in 0..SIZE {
			let mut tmprow: Vec<usize> = vec![];
			for col in 0..SIZE {
				let data = _data[row][col];
				if data.is_some() {
					tmprow.push(data.unwrap());
				} else {
					tmprow.push(0)
				}
			}
			sudoku.data.push(tmprow);
		};
		
		return sudoku;
	}
}


impl fmt::Display for Sudoku {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "\n ");
		for col in 0..SIZE {
			write!(formatter, "--- ");
			if col % 3 == 2 {
				write!(formatter, " ");
			}
		}
		write!(formatter, "\n");
		
		for row in 0..SIZE {
			write!(formatter, "|");
			for col in 0..SIZE {
				if self.data[row as usize][col as usize] == 0 {
					write!(formatter, "   ");
				} else {
					write!(formatter, " {} ", self.data[row as usize][col as usize]);
				}
				if col % 3 == 2 && col != SIZE - 1 {
					write!(formatter, "|");
				}
				write!(formatter, "|");
			}
			write!(formatter, "\n");
			
			write!(formatter, " ");
			if row % 3 == 2 && row != SIZE - 1 {
				for row in 0..SIZE {
					write!(formatter, "=== ");
					if row % 3 == 2 {
						write!(formatter, " ");
					}
				}
			} else {
				for col in 0..SIZE {
					write!(formatter, "--- ");
					if col % 3 == 2 {
						write!(formatter, " ");
					}
				}
			}
			write!(formatter, "\n");
		}
		return write!(formatter, "count:{} \n", self.count);
	}
}


/**
Rows
Vec<Vec<>>
 */
pub fn read_file_to_json(path: &str) -> Vec<Vec<Option<usize>>> {
	let mut file = File::open(path).unwrap();
	let mut data = String::new();
	
	let read = file.read_to_string(&mut data);
	if read.is_err() {
		
		//println!("Err reading file: {}", read.err().unwrap());
	}
	
	let json = Json::from_str(&data).unwrap().into_array().unwrap();
	
	
	//println!("File: {:?}", json);
	
	let mut list: Vec<Vec<Option<usize>>> = Vec::with_capacity(json.len());
	
	for line in json.iter() {
		let linearr = line.as_array().unwrap();
		let mut tmplist = Vec::with_capacity(linearr.len());
		for val in linearr.iter() {
			tmplist.push(if val.is_null() { None } else { Some(val.as_i64().unwrap() as usize) })
		}
		list.push(tmplist)
	}
	
	return list;
}


pub struct Collection {
	data: Vec<usize>,
}

impl fmt::Debug for Collection {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		return write!(formatter, "data: {:?}", self.data);
	}
}

impl Collection {
	pub fn add_value(&mut self, value: usize) {
		self.data.push(value)
	}
	pub fn remove_value(&mut self, value: usize) {
		self.data.retain(|x| { *x != value })
	}
}

impl Sudoku {
	pub fn get_first_missing(&self, startrow: usize, startcol: usize) -> (usize, usize) {
		if startrow >= SIZE || startcol >= SIZE {
			return (SIZE, SIZE);
		}
		return if self.data[startrow][startcol] == 0 {
			(startrow, startcol)
		} else {
			self.get_first_missing(get_next_row(startrow, startcol), get_next_col(startcol))
		};
	}
	
	pub fn start_solve(&mut self) {
		for row in 0..SIZE {
			let mut rowvec = vec![];
			let mut colvec = vec![];
			for el in 0..SIZE {
				if self.data[row][el] != 0 {
					rowvec.push(self.data[row][el])
				}
			}
			for el in 0..SIZE {
				if self.data[el][row] != 0 {
					colvec.push(self.data[el][row])
				}
			}
			self.rows.push(Collection { data: rowvec });
			self.cols.push(Collection { data: colvec });
		}
		for cell in 0..SIZE {
			let y_offset = (cell / CELL_SIZE) * CELL_SIZE;
			let x_offset = (cell - y_offset) * CELL_SIZE;
			
			let mut cellvec = vec![];
			for row in 0..CELL_SIZE {
				for el in 0..CELL_SIZE {
					if self.data[y_offset + row][x_offset + el] != 0 {
						cellvec.push(self.data[y_offset + row][x_offset + el])
					}
				}
			}
			self.cells.push(Collection { data: cellvec })
		}
		
		self.solve(0, 0);
	}
	
	pub fn solve(&mut self, startrow: usize, startcol: usize) -> bool {
		self.count += 1;
		let (row, col) = self.get_first_missing(startrow, startcol);
		
		if (row, col) == (SIZE, SIZE) {
			
			//println!("finished");
			return true;
		}
		
		
		//println!("cell:{:?} at {}-{}", self.data[row][col], row, col);
		
		let available_values: Vec<usize> = self.get_available(&self.rows[row], &self.cols[col], &self.cells[get_cell(row, col)]);
		
		//println!("available:{:?}", available_values);
		if available_values.is_empty() {
			
			//println!("filled (available_values) empty");
			return false;
		}
		
		for value in available_values {
			
			//println!("testing {} on {}-{}: {}", value, row, col, self.data[row][col]);
			
			self.rows[row].add_value(value);
			self.cols[col].add_value(value);
			self.cells[get_cell(row, col)].add_value(value);
			
			self.data[row][col] = value;
			
			//println!("set value");
			
			
			//println!("{}", self);
			
			if self.solve(row, col) {
				return true;
			}
			
			self.rows[row].remove_value(value);
			self.cols[col].remove_value(value);
			self.cells[get_cell(row, col)].remove_value(value);
			
			self.data[row][col] = 0;
		}
		
		
		//println!();
		return false;
	}
	
	pub fn get_available(&self, row: &Collection, col: &Collection, cell: &Collection) -> Vec<usize> {
		let mut available: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
		for ell in row.data.iter() {
			if *ell != 0 {
				available.retain(|x| x != ell);
			}
		}
		for ell in col.data.iter() {
			if *ell != 0 {
				available.retain(|x| x != ell);
			}
		}
		for ell in cell.data.iter() {
			if *ell != 0 {
				available.retain(|x| x != ell);
			}
		}
		return available;
	}
}

fn get_next_row(row: usize, col: usize) -> usize {
	return row + (col + 1) / SIZE;
}

fn get_next_col(col: usize) -> usize {
	return (col + 1) % SIZE;
}

fn get_cell(row: usize, col: usize) -> usize {
	(row / CELL_SIZE) * CELL_SIZE + col / CELL_SIZE
}

fn main() {
	//println!("\n");
	let sudokudata = read_file_to_json("resources/sudoku2.json");
	
	//println!("Json: {:?} \n", sudokudata);
	
	let mut sudoku = Sudoku::new(sudokudata);
	
	let start = Instant::now();
	sudoku.start_solve();
	let elapsed = start.elapsed();
	
	println!("{}",sudoku);
	println!("in: {:?}", elapsed);
}