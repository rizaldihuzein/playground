// searches index from slice
// for the first index occurence of n_min
pub fn binary_search_left_most(slice:&Vec<u64>, n_min:u64)->usize{
    let mut start:usize = 0;
    let mut end = slice.len() - 1;
    let mut pos = (start+end)/2;
    if pos == 0{
        return pos;
    }
    loop{
        if slice[pos]<n_min{
            start = pos;
        }else{
            end = pos;
        }
        pos = (start+end)/2;
        if end-start == 1{
            break;
        }
    }
    
    pos+1
}