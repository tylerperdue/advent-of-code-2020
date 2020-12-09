use std::collections::HashMap;

fn main() {
    let nums: Vec<i64> = include_str!("xmas-cipher.txt")
    	.lines()
    	.map(|l| l.parse::<i64>().unwrap())
    	.map(|n| n)
    	.collect();

    let hashed_preambles: Vec<HashMap<i64, bool>> = nums.iter()
    	.enumerate()
    	.filter(|&(i, _)| i >= 25)
    	.map(|(i, _)| (get_preamble(&nums, i)))
    	.map(|p| (hash(&p)))
    	.collect();

    println!("{:#?}", part_1(&nums, &hashed_preambles))
    
}

fn get_preamble(v: &Vec<i64>, i: usize) -> Vec<i64> {
	v[i-25..i].to_vec()
}

fn hash(v: &Vec<i64>) -> HashMap<i64, bool> {
	let mut m: HashMap<i64, bool> = HashMap::new();

	for i in v {
		m.insert(*i, true);
	}

	m
}

fn part_1(nums: &Vec<i64>, v: &Vec<HashMap<i64, bool>>) -> i64 {
	for i in 25..nums.len() {
		if !contains_a_sum(&v[i-25], &nums[i]) {
			return nums[i]
		}
	}

	0
} 

fn contains_a_sum(m: &HashMap<i64, bool>, n: &i64) -> bool {
	for k in m.keys() {
		let d = n - k;

		if m.contains_key(&d) {
			return true
		}
	}

	false
}
