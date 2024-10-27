#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]

mod common;
use std::collections::HashSet;

use common::get_time;
use proconio::input_interactive;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

const DIJ8: [(usize, usize); 8] = [
    (!0, 0),
    (!0, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, !0),
    (0, !0),
    (!0, !0),
];

fn main() {
    get_time();
    let input = read_input();
    let answer = climbing(&input);
    answer.output(&input);
}

fn climbing(input: &Input) -> Answer {
    let mut rng = ChaCha20Rng::seed_from_u64(0);
    let dt = 0.001;
    let limit = 4.0;
    let mut answer = solve_init(input, &mut rng, get_time() + dt);
    let mut not_updated_cnt = 0;

    while get_time() < limit {
        let is_updated = if rng.gen_bool(0.9) {
            neighbor1(&mut rng, input, &mut answer, dt)
        } else {
            neighbor2(&mut rng, input, &mut answer, dt)
        };
        if !is_updated {
            not_updated_cnt += 1;
        } else {
            not_updated_cnt = 0;
        }
        if not_updated_cnt >= 100 {
            not_updated_cnt = 0;
            let output_ans = answer.output_ans.clone();
            let output_score = answer.output_score;
            answer = solve_init(input, &mut rng, get_time() + dt);
            answer.output_ans = output_ans;
            answer.output_score = output_score;
        }
    }
    answer
}

fn neighbor1(rng: &mut ChaCha20Rng, input: &Input, answer: &mut Answer, dt: f64) -> bool {
    answer.score = 0;
    answer.ans.clear();
    let len = answer.best_ans.len();
    let start_idx = rng.gen_range(0..len - 1);
    let end_idx = (start_idx + rng.gen_range(1..=20.min(len / 2))).min(len - 1);
    let range = start_idx..=end_idx;
    let start = answer.best_ans[start_idx];
    let end = answer.best_ans[end_idx];
    let mut used = vec![false; input.N * input.N];

    let before_ans = answer.best_ans.clone();
    let before_socre = answer.best_score;
    for i in 0..start_idx + 1 {
        let pos = before_ans[i];
        let r = pos / input.N;
        let c = pos % input.N;
        answer.score += input.mults[r][c] * (i + 1);
        answer.ans.push(pos);
    }
    answer.best_score = answer.score;

    for (i, a) in before_ans.iter().enumerate() {
        if !range.contains(&i) {
            used[*a] = true;
        }
    }
    used[start] = true;
    let until = get_time() + dt;
    dfs_connect(start, end, input, &mut used, answer, &until);

    answer.ans = answer.best_ans.clone();
    answer.score = answer.best_score;

    if *answer.ans.last().unwrap() != end {
        answer.best_score = before_socre;
        answer.best_ans = before_ans;
        return false;
    }

    let mut num = answer.ans.len() + 1;
    for i in end_idx + 1..len {
        let pos = before_ans[i];
        let r = pos / input.N;
        let c = pos % input.N;
        answer.score += input.mults[r][c] * num;
        num += 1;
        answer.ans.push(pos);
    }
    if before_socre >= answer.score {
        answer.best_score = before_socre;
        answer.best_ans = before_ans;
        false
    } else {
        answer.best_score = answer.score;
        answer.best_ans = answer.ans.clone();
        answer.update_output();
        true
    }
}

fn neighbor2(rng: &mut ChaCha20Rng, input: &Input, answer: &mut Answer, dt: f64) -> bool {
    answer.score = 0;
    answer.ans.clear();
    let len = answer.best_ans.len();
    let start_idx = rng.gen_range(0..len / 2);
    let end_idx = len - rng.gen_range(0..len / 2) - 2;
    if start_idx >= end_idx {
        return false;
    }
    let range = start_idx..=end_idx;
    let start = answer.best_ans[end_idx + 1];
    let mut used = vec![false; input.N * input.N];

    let before_ans = answer.best_ans.clone();
    let before_socre = answer.best_score;
    let mut num = 1;
    for i in range {
        let pos = before_ans[i];
        let r = pos / input.N;
        let c = pos % input.N;
        answer.score += input.mults[r][c] * num;
        answer.ans.push(pos);
        num += 1;
        used[pos] = true;
    }
    answer.best_score = answer.score;

    used[start] = true;
    answer.ans.push(start);
    let until = get_time() + dt;
    dfs(start, input, &mut used, answer, until);

    answer.ans = answer.best_ans.clone();
    answer.score = answer.best_score;

    if before_socre >= answer.score {
        answer.best_score = before_socre;
        answer.best_ans = before_ans;
        false
    } else {
        answer.best_score = answer.score;
        answer.best_ans = answer.ans.clone();
        answer.update_output();
        true
    }
}

fn solve_init(input: &Input, rng: &mut ChaCha20Rng, until: f64) -> Answer {
    let mut used = vec![false; input.N * input.N];
    let sr = rng.gen_range(0..input.N);
    let sc = rng.gen_range(0..input.N);
    let st = sr * input.N + sc;
    used[st] = true;
    let init_score = input.mults[sr][sc];
    let mut answer = Answer::new(vec![st], init_score);
    dfs(st, input, &mut used, &mut answer, until);
    answer.update_output();
    // eprintln!("init score: {}", answer.output_score);
    answer
}

fn dfs(pos: usize, input: &Input, used: &mut Vec<bool>, answer: &mut Answer, until: f64) {
    if get_time() > until {
        return;
    }
    answer.update();
    let num = answer.ans.len() + 1;
    for &next in &input.G[pos] {
        if used[next] {
            continue;
        }
        used[next] = true;
        let nr = next / input.N;
        let nc = next % input.N;
        let add = num * input.mults[nr][nc];
        answer.score += add;
        answer.ans.push(next);
        dfs(next, input, used, answer, until);
        answer.score -= add;
        answer.ans.pop();
        used[next] = false;
    }
}

fn dfs_connect(
    pos: usize,
    end: usize,
    input: &Input,
    used: &mut Vec<bool>,
    answer: &mut Answer,
    until: &f64,
) {
    if get_time() > *until {
        return;
    }
    if pos == end {
        answer.update();
    }
    let num = answer.ans.len() + 1;
    for &next in &input.G[pos] {
        if used[next] {
            continue;
        }
        used[next] = true;
        let nr = next / input.N;
        let nc = next % input.N;
        let add = num * input.mults[nr][nc];
        answer.score += add;
        answer.ans.push(next);
        dfs_connect(next, end, input, used, answer, until);
        answer.score -= add;
        answer.ans.pop();
        used[next] = false;
    }
}

#[derive(Debug)]
struct Answer {
    ans: Vec<usize>,
    best_ans: Vec<usize>,
    output_ans: Vec<usize>,
    score: usize,
    best_score: usize,
    output_score: usize,
}

impl Answer {
    fn new(init_ans: Vec<usize>, init_score: usize) -> Self {
        Self {
            ans: init_ans.clone(),
            best_ans: init_ans.clone(),
            output_ans: init_ans,
            score: init_score,
            best_score: init_score,
            output_score: init_score,
        }
    }
    fn update(&mut self) -> bool {
        if self.score > self.best_score {
            self.best_score = self.score;
            self.best_ans = self.ans.clone();
            true
        } else {
            false
        }
    }
    fn update_output(&mut self) -> bool {
        if self.best_score > self.output_score {
            self.output_score = self.best_score;
            self.output_ans = self.best_ans.clone();
            true
        } else {
            false
        }
    }
    fn output(&self, input: &Input) {
        println!("{}", self.output_ans.len());
        for a in self.output_ans.iter() {
            let r = a / input.N;
            let c = a % input.N;
            println!("{} {}", r, c);
        }
        eprintln!("final score: {}", self.output_score);
    }
}

struct Input {
    N: usize,
    arrows: Vec<Vec<usize>>,
    mults: Vec<Vec<usize>>,
    G: Vec<Vec<usize>>,
    reverse_G: Vec<Vec<usize>>,
}

fn read_input() -> Input {
    input_interactive! {
        N: usize,
    }
    // #[cfg(feature = "build")]
    // {
    //     eprintln!("{}", N);
    // }
    let mut arrows = vec![vec![0; N]; N];
    let mut mults = vec![vec![0; N]; N];
    for i in 0..N {
        for j in 0..N {
            input_interactive! {
                arrow: usize,
                mult: usize,
            }
            // #[cfg(feature = "build")]
            // {
            //     eprintln!("{} {}", arrow, mult);
            // }
            arrows[i][j] = arrow;
            mults[i][j] = mult;
        }
    }
    let mut G = vec![vec![]; N * N];
    let mut reverse_G = vec![vec![]; N * N];

    for i in 0..N {
        // eprintln!("{:?}", mults[i]);
        for j in 0..N {
            let dir = arrows[i][j];
            let (dr, dc) = DIJ8[dir];
            let u = i * N + j;
            let mut r = i + dr;
            let mut c = j + dc;
            while r < N && c < N {
                let v = r * N + c;
                G[u].push(v);
                reverse_G[v].push(u);
                r += dr;
                c += dc;
            }
        }
    }

    Input {
        N,
        arrows,
        mults,
        G,
        reverse_G,
    }
}
