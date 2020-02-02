#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}


/// shoe_size 와 동일한 신발들을 반환한다.
///
/// # Examples
/// 
/// ```
/// #[derive(PartialEq, Debug)]
/// pub struct Shoe {
/// pub size: u32,
/// pub style: String,
/// }
/// 
/// let shoes = vec![
/// shoe_size::Shoe { size: 10, style: String::from("스니커즈")},
/// shoe_size::Shoe { size: 13, style: String::from("샌달")},
/// shoe_size::Shoe { size: 10, style: String::from("부츠")},
/// ];
/// 
/// assert_eq!(
/// shoe_size::shoes_in_my_size(shoes, 10),
/// vec![
///     shoe_size::Shoe { size: 10, style: String::from("스니커즈") },
///     shoe_size::Shoe { size: 10, style: String::from("부츠")}
/// ]
/// )
/// ```
pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("스니커즈")},
        Shoe { size: 13, style: String::from("샌달")},
        Shoe { size: 10, style: String::from("부츠")},
    ];
    
    let in_my_size = shoes_in_my_size(shoes, 10);
    
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("스니커즈") },
            Shoe { size: 10, style: String::from("부츠")}
        ]
    );
}


pub struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
    
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    
}

#[test]
fn using_other_iterator_trait_methods() {
    let vec :Vec<u32>= Counter::new().zip(Counter::new().skip(1).skip(1))
    .map(|(a,b)| a*b).filter(|x|x%3==0).collect();

    println!("{:?}",vec);

    let sum: u32 = vec.iter().sum();

    assert_eq!(18,sum);
}

#[test]
fn calling_skip() {
    let mut counter = Counter::new().skip(1).skip(1);

    assert_eq!(counter.next(), Some(3));
    
}