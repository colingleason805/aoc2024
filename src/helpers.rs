
pub

fn bin_search_ins(list: &mut Vec<i32>, val: i32) {
    if list.is_empty() {
        list.push(val);
        return;
    }
    // if list.len() == 1 {
    //     if list[0] <= val {
    //         list.push(val);
    //     } else {
    //         list.insert(0, val);
    //     }
    //     return;
    // }

    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        if low == high {
            // found the index to insert at
            if val <= list[low] {
                list.insert(low, val);
            } else {
                //don't go out of bounds if its the last element
                if low == list.len() {
                    list.push(val);
                } else {
                    list.insert(low + 1, val);
                }
            }
            return;
        }
        let mid = (high + low) / 2;

        // found the same num. insert here
        if val == list[mid] {
            list.insert(mid, val);
            return;
        }

        if val > list[mid] {
            low = mid + 1;
        } else {
            if mid == 0 {
                high = 0;
            } else {
                high = mid - 1;
            }
        }
    }
}
