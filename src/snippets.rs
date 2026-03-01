use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Rust,
    Python,
}

impl Language {
    pub fn label(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Rust => "Rust",
            Language::Python => "Python",
        }
    }
}

pub const MENU_OPTIONS: &[Language] = &[Language::English, Language::Rust, Language::Python];

pub fn random_snippet(lang: Language) -> String {
    let pool = match lang {
        Language::English => ENGLISH,
        Language::Rust => RUST_SNIPPETS,
        Language::Python => PYTHON_SNIPPETS,
    };
    let snippet = pool.choose(&mut thread_rng()).unwrap();
    snippet.trim_matches('\n').to_string()
}

const ENGLISH: &[&str] = &[
    "the quick brown fox jumps over the lazy dog near the riverbank",
    "practice makes perfect and consistency is the key to mastery",
    "a journey of a thousand miles begins with a single step forward",
    "the only way to do great work is to love what you do every day",
    "simplicity is the ultimate sophistication in both design and code",
    "every expert was once a beginner who refused to give up on learning",
    "typing fast is not about speed alone but about accuracy and rhythm",
    "the best time to plant a tree was twenty years ago the second best time is now",
    "focus on being productive instead of busy and results will follow naturally",
    "do not wait to strike until the iron is hot but make it hot by striking",
];

const RUST_SNIPPETS: &[&str] = &[
    r#"fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}"#,
    r#"fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}"#,
    r#"use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}"#,
    r#"struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}"#,
    r#"fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}"#,
    r#"use std::fmt;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}"#,
    r#"fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}"#,
    r#"fn flatten(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
}

fn main() {
    let data = vec![vec![1, 2], vec![3, 4], vec![5]];
    println!("{:?}", flatten(data));
}"#,
    r#"trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    fn describe(&self) {
        println!("{} says {}", self.name(), self.sound());
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> &str { "Dog" }
    fn sound(&self) -> &str { "woof" }
}"#,
    r#"fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { result.push(a[i]); i += 1; }
        else { result.push(b[j]); j += 1; }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}"#,
];

const PYTHON_SNIPPETS: &[&str] = &[
    r#"def fibonacci(n):
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(n - 1):
        a, b = b, a + b
    return b"#,
    r#"from collections import Counter

def most_common(words):
    count = Counter(words)
    return count.most_common(3)"#,
    r#"def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1"#,
    r#"class Stack:
    def __init__(self):
        self.items = []

    def push(self, item):
        self.items.append(item)

    def pop(self):
        return self.items.pop() if self.items else None

    def peek(self):
        return self.items[-1] if self.items else None"#,
    r#"def flatten(nested):
    result = []
    for item in nested:
        if isinstance(item, list):
            result.extend(flatten(item))
        else:
            result.append(item)
    return result"#,
    r#"import functools

def memoize(func):
    cache = {}
    @functools.wraps(func)
    def wrapper(*args):
        if args not in cache:
            cache[args] = func(*args)
        return cache[args]
    return wrapper"#,
    r#"def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)"#,
    r#"from dataclasses import dataclass
from typing import List

@dataclass
class Student:
    name: str
    grade: float

def top_students(students: List[Student], n: int) -> List[Student]:
    return sorted(students, key=lambda s: s.grade, reverse=True)[:n]"#,
    r#"def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(n ** 0.5) + 1):
        if n % i == 0:
            return False
    return True

primes = [n for n in range(2, 50) if is_prime(n)]"#,
    r#"def word_frequency(text):
    words = text.lower().split()
    freq = {}
    for word in words:
        freq[word] = freq.get(word, 0) + 1
    return dict(sorted(freq.items(), key=lambda x: x[1], reverse=True))"#,
];
