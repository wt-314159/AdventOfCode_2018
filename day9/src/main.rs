use std::collections::VecDeque;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // test();
    part_two(input);
}

#[allow(dead_code)]
fn test() {
    let scores = play_game(9, 25);
    let (win_elf_idx, win_score) = scores
        .iter()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .unwrap();
    println!("Max score: {win_score} scored by elf: {}", win_elf_idx + 1);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let (num_players, remainder) = input.split_once(' ').unwrap();
    let num_players = num_players.parse::<usize>().unwrap();
    let last_marble = remainder
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let scores = play_game(num_players, last_marble);
    let (win_elf_idx, win_score) = scores
        .iter()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .unwrap();
    println!(
        "Winning score is {win_score}, by player no. {}",
        win_elf_idx + 1
    );
}

fn part_two(input: &str) {
    let (num_players, remainder) = input.split_once(' ').unwrap();
    let num_players = num_players.parse::<usize>().unwrap();
    let last_marble = remainder
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap()
        * 100;

    let scores = play_game(num_players, last_marble);
    let (win_elf_idx, win_score) = scores
        .iter()
        .enumerate()
        .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .unwrap();
    println!(
        "Winning score is {win_score}, by player no. {}",
        win_elf_idx + 1
    );
}

fn play_game(num_players: usize, last_marble: usize) -> Box<[usize]> {
    let mut scores = vec![0; num_players].into_boxed_slice();
    let mut played_marbles = VecDeque::with_capacity(last_marble + 1);
    // Do first 3 turns so we don't need to check if 2 > played_marbles.len()
    played_marbles.push_back(0);
    played_marbles.push_back(2);
    played_marbles.push_back(1);
    // print_state(0, &played_marbles);
    played_marbles.rotate_right(1);
    // print_state(0, &played_marbles);

    let mut cur_player = 3;
    for m in 3..=last_marble {
        if m % 23 == 0 {
            scores[cur_player - 1] += m;
            played_marbles.rotate_right(seven_or_wrapped(played_marbles.len()));
            scores[cur_player - 1] += played_marbles.pop_back().unwrap();
            played_marbles.rotate_left(1);
        } else {
            played_marbles.rotate_left(1);
            played_marbles.push_back(m);
        }
        // print_state(cur_player, &played_marbles);
        cur_player += 1;
        if cur_player > num_players {
            cur_player = 1
        }
    }
    scores
}

#[allow(dead_code)]
fn print_state(cur_player: usize, marbles: &VecDeque<usize>) {
    print!("[{cur_player}] - ");
    for m in marbles {
        print!("{m:>2} ");
    }
    println!();
}

fn seven_or_wrapped(vec_len: usize) -> usize {
    if vec_len >= 7 {
        7
    } else {
        let mut wrapped = 7;
        while wrapped > vec_len {
            wrapped -= vec_len;
        }
        wrapped
    }
}

fn wrap_index(mut idx: i32, vec_len: usize) -> usize {
    let vec_len_i32 = vec_len as i32;
    while idx > vec_len_i32 {
        idx -= vec_len_i32;
    }
    while idx < 0 {
        idx += vec_len_i32;
    }
    idx as usize
}
