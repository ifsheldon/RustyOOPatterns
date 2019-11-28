//!
//! The Iterator Pattern is fairly easy
//!
//! For more info and details, see https://doc.rust-lang.org/book/ch13-02-iterators.html
//!

pub fn easy_test()
{
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    for val in v1_iter
        {
            println!("{}", val);
        }
}

pub fn stream_like_usage()
{
    let v1 = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    for val in v2
        {
            println!("{}", val);
        }
}

#[cfg(test)]
pub mod iter_test
{
    use super::*;

    #[test]
    fn test_run()
    {
        easy_test();
        println!("-------------------------");
        stream_like_usage();
    }
}