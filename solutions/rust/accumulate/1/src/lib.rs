pub fn map<F,U,T>(input: Vec<T>, mut f: F) -> Vec<U>  
    where F: FnMut(T) -> U {
    let mut new: Vec<_> = vec![];
    for e in input {
        new.push(f(e))
    }

    new
}