#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() > _second_list.len() {
        if _second_list.is_empty() {
            return Comparison::Superlist;
        }
        let contains_list_two = contains_list(_first_list, _second_list);
        if contains_list_two {
            return Comparison::Superlist;
        } else {
            return Comparison::Unequal;
        }
    } else if _first_list.len() < _second_list.len() {
       if _first_list.is_empty() {
            return Comparison::Sublist;
        }
        let contains_list_one = contains_list(_second_list, _first_list);

        if contains_list_one {
            return Comparison::Sublist;
        } else {
            return Comparison::Unequal;
        }
    } else {
        if _first_list == _second_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    }
}

pub fn contains_list<T: PartialEq>(list_one: &[T], list_two: &[T]) -> bool {
    let contains = list_one
        .windows(list_two.len())
        .filter(|sl| *sl == list_two)
        .collect::<Vec<&[T]>>();
    !contains.is_empty() 
      
    
}