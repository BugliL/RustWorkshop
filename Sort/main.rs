pub fn sort(v: Vec<u8>) -> Vec<u8> {
    // step0: use the sort function provided by the standard library
    // step1: implement bubble sort https://en.wikipedia.org/wiki/Bubble_sort
    v
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_sort() {
        assert_eq!(sort(vec![]), vec![]);
        assert_eq!(sort(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(sort(vec![4, 3, 2, 1]), vec![1, 2, 3, 4]);
        assert_eq!(sort(vec![3, 1, 2, 5, 7]), vec![1, 2, 3, 5, 7]);
    }

    // implement the sort_ref function to make the 
    // following test pass. Refactor sort in order to
    // to use the new sort_ref
    // #[test]
    // fn test_sort_ref() {
    //     let mut v = vec![8, 42, 12];
        
    //     assert_eq!(sort_ref(&mut v), &vec![8, 12, 42]);
    //     assert_eq!(v, vec![8, 12, 42]);
    // }
}

