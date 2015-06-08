
	pub fn fibonacci() {
		let v = get_even_fibs();
		let sum_num = sum_the_fibs(&v);	
		println!("The sum is: {}", sum_num );
	}

	fn get_even_fibs() -> Vec<i32>{
		let mut v: Vec<i32> = Vec::new();
		let mut x: i32 = 1;
		let mut y: i32 = 1;

		while x < 4000000 {
			let b: i32 = x;
			x = x + y;
			y = b;
			if x % 2 == 0 {
				v.push(x);
			}
		}
		v
	}

	fn sum_the_fibs(v: &Vec<i32>) -> i32 {
		let mut sum_num: i32 = 0;
		for z in v {
			println!("{}", z.to_string() );
			sum_num = sum_num + z;
		}
		sum_num	
	}

