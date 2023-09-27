#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    let (larger_list, smaller_list) = if _first_list.len() > _second_list.len() {
        (_first_list, _second_list)
    } else {
        (_second_list, _first_list)
    };
    for i in 0..=larger_list.len() - smaller_list.len() {
        if smaller_list == &larger_list[i..i + smaller_list.len()] {
            if larger_list == _first_list {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
    }
    return Comparison::Unequal;
}
