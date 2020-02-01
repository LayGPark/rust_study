

fn main() {
    println!("Hello, world!");

    let shoes = vec![
        shoe_size::Shoe { size: 10, style: String::from("스니커즈")},
        shoe_size::Shoe { size: 13, style: String::from("샌달")},
        shoe_size::Shoe { size: 10, style: String::from("부츠")},

    ];

    let in_my_size = shoe_size::shoes_in_my_size(shoes, 10);

    println!("{:?}", in_my_size);

    /*
    let in_my_size = shoe_size::shoes_in_my_size(shoes, 10);
   |                                                  ----- value moved here
...
16 |     println!("{:?}", shoes);
   |                      ^^^^^ value borrowed here after move
    */
    //이미 소유권이 넘어감
    //println!("{:?}", shoes);


    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1;
    /*
    28 |     let v2: Vec<_> = v1;
   |                      -- value moved here
29 |     println!("v1 {:?}", v1);
   |                         ^^ value borrowed here after move
   */
    //println!("v1 {:?}", v1);

    println!("v2 {:?}", v2);




    //=========================

}
